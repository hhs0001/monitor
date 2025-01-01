# Monitor do Sistema

*Leia em outros idiomas: [Inglês](README.md), [Português](README.pt-br.md)*

Um monitor de recursos do sistema multiplataforma escrito em Rust, que fornece informações em tempo real sobre CPU, memória, GPU e rede em uma interface de terminal interativa. Este é um projeto de aprendizado enquanto exploro Rust, então, embora totalmente funcional, o código pode não seguir todas as melhores práticas.

![Status: Em Desenvolvimento](https://img.shields.io/badge/Status-Em%20Desenvolvimento-yellow)
![Licença: MIT](https://img.shields.io/badge/Licença-MIT-green)
![Versão Rust: 1.75+](https://img.shields.io/badge/Rust-1.75+-orange)

## Características

- 📊 Visualização em tempo real com gráficos TUI (Interface de Usuário em Terminal)
- 🖥️ Monitoramento detalhado de CPU com suporte multicore
- 🎮 Suporte a GPUs NVIDIA, AMD e Intel
- 💾 Monitoramento de memória RAM e SWAP
- 🌐 Estatísticas de rede por interface
- 🎯 Interface interativa e responsiva
- ⚙️ Configurações personalizáveis e persistentes
- 💻 Suporte para Windows, Linux e macOS

## Instalação

### Binários Pré-compilados

Você pode baixar os binários pré-compilados para seu sistema operacional na página de [Releases](https://github.com/hhs0001/monitor/releases). Disponível para:
- Windows (x64)
- Linux (x64)
- macOS (Intel x64 e Apple Silicon)

### Usando o Script de Instalação

```bash
./install.sh
```

### Compilando Manualmente

1. Certifique-se de ter o Rust e Cargo instalados
2. Clone o repositório
3. Execute um dos scripts de compilação de acordo com seu sistema operacional:

**Linux:**
```bash
./build-linux.sh
```

**macOS:**
```bash
./build-mac.sh
```

## Uso

```bash
monitor [OPÇÕES]
```

### Opções

- `--no-gpu`: Desativa o monitoramento de GPU
- `--no-network`: Desativa o monitoramento de rede
- `--interval <MS>`: Define o intervalo de atualização em milissegundos (padrão: 50)
- `--history <N>`: Define o número de pontos de dados nos gráficos (padrão: 100)
- `--save-config`: Salva as configurações atuais como padrão
- `--reset-config`: Restaura as configurações para o padrão

### Controles

- `q`: Sair do programa
- `Ctrl+C`: Sair do programa

## Requisitos do Sistema

- **Sistema Operacional:** Windows, Linux ou macOS
- **GPU (opcional):** 
  - NVIDIA: Drivers NVIDIA e NVML
  - AMD: Drivers AMD
  - Intel: Drivers Intel

## Configuração

O arquivo de configuração é armazenado em:
- Linux: `~/.config/system-monitor/config.toml`
- macOS: `~/Library/Application Support/system-monitor/config.toml`
- Windows: `%APPDATA%\system-monitor\config.toml`

## Dependências Principais

- `tui`: Interface de usuário em terminal
- `crossterm`: Manipulação do terminal multiplataforma
- `sysinfo`: Informações do sistema
- `nvml-wrapper`: Suporte a GPUs NVIDIA
- `clap`: Processamento de argumentos de linha de comando

## Contribuindo

1. Fork o projeto
2. Crie uma branch para sua feature (`git checkout -b feature/NovaFuncionalidade`)
3. Commit suas mudanças (`git commit -m 'Adiciona nova funcionalidade'`)
4. Push para a branch (`git push origin feature/NovaFuncionalidade`)
5. Abra um Pull Request

## Licença

Este projeto está licenciado sob a Licença MIT - veja o arquivo LICENSE para detalhes.

## Agradecimentos

- Comunidade Rust
- Contribuidores do projeto tui-rs
- Desenvolvedores das bibliotecas utilizadas