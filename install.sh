#!/bin/bash

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${GREEN}Installing System Monitor...${NC}"

# Check if cargo is installed
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}Error: cargo is not installed${NC}"
    echo "Please install Rust and cargo from https://rustup.rs/"
    exit 1
fi

# Build and install
if cargo install --path .; then
    echo -e "${GREEN}Installation successful!${NC}"
    echo "You can now run 'monitor' from anywhere in your terminal"
else
    echo -e "${RED}Installation failed${NC}"
    exit 1
fi 