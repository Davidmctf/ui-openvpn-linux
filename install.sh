#!/bin/bash

set -e

echo "🚀 OpenVPN Manager - Installation Script"
echo "========================================"

# Check if running as root
if [[ $EUID -eq 0 ]]; then
   echo "❌ Don't run this script as root!"
   exit 1
fi

# Check dependencies
echo "📋 Checking system dependencies..."

# Check if NetworkManager is available
if ! command -v nmcli &> /dev/null; then
    echo "❌ NetworkManager (nmcli) is required but not installed."
    echo "   Install with: sudo dnf install NetworkManager"
    exit 1
fi

# Check if OpenVPN is available  
if ! rpm -qa | grep -q openvpn; then
    echo "⚠️  OpenVPN is recommended but not installed."
    echo "   Install with: sudo dnf install openvpn NetworkManager-openvpn"
fi

# Check if Rust is available
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust/Cargo is required but not installed."
    echo "   Install with: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

echo "✅ Dependencies check passed"

# Compile with web features
echo "🔧 Compiling OpenVPN Manager with web interface..."
cargo build --release --features web

# Install binary
echo "📦 Installing binary to /usr/local/bin..."
sudo cp target/release/openvpn-manager /usr/local/bin/openvpn-manager
sudo chmod +x /usr/local/bin/openvpn-manager

# Create desktop entry for web interface
echo "🖥️  Creating desktop entry..."
mkdir -p ~/.local/share/applications

cat > ~/.local/share/applications/openvpn-manager.desktop << 'EOF'
[Desktop Entry]
Name=🚀 OpenVPN Manager
Comment=Navegador Espacial de Conexiones VPN
Exec=openvpn-manager web
Icon=network-vpn
Terminal=false
Type=Application
Categories=Network;
StartupNotify=true
Keywords=vpn;openvpn;network;
EOF

# Update desktop database
if command -v update-desktop-database &> /dev/null; then
    update-desktop-database ~/.local/share/applications
fi

echo ""
echo "🎉 Installation completed!"
echo ""
echo "📋 Available commands:"
echo "   openvpn-manager list              # List VPNs"
echo "   openvpn-manager connect <name>    # Connect to VPN" 
echo "   openvpn-manager disconnect        # Disconnect VPN"
echo "   openvpn-manager status            # Check status"
echo "   openvpn-manager web               # Launch web interface"
echo "   openvpn-manager import <file>     # Import .ovpn file"
echo ""
echo "🌐 Web interface will be available at: http://localhost:8081"
echo "🖥️  Desktop app: Search 'OpenVPN Manager' in your applications"
echo ""
echo "💡 Try: openvpn-manager list"