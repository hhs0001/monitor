#!/bin/bash

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m'

echo -e "${GREEN}Instalando System Monitor...${NC}"

# Verificar se cargo está instalado
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}Erro: cargo não está instalado${NC}"
    echo "Por favor, instale o Rust e cargo de https://rustup.rs/"
    exit 1
fi

# Build e instalação
if cargo install --path ..; then
    echo -e "${GREEN}Instalação concluída com sucesso!${NC}"
    echo "Agora você pode executar 'monitor' de qualquer lugar no terminal"
else
    echo -e "${RED}Falha na instalação${NC}"
    exit 1
fi 