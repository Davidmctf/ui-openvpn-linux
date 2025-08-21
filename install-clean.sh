#!/bin/bash

set -e

echo "🚀 OpenVPN Manager - Clean Installation"
echo "======================================="

# Check if running as root
if [[ $EUID -eq 0 ]]; then
   echo "❌ Don't run this script as root!"
   exit 1
fi

# Function to check command exists
check_command() {
    if ! command -v "$1" &> /dev/null; then
        echo "❌ $1 is required but not installed."
        echo "   Install with: $2"
        exit 1
    else
        echo "✅ $1 found"
    fi
}

# Check if we're in the right directory
if [[ ! -f "Cargo.toml" ]]; then
    echo "❌ Please run this script from the project root directory"
    echo "   (where Cargo.toml is located)"
    exit 1
fi

echo "📋 Checking system dependencies..."

# Check dependencies
check_command "nmcli" "sudo dnf install NetworkManager"
check_command "cargo" "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"

# Check if OpenVPN is available (optional but recommended)
if ! rpm -qa | grep -q openvpn 2>/dev/null && ! dpkg -l | grep -q openvpn 2>/dev/null; then
    echo "⚠️  OpenVPN is recommended but not installed."
    echo "   Fedora: sudo dnf install openvpn NetworkManager-openvpn"
    echo "   Ubuntu: sudo apt install openvpn network-manager-openvpn"
    echo ""
    read -p "Continue anyway? (y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
fi

echo "✅ Dependencies check passed"
echo ""

# Clean any previous builds
if [ -d "target" ]; then
    echo "🧹 Cleaning previous builds..."
    cargo clean
fi

# Compile with web features
echo "🔧 Compiling OpenVPN Manager..."
echo "   This may take a few minutes on first run..."
cargo build --release --features web

if [ ! -f "target/release/openvpn-manager" ]; then
    echo "❌ Build failed - binary not found"
    exit 1
fi

echo "✅ Build successful"

# Install binary
echo "📦 Installing binary to /usr/local/bin..."
if sudo cp target/release/openvpn-manager /usr/local/bin/openvpn-manager; then
    sudo chmod +x /usr/local/bin/openvpn-manager
    echo "✅ Binary installed successfully"
else
    echo "❌ Failed to install binary - check sudo permissions"
    exit 1
fi

# Test installation
echo "🧪 Testing installation..."
if openvpn-manager --help >/dev/null 2>&1; then
    echo "✅ Binary is working correctly"
else
    echo "❌ Binary test failed"
    exit 1
fi

# Install logo
echo "🎨 Installing app logo..."
mkdir -p ~/.local/share/icons/hicolor/256x256/apps
if [ -f "assets/openvpn-manager-logo.png" ]; then
    cp assets/openvpn-manager-logo.png ~/.local/share/icons/hicolor/256x256/apps/openvpn-manager.png
    echo "✅ Logo installed"
else
    echo "⚠️  Logo not found, using default icon"
fi

# Create desktop entry
echo "🖥️  Creating desktop entry..."
mkdir -p ~/.local/share/applications

cat > ~/.local/share/applications/openvpn-manager.desktop << 'EOF'
[Desktop Entry]
Name=OpenVPN Manager
Comment=🌌 Navegador Espacial de Conexiones VPN
Exec=openvpn-manager web
Icon=openvpn-manager
Terminal=false
Type=Application
Categories=Network;System;
StartupNotify=true
Keywords=vpn;openvpn;network;manager;space;
Version=1.0
EOF

# Update desktop database
if command -v update-desktop-database &> /dev/null; then
    update-desktop-database ~/.local/share/applications 2>/dev/null || true
fi

echo "✅ Desktop entry created"
echo ""

# Final verification
echo "🔍 Final verification..."
INSTALLED_VERSION=$(openvpn-manager --version 2>/dev/null | head -1 || echo "unknown")
echo "✅ Installed version: $INSTALLED_VERSION"

echo ""
echo "🎉 Installation completed successfully!"
echo ""
echo "📋 Available commands:"
echo "   openvpn-manager list              # List VPNs"
echo "   openvpn-manager connect <name>    # Connect to VPN" 
echo "   openvpn-manager disconnect        # Disconnect VPN"
echo "   openvpn-manager status            # Check status"
echo "   openvpn-manager web               # Launch web interface"
echo "   openvpn-manager import <file>     # Import .ovpn file"
echo ""
echo "🌐 Web interface: http://localhost:8081"
echo "🖥️  Desktop app: Search 'OpenVPN Manager' in applications"
echo ""
echo "💡 Try it now: openvpn-manager list"