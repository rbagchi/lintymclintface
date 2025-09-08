#!/bin/bash

set -e

INSTALL_DIR="/usr/local/bin"
BINARY_NAME="lintymclintface"
PROJECT_ROOT="$(dirname "$(readlink -f "$0")")"

# Function to check if a command exists
command_exists () {
  command -v "$1" >/dev/null 2>&1
}

# Check for Rust and Cargo
if ! command_exists cargo; then
  echo "Error: Cargo (Rust package manager) is not installed."
  echo "Please install Rust and Cargo: https://www.rust-lang.org/tools/install"
  exit 1
fi

# Build the project
echo "Building $BINARY_NAME in release mode..."
cd "$PROJECT_ROOT"
cargo build --release

# Check if build was successful
if [ ! -f "target/release/$BINARY_NAME" ]; then
  echo "Error: Build failed. Executable not found at target/release/$BINARY_NAME"
  exit 1
fi

# Installation
echo "Installing $BINARY_NAME to $INSTALL_DIR/"
sudo cp "target/release/$BINARY_NAME" "$INSTALL_DIR/"

echo "Installation complete. You can now run '$BINARY_NAME' from anywhere."
echo "To uninstall, run: sudo rm $INSTALL_DIR/$BINARY_NAME"
