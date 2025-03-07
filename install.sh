#!/bin/bash

set -e

echo "Installing rshell..."

# Build the Rust project
cargo build --release

# Move the binary to /usr/local/bin
sudo mv target/release/rust_k9s_wizard /usr/local/bin/rshell

echo "rshell installed successfully!"
echo "You can now run 'rshell <pod_name>' from anywhere."
