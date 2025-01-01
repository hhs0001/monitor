#!/bin/bash

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}Compilando System Monitor para macOS...${NC}"

# Verificar se estamos em um macOS
if [[ $(uname) != "Darwin" ]]; then
    echo -e "${RED}Este script deve ser executado em um macOS${NC}"
    exit 1
fi

# Criar diretório de output se não existir
mkdir -p builds

# Detectar arquitetura do sistema
ARCH=$(uname -m)
echo -e "${GREEN}Arquitetura detectada: $ARCH${NC}"

if [[ "$ARCH" == "arm64" ]]; then
    # Compilar para Apple Silicon (M1/M2)
    echo -e "${GREEN}Compilando para Apple Silicon...${NC}"
    cargo build --release --target aarch64-apple-darwin
    cp target/aarch64-apple-darwin/release/monitor builds/monitor-macos-arm64
    echo -e "${BLUE}Executável criado em: builds/monitor-macos-arm64${NC}"
else
    # Compilar para Intel
    echo -e "${GREEN}Compilando para Intel...${NC}"
    cargo build --release --target x86_64-apple-darwin
    cp target/x86_64-apple-darwin/release/monitor builds/monitor-macos-x64
    echo -e "${BLUE}Executável criado em: builds/monitor-macos-x64${NC}"
fi

# Tornar o executável executável
chmod +x builds/monitor-macos-*

echo -e "${GREEN}Compilação concluída!${NC}" 