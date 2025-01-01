#!/bin/bash

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}Compilando System Monitor para Linux...${NC}"

# Criar diretório de output se não existir
mkdir -p ../builds

# Linux (x86_64)
echo -e "${GREEN}Compilando para Linux (x86_64)...${NC}"
cargo build --release --target x86_64-unknown-linux-gnu
cp ../target/x86_64-unknown-linux-gnu/release/monitor ../builds/monitor-linux-x86_64

echo -e "${BLUE}Compilação concluída! Verifique a pasta 'builds'${NC}" 