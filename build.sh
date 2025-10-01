#!/bin/bash
# Build script for VedDB Rust Client

echo "Building VedDB Rust Client..."
cargo build --release
if [ $? -ne 0 ]; then
    echo "Failed to build client"
    exit 1
fi

echo ""
echo "Build complete!"
echo "Client library: target/release/"
echo "CLI tool: target/release/veddb-cli"
