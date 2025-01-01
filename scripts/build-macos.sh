#!/bin/bash

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}Compilando System Monitor para macOS...${NC}"

# Criar diretório de output se não existir
mkdir -p ../builds

# macOS (x86_64 e ARM)
echo -e "${GREEN}Compilando para macOS...${NC}"
cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin

cp ../target/x86_64-apple-darwin/release/monitor ../builds/monitor-macos-x86_64
cp ../target/aarch64-apple-darwin/release/monitor ../builds/monitor-macos-arm64

echo -e "${BLUE}Compilação concluída! Verifique a pasta 'builds'${NC}" 