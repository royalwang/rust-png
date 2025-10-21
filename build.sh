#!/bin/bash

echo "Building Rust PNG Decoder for WASM..."

# Check if wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    echo "Error: wasm-pack is not installed"
    echo "Please install it with: npm install -g wasm-pack"
    exit 1
fi

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "Error: Rust is not installed"
    echo "Please install Rust from https://rustup.rs/"
    exit 1
fi

# Clean previous builds
echo "Cleaning previous builds..."
rm -rf pkg target

# Build WASM module
echo "Building WASM module..."
wasm-pack build --target web --out-dir pkg --out-name index --release

if [ $? -eq 0 ]; then
    echo "Build successful!"
    echo "Output files are in the pkg/ directory"
    echo ""
    echo "To test the module:"
    echo "1. Serve the files with a web server (e.g., python -m http.server)"
    echo "2. Open example.html in your browser"
else
    echo "Build failed!"
    exit 1
fi
