#!/bin/bash

set -e

echo "Installing rshell..."

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "Rust is not installed. Please install Rust first: https://www.rust-lang.org/tools/install"
    exit 1
fi

# Clone the repository if not already present
if [ ! -d "rshell" ]; then
    git clone https://github.com/AncientiCe/rshell.git
    cd rshell
else
    cd rshell
    git pull origin main
fi

# Build the Rust project
cargo build --release

# Move the binary to /usr/local/bin
sudo mv target/release/rust_k9s_wizard /usr/local/bin/rshell

echo "rshell installed successfully!"
echo "You can now run 'rshell <pod_name>' from anywhere."
