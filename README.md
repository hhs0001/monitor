# System Monitor

*Read this in other languages: [English](README.md), [Portuguese](README.pt-br.md)*

A cross-platform system resource monitor written in Rust that provides real-time information about CPU, memory, GPU, and network in an interactive terminal interface. This is a learning project as I'm exploring Rust, so while fully functional, the code might not follow all best practices.

![Status: In Development](https://img.shields.io/badge/Status-In%20Development-yellow)
![License: MIT](https://img.shields.io/badge/License-MIT-green)
![Rust Version: 1.75+](https://img.shields.io/badge/Rust-1.75+-orange)

## Features

- 📊 Real-time visualization with TUI (Terminal User Interface) graphs
- 🖥️ Detailed CPU monitoring with multicore support
- 🎮 NVIDIA, AMD, and Intel GPU support
- 💾 RAM and SWAP memory monitoring
- 🌐 Network statistics per interface
- 🎯 Interactive and responsive interface
- ⚙️ Customizable and persistent settings
- 💻 Support for Windows, Linux, and macOS

## Installation

### Pre-compiled Binaries

You can download the pre-compiled binaries for your operating system from the [Releases](https://github.com/hhs0001/monitor/releases) page. Available for:
- Windows (x64)
- Linux (x64)
- macOS (Intel x64 and Apple Silicon)

### Using Installation Script

```bash
./install.sh
```

### Manual Compilation

1. Make sure you have Rust and Cargo installed
2. Clone the repository
3. Run one of the build scripts according to your operating system:

**Linux:**
```bash
./build-linux.sh
```

**macOS:**
```bash
./build-mac.sh
```

## Usage

```bash
monitor [OPTIONS]
```

### Options

- `--no-gpu`: Disable GPU monitoring
- `--no-network`: Disable network monitoring
- `--interval <MS>`: Set update interval in milliseconds (default: 50)
- `--history <N>`: Set number of data points in graphs (default: 100)
- `--save-config`: Save current settings as default
- `--reset-config`: Reset settings to default

### Controls

- `q`: Quit program
- `Ctrl+C`: Quit program

## System Requirements

- **Operating System:** Windows, Linux, or macOS
- **GPU (optional):** 
  - NVIDIA: NVIDIA drivers and NVML
  - AMD: AMD drivers
  - Intel: Intel drivers

## Configuration

The configuration file is stored in:
- Linux: `~/.config/system-monitor/config.toml`
- macOS: `~/Library/Application Support/system-monitor/config.toml`
- Windows: `%APPDATA%\system-monitor\config.toml`

## Main Dependencies

- `tui`: Terminal user interface
- `crossterm`: Cross-platform terminal manipulation
- `sysinfo`: System information
- `nvml-wrapper`: NVIDIA GPU support
- `clap`: Command line argument processing

## Contributing

1. Fork the project
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Rust Community
- tui-rs contributors
- Developers of used libraries