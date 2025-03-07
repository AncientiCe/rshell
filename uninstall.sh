#!/bin/bash

set -e

echo "Uninstalling rshell..."

# Remove the binary from /usr/local/bin
sudo rm -f /usr/local/bin/rshell

echo "rshell has been uninstalled successfully."
