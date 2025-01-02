use std::error::Error;
use sysinfo::{CpuExt, System, SystemExt};
use tui::style::Color;

#[cfg(target_os = "linux")]
use std::fs::read_to_string;

#[cfg(target_os = "windows")]
use serde::Deserialize;

#[derive(Clone)]
pub enum GpuType {
    Nvidia,
    Amd,
    Intel,
}

#[derive(Clone)]
pub struct SystemInfo {
    pub cpu_model: String,
    pub cpu_cores: usize,
    pub cpu_threads: usize,
    pub gpu_model: String,
    pub os_name: String,
    pub os_version: String,
}

impl SystemInfo {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let mut sys = System::new_all();
        sys.refresh_all();

        let cpu = sys.global_cpu_info();

        // Detectar GPU
        let (_, gpu_model) = detect_gpu()?;

        let os_name = if cfg!(target_os = "linux") {
            "Linux".to_string()
        } else if cfg!(target_os = "macos") {
            "macOS".to_string()
        } else if cfg!(target_os = "windows") {
            "Windows".to_string()
        } else {
            "Unknown".to_string()
        };

        let os_version = sys
            .long_os_version()
            .unwrap_or_else(|| sys.os_version().unwrap_or_else(|| "Unknown".to_string()));

        Ok(Self {
            cpu_model: cpu.brand().to_string(),
            cpu_cores: sys.physical_core_count().unwrap_or(0),
            cpu_threads: sys.cpus().len(),
            gpu_model,
            os_name,
            os_version,
        })
    }

    pub fn get_ascii_art(&self) -> String {
        match self.os_name.to_lowercase().as_str() {
            "linux" => format!("OS: Linux {} {}", self.os_version, self.cpu_model),
            "macos" => format!("OS: macOS {} {}", self.os_version, self.cpu_model),
            _ => format!("OS: Windows {} {}", self.os_version, self.cpu_model),
        }
    }

    pub fn get_os_color(&self) -> Color {
        match self.os_name.to_lowercase().as_str() {
            "linux" => Color::LightRed,  // Vermelho claro para Linux
            "macos" => Color::LightBlue, // Azul claro para macOS
            _ => Color::LightCyan,       // Ciano claro para Windows
        }
    }
}

#[cfg(target_os = "linux")]
fn detect_gpu() -> Result<(GpuType, String), Box<dyn Error>> {
    // Tentar NVIDIA primeiro
    if let Ok(nvml) = nvml_wrapper::Nvml::init() {
        if let Ok(device) = nvml.device_by_index(0) {
            return Ok((GpuType::Nvidia, device.name()?.to_string()));
        }
    }

    // Tentar AMD
    if let Ok(contents) = read_to_string("/sys/class/drm/card0/device/vendor") {
        if contents.trim() == "0x1002" {
            let model = read_to_string("/sys/class/drm/card0/device/product_name")?;
            return Ok((GpuType::Amd, model.trim().to_string()));
        }
    }

    Ok((GpuType::Unknown, "Unknown GPU".to_string()))
}

#[cfg(target_os = "macos")]
fn detect_gpu() -> Result<(GpuType, String), Box<dyn Error>> {
    use std::process::Command;

    let output = Command::new("system_profiler")
        .arg("SPDisplaysDataType")
        .output()?;

    let output = String::from_utf8_lossy(&output.stdout);
    let output_lower = output.to_lowercase();

    // Detectar modelo específico
    let model = output
        .lines()
        .find(|line| {
            line.contains("Chip") || 
            line.contains("Model") || 
            line.contains("Chipset") ||
            line.contains("Processor")
        })
        .and_then(|line| line.split(':').nth(1))
        .map(|s| s.trim())
        .unwrap_or_else(|| {
            if output_lower.contains("apple m") || output_lower.contains("apple silicon") {
                "Apple Silicon GPU"
            } else {
                "Apple GPU"
            }
        });

    if output_lower.contains("amd") || output_lower.contains("radeon") {
        Ok((GpuType::Amd, format!("AMD {}", model)))
    } else if output_lower.contains("nvidia") {
        Ok((GpuType::Nvidia, format!("NVIDIA {}", model)))
    } else if output_lower.contains("apple m") || output_lower.contains("apple silicon") {
        Ok((GpuType::Intel, format!("Apple {}", model))) // Usando Intel como tipo para Apple Silicon
    } else {
        Ok((GpuType::Intel, model.to_string()))
    }
}

#[cfg(target_os = "windows")]
fn detect_gpu() -> Result<(GpuType, String), Box<dyn Error>> {
    use std::collections::HashMap;
    use wmi::Variant;

    // Tentar NVIDIA primeiro
    if let Ok(nvml) = nvml_wrapper::Nvml::init() {
        if let Ok(device) = nvml.device_by_index(0) {
            return Ok((GpuType::Nvidia, device.name()?.to_string()));
        }
    }

    // Para AMD e outros, usar WMI
    let com_con = wmi::COMLibrary::new()?;
    let wmi_con = wmi::WMIConnection::new(com_con)?;

    #[derive(Deserialize)]
    struct GPUInfo {
        #[serde(rename = "Caption")]
        caption: String,
    }

    let results: Vec<GPUInfo> = wmi_con.query().map_err(|e| Box::new(e) as Box<dyn Error>)?;

    for gpu in results {
        if gpu.caption.contains("AMD") {
            return Ok((GpuType::Amd, gpu.caption));
        } else if gpu.caption.contains("NVIDIA") {
            return Ok((GpuType::Nvidia, gpu.caption));
        } else if gpu.caption.contains("Intel") {
            return Ok((GpuType::Intel, gpu.caption));
        }
    }

    // Fallback para raw query se a abordagem estruturada falhar
    let results: Vec<HashMap<String, Variant>> = wmi_con
        .raw_query("SELECT Caption FROM Win32_VideoController")
        .map_err(|e| Box::new(e) as Box<dyn Error>)?;

    for gpu in results {
        if let Some(Variant::String(caption)) = gpu.get("Caption") {
            if caption.contains("AMD") {
                return Ok((GpuType::Amd, caption.clone()));
            } else if caption.contains("NVIDIA") {
                return Ok((GpuType::Nvidia, caption.clone()));
            } else if caption.contains("Intel") {
                return Ok((GpuType::Intel, caption.clone()));
            }
        }
    }

    Ok((GpuType::Unknown, "Unknown GPU".to_string()))
}
