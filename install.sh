#!/bin/bash
set -e

# xlsq installer script

REPO="nikhileshva/xlsq"
BINARY_NAME="xlsq"

# Detect OS and architecture
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

case $ARCH in
    x86_64) ARCH="x86_64" ;;
    arm64|aarch64) ARCH="arm64" ;;
    *) echo "Unsupported architecture: $ARCH" && exit 1 ;;
esac

case $OS in
    linux) PLATFORM="linux" ;;
    darwin) PLATFORM="macos" ;;
    *) echo "Unsupported OS: $OS" && exit 1 ;;
esac

BINARY_NAME_FULL="${BINARY_NAME}-${PLATFORM}-${ARCH}"
DOWNLOAD_URL="https://github.com/${REPO}/releases/latest/download/${BINARY_NAME_FULL}"
INSTALL_DIR="$HOME/.local/bin"

echo "Downloading $BINARY_NAME for $OS/$ARCH..."

# Create install directory if it doesn't exist
mkdir -p "$INSTALL_DIR"

# Download binary
if command -v curl >/dev/null 2>&1; then
    curl -L "$DOWNLOAD_URL" -o "$INSTALL_DIR/$BINARY_NAME"
elif command -v wget >/dev/null 2>&1; then
    wget "$DOWNLOAD_URL" -O "$INSTALL_DIR/$BINARY_NAME"
else
    echo "Error: curl or wget is required"
    exit 1
fi

# Make binary executable
chmod +x "$INSTALL_DIR/$BINARY_NAME"

echo "$BINARY_NAME has been installed to $INSTALL_DIR"
echo "Make sure $INSTALL_DIR is in your PATH:"
echo "  export PATH=\"\$HOME/.local/bin:\$PATH\""
echo ""
echo "Run '$BINARY_NAME --help' to get started!"