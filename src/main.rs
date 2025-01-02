use chrono::Local;
use clap::Parser;
use config::{Config, File};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use directories::ProjectDirs;
use humansize::{format_size, BINARY};
use nvml_wrapper::{enum_wrappers::device::TemperatureSensor, Nvml};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::PathBuf;
use std::time::Duration;
use sysinfo::{CpuExt, NetworkExt, System, SystemExt};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    symbols,
    text::{Span, Spans},
    widgets::{Block, Borders, Chart, Dataset, GraphType, Paragraph, Wrap},
    Terminal,
};

mod hardware;
use crate::hardware::SystemInfo;

/// System resource monitor
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Disable GPU monitoring
    #[arg(long)]
    no_gpu: bool,

    /// Disable network monitoring
    #[arg(long)]
    no_network: bool,

    /// Update interval in milliseconds
    #[arg(long, default_value_t = 50)]
    interval: u64,

    /// Number of data points in graphs
    #[arg(long, default_value_t = 100)]
    history: usize,

    /// Save current settings as default
    #[arg(long)]
    save_config: bool,

    /// Reset settings to default
    #[arg(long)]
    reset_config: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct AppConfig {
    no_gpu: bool,
    no_network: bool,
    interval: u64,
    history: usize,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            no_gpu: false,
            no_network: false,
            interval: 50,
            history: 100,
        }
    }
}

impl AppConfig {
    fn load() -> Self {
        if let Some(config_path) = get_config_path() {
            if config_path.exists() {
                if let Ok(config) = Config::builder()
                    .add_source(File::with_name(config_path.to_str().unwrap()))
                    .build()
                {
                    return config.try_deserialize().unwrap_or_default();
                }
            }
        }
        Self::default()
    }

    fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(config_path) = get_config_path() {
            // Criar diretório de configuração se não existir
            if let Some(parent) = config_path.parent() {
                fs::create_dir_all(parent)?;
            }

            // Serializar e salvar configurações
            let toml = toml::to_string_pretty(self)?;
            fs::write(config_path, toml)?;
        }
        Ok(())
    }

    fn update_from_args(&mut self, args: &Args) {
        if args.no_gpu {
            self.no_gpu = true;
        }
        if args.no_network {
            self.no_network = true;
        }
        if args.interval != 50 {
            self.interval = args.interval;
        }
        if args.history != 100 {
            self.history = args.history;
        }
    }
}

fn get_config_path() -> Option<PathBuf> {
    ProjectDirs::from("com", "monitor", "system-monitor")
        .map(|proj_dirs| proj_dirs.config_dir().join("config.toml"))
}

#[derive(Clone)]
enum ChartKind {
    Cpu,
    Memory,
    Gpu,
    Swap,
}

#[derive(Clone)]
struct Graph {
    graph_type: ChartKind,
    data: Vec<(f64, f64)>,
    title: String,
    color: Color,
}

impl Graph {
    fn new(graph_type: ChartKind) -> Self {
        let (title, color) = match graph_type {
            ChartKind::Cpu => ("CPU Usage", Color::Cyan),
            ChartKind::Memory => ("Memory Usage", Color::Yellow),
            ChartKind::Gpu => ("GPU Usage", Color::Green),
            ChartKind::Swap => ("SWAP Usage", Color::Magenta),
        };
        Self {
            graph_type,
            data: vec![(0.0, 0.0)],
            title: title.to_string(),
            color,
        }
    }

    fn update(&mut self, data: &SystemData) {
        let value = match self.graph_type {
            ChartKind::Cpu => data.cpu_current,
            ChartKind::Memory => data.mem_current,
            ChartKind::Gpu => data.gpu_current,
            ChartKind::Swap => {
                if data.swap_total > 0 {
                    (data.swap_used as f64 / data.swap_total as f64) * 100.0
                } else {
                    0.0
                }
            }
        };
        self.data.push((data.counter, value));
        if self.data.len() > data.config.history {
            self.data.remove(0);
        }
    }
}

#[allow(dead_code)]
struct SystemData {
    cpu_data: Vec<(f64, f64)>,
    memory_data: Vec<(f64, f64)>,
    gpu_data: Vec<(f64, f64)>,
    counter: f64,
    cpu_current: f64,
    mem_current: f64,
    gpu_current: f64,
    gpu_memory: f64,
    gpu_temp: f64,
    total_memory: u64,
    used_memory: u64,
    available_memory: u64,
    swap_total: u64,
    swap_used: u64,
    rx_bytes: u64,
    tx_bytes: u64,
    rx_bytes_total: u64,
    tx_bytes_total: u64,
    networks: Vec<String>,
    config: AppConfig,
    system_info: SystemInfo,
    graphs: Vec<Graph>,
}

impl SystemData {
    fn new(config: AppConfig) -> Result<SystemData, Box<dyn std::error::Error>> {
        let mut graphs = vec![Graph::new(ChartKind::Cpu)];

        // Adicionar gráficos baseados na configuração
        if !config.no_gpu {
            graphs.push(Graph::new(ChartKind::Gpu));
        }
        graphs.push(Graph::new(ChartKind::Memory));
        graphs.push(Graph::new(ChartKind::Swap));

        let system_info = SystemInfo::new()?;

        Ok(SystemData {
            cpu_data: vec![(0.0, 0.0)],
            memory_data: vec![(0.0, 0.0)],
            gpu_data: vec![(0.0, 0.0)],
            counter: 1.0,
            cpu_current: 0.0,
            mem_current: 0.0,
            gpu_current: 0.0,
            gpu_memory: 0.0,
            gpu_temp: 0.0,
            total_memory: 0,
            used_memory: 0,
            available_memory: 0,
            swap_total: 0,
            swap_used: 0,
            rx_bytes: 0,
            tx_bytes: 0,
            rx_bytes_total: 0,
            tx_bytes_total: 0,
            networks: Vec::new(),
            config,
            system_info,
            graphs,
        })
    }

    fn update(
        &mut self,
        sys: &mut System,
        nvml: &Option<Nvml>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        sys.refresh_memory();
        sys.refresh_cpu();

        // CPU usage
        self.cpu_current = sys.global_cpu_info().cpu_usage() as f64;
        self.cpu_data.push((self.counter, self.cpu_current));
        if self.cpu_data.len() > 100 {
            self.cpu_data.remove(0);
        }

        // Atualização detalhada da memória
        self.total_memory = sys.total_memory();
        self.used_memory = sys.used_memory();
        self.available_memory = sys.available_memory();
        self.swap_total = sys.total_swap();
        self.swap_used = sys.used_swap();

        // Memória para o gráfico (mantendo o comportamento suave)
        let target = (self.used_memory as f64 / self.total_memory as f64) * 100.0;
        self.mem_current = self.mem_current * 0.7 + target * 0.3; // Suavização
        self.memory_data.push((self.counter, self.mem_current));
        if self.memory_data.len() > 100 {
            self.memory_data.remove(0);
        }

        // GPU update com verificação
        if !self.config.no_gpu {
            if let Some(nvml) = nvml {
                if let Ok(device) = nvml.device_by_index(0) {
                    self.gpu_current = device.utilization_rates()?.gpu as f64;
                    self.gpu_data.push((self.counter, self.gpu_current));
                    if self.gpu_data.len() > self.config.history {
                        self.gpu_data.remove(0);
                    }

                    let memory_info = device.memory_info()?;
                    self.gpu_memory = (memory_info.used as f64 / memory_info.total as f64) * 100.0;
                    self.gpu_temp = device.temperature(TemperatureSensor::Gpu)? as f64;
                }
            }
        }

        // Network update com verificação
        if !self.config.no_network {
            sys.refresh_networks();
            let networks = sys.networks();

            let mut new_rx = 0;
            let mut new_tx = 0;

            self.networks.clear();
            for (interface_name, data) in networks {
                new_rx += data.received();
                new_tx += data.transmitted();

                self.networks.push(format!(
                    "{}: ↓{}/s ↑{}/s",
                    interface_name,
                    format_size(data.received(), BINARY),
                    format_size(data.transmitted(), BINARY)
                ));
            }

            self.rx_bytes = new_rx.saturating_sub(self.rx_bytes_total);
            self.tx_bytes = new_tx.saturating_sub(self.tx_bytes_total);
            self.rx_bytes_total = new_rx;
            self.tx_bytes_total = new_tx;
        }

        // Criar uma struct temporária para passar os dados
        let update_data = SystemData {
            counter: self.counter,
            cpu_current: self.cpu_current,
            mem_current: self.mem_current,
            gpu_current: self.gpu_current,
            swap_total: self.swap_total,
            swap_used: self.swap_used,
            config: self.config.clone(),
            cpu_data: vec![],
            memory_data: vec![],
            gpu_data: vec![],
            gpu_memory: 0.0,
            gpu_temp: 0.0,
            total_memory: 0,
            used_memory: 0,
            available_memory: 0,
            rx_bytes: 0,
            tx_bytes: 0,
            rx_bytes_total: 0,
            tx_bytes_total: 0,
            networks: vec![],
            system_info: self.system_info.clone(),
            graphs: vec![],
        };

        for graph in &mut self.graphs {
            graph.update(&update_data);
        }

        self.counter += 1.0;
        Ok(())
    }
}

fn draw_chart<'a>(graph: &'a Graph, counter: f64) -> Chart<'a> {
    let current_value = graph.data.last().map(|&(_, v)| v).unwrap_or(0.0);

    let dataset = Dataset::default()
        .name(&graph.title)
        .marker(symbols::Marker::Braille)
        .graph_type(GraphType::Line)
        .style(Style::default().fg(graph.color.clone()))
        .data(&graph.data);

    Chart::new(vec![dataset])
        .block(
            Block::default()
                .title(Span::styled(
                    format!("{} ({:.1}%)", graph.title, current_value),
                    Style::default()
                        .fg(graph.color.clone())
                        .add_modifier(Modifier::BOLD),
                ))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(graph.color.clone())),
        )
        .x_axis(
            tui::widgets::Axis::default()
                .style(Style::default().fg(Color::Gray))
                .bounds([counter - 100.0, counter]),
        )
        .y_axis(
            tui::widgets::Axis::default()
                .style(Style::default().fg(Color::Gray))
                .bounds([0.0, 100.0]),
        )
}

fn draw_stats(data: &SystemData) -> Paragraph {
    let time = Local::now().format("%H:%M:%S").to_string();
    let mut text = vec![
        // Cabeçalho com OS colorido
        Spans::from(vec![Span::styled(
            data.system_info.get_ascii_art(),
            Style::default()
                .fg(data.system_info.get_os_color())
                .add_modifier(Modifier::BOLD),
        )]),
        Spans::from(vec![
            Span::styled("System Status ", Style::default().fg(Color::White)),
            Span::styled(time, Style::default().fg(Color::Cyan)),
        ]),
        Spans::from(""),
        // CPU Info
        Spans::from(vec![Span::styled(
            "CPU",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )]),
        Spans::from(vec![
            Span::raw("├─ Model:  "),
            Span::styled(
                &data.system_info.cpu_model,
                Style::default().fg(Color::Cyan),
            ),
        ]),
        Spans::from(vec![
            Span::raw("├─ Usage:  "),
            Span::styled(
                format!("{:>5.1}%", data.cpu_current),
                Style::default().fg(Color::Cyan),
            ),
        ]),
        Spans::from(vec![
            Span::raw("└─ Cores:  "),
            Span::styled(
                format!(
                    "{} ({}T)",
                    data.system_info.cpu_cores, data.system_info.cpu_threads
                ),
                Style::default().fg(Color::Cyan),
            ),
        ]),
        // Memory Info
        Spans::from(""),
        Spans::from(vec![Span::styled(
            "Memory",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )]),
        Spans::from(vec![
            Span::raw("├─ Usage:     "),
            Span::styled(
                format!("{:>5.1}%", data.mem_current),
                Style::default().fg(Color::Yellow),
            ),
        ]),
        Spans::from(vec![
            Span::raw("├─ Total:     "),
            Span::styled(
                format_size(data.total_memory, BINARY),
                Style::default().fg(Color::Yellow),
            ),
        ]),
        Spans::from(vec![
            Span::raw("├─ Used:      "),
            Span::styled(
                format_size(data.used_memory, BINARY),
                Style::default().fg(Color::Yellow),
            ),
        ]),
        Spans::from(vec![
            Span::raw("└─ Available: "),
            Span::styled(
                format_size(data.available_memory, BINARY),
                Style::default().fg(Color::Green),
            ),
        ]),
        // SWAP Info
        Spans::from(""),
        Spans::from(vec![Span::styled(
            "SWAP",
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
        )]),
        Spans::from(vec![
            Span::raw("├─ Usage: "),
            Span::styled(
                format!(
                    "{:>5.1}%",
                    if data.swap_total > 0 {
                        (data.swap_used as f64 / data.swap_total as f64) * 100.0
                    } else {
                        0.0
                    }
                ),
                Style::default().fg(Color::Magenta),
            ),
        ]),
        Spans::from(vec![
            Span::raw("├─ Total: "),
            Span::styled(
                format_size(data.swap_total, BINARY),
                Style::default().fg(Color::Magenta),
            ),
        ]),
        Spans::from(vec![
            Span::raw("└─ Used:  "),
            Span::styled(
                format_size(data.swap_used, BINARY),
                Style::default().fg(Color::Magenta),
            ),
        ]),
    ];

    // GPU Info (condicional)
    if !data.config.no_gpu {
        text.extend_from_slice(&[
            Spans::from(""),
            Spans::from(vec![Span::styled(
                "GPU",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            )]),
            Spans::from(vec![
                Span::raw("├─ Model:       "),
                Span::styled(
                    &data.system_info.gpu_model,
                    Style::default().fg(Color::Green),
                ),
            ]),
            Spans::from(vec![
                Span::raw("├─ Usage:       "),
                Span::styled(
                    format!("{:>5.1}%", data.gpu_current),
                    Style::default().fg(Color::Green),
                ),
            ]),
            Spans::from(vec![
                Span::raw("├─ Memory:      "),
                Span::styled(
                    format!("{:>5.1}%", data.gpu_memory),
                    Style::default().fg(Color::Green),
                ),
            ]),
            Spans::from(vec![
                Span::raw("└─ Temperature: "),
                Span::styled(
                    format!("{:>5.1}°C", data.gpu_temp),
                    Style::default().fg(Color::Green),
                ),
            ]),
        ]);
    }

    // Network Info (condicional)
    if !data.config.no_network {
        text.extend_from_slice(&[
            Spans::from(""),
            Spans::from(vec![Span::styled(
                "Network",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            )]),
            Spans::from(vec![
                Span::raw("├─ Download: "),
                Span::styled(
                    format!("{}/s", format_size(data.rx_bytes, BINARY)),
                    Style::default().fg(Color::Blue),
                ),
            ]),
            Spans::from(vec![
                Span::raw("└─ Upload:   "),
                Span::styled(
                    format!("{}/s", format_size(data.tx_bytes, BINARY)),
                    Style::default().fg(Color::Blue),
                ),
            ]),
        ]);

        // Interfaces de rede
        for (i, network_info) in data.networks.iter().enumerate() {
            let is_last = i == data.networks.len() - 1;
            text.push(Spans::from(vec![
                Span::raw(if is_last {
                    "    └─ "
                } else {
                    "    ├─ "
                }),
                Span::styled(network_info, Style::default().fg(Color::Blue)),
            ]));
        }
    }

    Paragraph::new(text)
        .block(
            Block::default()
                .title("Information")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::White)),
        )
        .wrap(Wrap { trim: true })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let mut config = AppConfig::load();

    // Lidar com reset de configuração
    if args.reset_config {
        config = AppConfig::default();
        config.save()?;
        println!("Configuration reset to defaults.");
        return Ok(());
    }

    // Atualizar configuração com argumentos da linha de comando
    config.update_from_args(&args);

    // Salvar configuração se solicitado
    if args.save_config {
        config.save()?;
        println!("Configuration saved as default.");
        return Ok(());
    }

    // Initialize NVML conditionally - skip on macOS
    let nvml = if !config.no_gpu && !cfg!(target_os = "macos") {
        match Nvml::init() {
            Ok(nvml) => Some(nvml),
            Err(_) => None,
        }
    } else {
        None
    };

    // Terminal setup
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app and system info
    let mut data = SystemData::new(config)?;
    let mut sys = System::new_all();

    let mut last_update = std::time::Instant::now();

    loop {
        // Só atualiza os dados se o intervalo configurado passou
        if last_update.elapsed() >= Duration::from_millis(data.config.interval) {
            if let Err(e) = data.update(&mut sys, &nvml) {
                eprintln!("Error updating data: {}", e);
            }
            last_update = std::time::Instant::now();
        }

        terminal.draw(|f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
                .split(size);

            // Criar layout para os gráficos
            let n_graphs = data.graphs.len();
            let constraints: Vec<Constraint> =
                vec![Constraint::Percentage(100 / n_graphs as u16); n_graphs];

            let charts = Layout::default()
                .direction(Direction::Vertical)
                .constraints(constraints)
                .split(chunks[0]);

            // Renderizar todos os gráficos
            for (i, graph) in data.graphs.iter().enumerate() {
                f.render_widget(draw_chart(graph, data.counter), charts[i]);
            }

            // Render stats
            f.render_widget(draw_stats(&data), chunks[1]);
        })?;

        // Polling de eventos com timeout curto
        if event::poll(Duration::from_millis(10))? {
            match event::read()? {
                Event::Key(key) => match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('c') if key.modifiers.contains(event::KeyModifiers::CONTROL) => {
                        break
                    }
                    _ => {}
                },
                Event::Mouse(_) => {} // Ignorar eventos do mouse
                _ => {}
            }
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
