#!/bin/bash

set -e

echo "🗑️  OpenVPN Manager - Complete Uninstaller"
echo "==========================================="

# Function to remove file safely
remove_file() {
    if [ -f "$1" ]; then
        echo "🗑️  Removing: $1"
        sudo rm -f "$1" 2>/dev/null || rm -f "$1" 2>/dev/null
    fi
}

# Function to remove directory safely
remove_dir() {
    if [ -d "$1" ]; then
        echo "🗑️  Removing directory: $1"
        sudo rm -rf "$1" 2>/dev/null || rm -rf "$1" 2>/dev/null
    fi
}

echo "🔍 Searching and removing OpenVPN Manager files..."

# 1. Remove main binary from all possible locations
remove_file "/usr/local/bin/openvpn-manager"
remove_file "/usr/bin/openvpn-manager"
remove_file "$HOME/.local/bin/openvpn-manager"
remove_file "$HOME/.cargo/bin/openvpn-manager"

# 2. Remove desktop entries and icons (both possible names)
remove_file "$HOME/.local/share/applications/openvpn-manager.desktop"
remove_file "$HOME/.local/share/applications/ui-openvpn-linux.desktop"
remove_file "/usr/share/applications/openvpn-manager.desktop"
remove_file "/usr/share/applications/ui-openvpn-linux.desktop"
remove_file "$HOME/Desktop/openvpn-manager.desktop"
remove_file "$HOME/Desktop/ui-openvpn-linux.desktop"
remove_file "$HOME/.local/share/icons/hicolor/256x256/apps/openvpn-manager.png"
remove_file "$HOME/.local/share/pixmaps/openvpn-manager.png"

# 3. Remove any created config directories
remove_dir "$HOME/.config/openvpn-manager"
remove_dir "$HOME/.connectvpn.conf"

# 4. Remove aliases from shell config files
echo "🔗 Cleaning shell aliases..."
for shell_config in "$HOME/.bashrc" "$HOME/.zshrc" "$HOME/.profile" "$HOME/.bash_profile"; do
    if [ -f "$shell_config" ]; then
        # Create backup
        cp "$shell_config" "$shell_config.backup.$(date +%Y%m%d_%H%M%S)" 2>/dev/null || true
        
        # Remove OpenVPN Manager aliases section
        if grep -q "OpenVPN Manager aliases" "$shell_config" 2>/dev/null; then
            echo "🧹 Cleaning aliases from: $shell_config"
            # Remove lines between "# OpenVPN Manager aliases" and next empty line or EOF
            sed -i '/# OpenVPN Manager aliases/,/^$/d' "$shell_config" 2>/dev/null || true
        fi
    fi
done

# 5. Remove any systemd services (if any were created)
remove_file "$HOME/.config/systemd/user/openvpn-manager.service"
remove_file "/etc/systemd/system/openvpn-manager.service"

# 6. Remove any created symlinks
find /usr/local/bin /usr/bin "$HOME/.local/bin" -name "*openvpn*" -type l -delete 2>/dev/null || true

# 7. Clean cargo cache if exists
if [ -d "$HOME/.cargo" ]; then
    echo "🧹 Cleaning cargo cache..."
    cargo cache --remove-dir openvpn-manager 2>/dev/null || true
fi

# 8. Remove build artifacts from current directory
if [ -d "target" ]; then
    echo "🧹 Cleaning build artifacts..."
    rm -rf target/
fi

# 9. Clean desktop cache and update database
if command -v gtk-update-icon-cache &> /dev/null; then
    gtk-update-icon-cache -f -t "$HOME/.local/share/icons/hicolor" 2>/dev/null || true
fi

if command -v update-desktop-database &> /dev/null; then
    update-desktop-database "$HOME/.local/share/applications" 2>/dev/null || true
fi

# Force refresh desktop entries
if command -v xdg-desktop-menu &> /dev/null; then
    xdg-desktop-menu forceupdate 2>/dev/null || true
fi

# 10. Kill any running processes
echo "⏹️  Stopping any running OpenVPN Manager processes..."
pkill -f openvpn-manager 2>/dev/null || true

echo ""
echo "✅ Complete uninstallation finished!"
echo "📋 What was removed:"
echo "   • Binary files from /usr/local/bin, /usr/bin, ~/.local/bin"
echo "   • Desktop entries from applications menu"
echo "   • Shell aliases from ~/.bashrc, ~/.zshrc, etc."
echo "   • Configuration directories"
echo "   • Build artifacts and cache"
echo ""
echo "💡 Shell config files were backed up with timestamp"
echo "🔄 Please restart your terminal or run: source ~/.bashrc"
echo ""
echo "🎯 System is now clean - ready for fresh installation!"