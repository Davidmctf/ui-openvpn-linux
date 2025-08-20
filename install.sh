#!/bin/bash

echo "🔐 Installing OpenVPN Manager..."
echo "================================"

# Build the application
echo "📦 Building application..."
cargo build --release

# Check if build was successful
if [ $? -eq 0 ]; then
    echo "✅ Build successful!"
else
    echo "❌ Build failed!"
    exit 1
fi

# Install system-wide
echo "🏗️  Installing system-wide..."
sudo cp target/release/openvpn-manager /usr/local/bin/

# Check if installation was successful
if [ $? -eq 0 ]; then
    echo "✅ Installation successful!"
    echo ""
    echo "📋 You can now use 'openvpn-manager' from anywhere:"
    echo "   openvpn-manager --help"
    echo "   openvpn-manager list"
    echo "   openvpn-manager import ~/my-vpn.ovpn"
    echo "   openvpn-manager connect <name>"
    echo ""
    echo "💡 Make sure you have NetworkManager OpenVPN plugin installed:"
    echo "   sudo dnf install NetworkManager-openvpn-gnome  # Fedora"
    echo "   sudo apt install network-manager-openvpn-gnome # Ubuntu/Debian"
else
    echo "❌ Installation failed!"
    exit 1
fi