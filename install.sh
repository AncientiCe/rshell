#!/bin/bash

set -e

echo "Installing rshell..."

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "Rust is not installed. Please install Rust first: https://www.rust-lang.org/tools/install"
    exit 1
fi

# Create a temporary directory
TEMP_DIR=$(mktemp -d)
cd $TEMP_DIR

# Clone the repository
git clone https://github.com/AncientiCe/rshell.git
cd rshell

# Build the Rust project
cargo build --release

# Move the binary to /usr/local/bin
sudo mv target/release/rust_k9s_wizard /usr/local/bin/rshell

# Clean up
cd ~
rm -rf $TEMP_DIR

echo "rshell installed successfully!"
echo "You can now run 'rshell <pod_name>' from anywhere."
