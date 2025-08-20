#!/bin/bash

# 🚀 UI OpenVPN Linux - Instalador Automático Espacial
# ====================================================

set -e  # Salir si cualquier comando falla

# Colores para output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # Sin color

# Logo espacial
print_space_logo() {
    echo -e "${CYAN}"
    echo "      🚀       🌟      🪐      "
    echo "         \\       |     /       "
    echo "          \\      |    /        "
    echo "           \\     |   /         "
    echo "     🌟     ╔═══════╗      ⭐  "
    echo "            ║  👨‍🚀   ║           "
    echo "            ║ ┌─────┐║           "
    echo "     ⭐     ║ │💻🔐 │║      🌟  "
    echo "            ║ └─────┘║           "
    echo "            ╚═══════╝           "
    echo "             ╱     ╲            "
    echo "            ╱ 🚀🔧 ╲           "
    echo "           ╱_________╲          "
    echo "          🪐          🌟        "
    echo ""
    echo "  🚀 UI OpenVPN Linux Installer 🚀   "
    echo "  ══════════════════════════════════════════"
    echo "  Instalación Automática de VPN Espacial! ✨"
    echo -e "${NC}"
}

# Detectar distribución
detect_distro() {
    if command -v dnf &> /dev/null; then
        echo "fedora"
    elif command -v apt &> /dev/null; then
        echo "debian"
    elif command -v pacman &> /dev/null; then
        echo "arch"
    elif command -v zypper &> /dev/null; then
        echo "opensuse"
    else
        echo "unknown"
    fi
}

# Función para imprimir mensajes
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[✅]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[⚠️]${NC} $1"
}

log_error() {
    echo -e "${RED}[❌]${NC} $1"
}

log_step() {
    echo -e "${PURPLE}[🚀]${NC} $1"
}

# Verificar si el usuario es root (para algunas instalaciones)
check_sudo() {
    if ! command -v sudo &> /dev/null; then
        log_error "sudo no está disponible. Por favor instala sudo primero."
        exit 1
    fi
}

# Instalar Rust y Cargo
install_rust() {
    log_step "Instalando Rust y Cargo..."
    
    if command -v cargo &> /dev/null; then
        log_success "Rust ya está instalado: $(cargo --version)"
        return
    fi
    
    log_info "Descargando e instalando Rust desde rustup.rs..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    
    # Agregar al PATH para esta sesión
    source "$HOME/.cargo/env"
    
    log_success "Rust instalado exitosamente: $(cargo --version)"
}

# Instalar dependencias del sistema según la distribución
install_system_deps() {
    local distro=$(detect_distro)
    log_step "Instalando dependencias del sistema para $distro..."
    
    case $distro in
        "fedora")
            log_info "Instalando dependencias para Fedora/RHEL/CentOS..."
            sudo dnf update -y
            sudo dnf install -y \
                gcc \
                openssl-devel \
                pkg-config \
                gtk4-devel \
                cairo-devel \
                glib2-devel \
                pango-devel \
                gdk-pixbuf2-devel \
                openvpn \
                git \
                curl \
                build-essential 2>/dev/null || true
            ;;
        "debian")
            log_info "Instalando dependencias para Ubuntu/Debian..."
            sudo apt update
            sudo apt install -y \
                build-essential \
                libssl-dev \
                pkg-config \
                libgtk-4-dev \
                libcairo2-dev \
                libglib2.0-dev \
                libpango1.0-dev \
                libgdk-pixbuf-2.0-dev \
                openvpn \
                git \
                curl \
                ca-certificates
            ;;
        "arch")
            log_info "Instalando dependencias para Arch Linux..."
            sudo pacman -Sy --noconfirm \
                base-devel \
                openssl \
                pkg-config \
                gtk4 \
                cairo \
                glib2 \
                pango \
                gdk-pixbuf2 \
                openvpn \
                git \
                curl
            ;;
        "opensuse")
            log_info "Instalando dependencias para openSUSE..."
            sudo zypper refresh
            sudo zypper install -y \
                gcc \
                openssl-devel \
                pkg-config \
                gtk4-devel \
                cairo-devel \
                glib2-devel \
                pango-devel \
                gdk-pixbuf-devel \
                openvpn \
                git \
                curl
            ;;
        *)
            log_warning "Distribución no reconocida. Por favor instala manualmente:"
            log_info "- Rust (https://rustup.rs/)"
            log_info "- GTK4 development headers"
            log_info "- OpenSSL development headers"
            log_info "- pkg-config"
            log_info "- OpenVPN"
            log_info "- git y curl"
            ;;
    esac
    
    log_success "Dependencias del sistema instaladas"
}

# Clonar o actualizar repositorio
setup_repository() {
    log_step "Configurando repositorio..."
    
    local repo_dir="$HOME/ui-openvpn-linux"
    
    if [ -d "$repo_dir" ]; then
        log_info "Repositorio existe, actualizando..."
        cd "$repo_dir"
        git pull origin main
    else
        log_info "Clonando repositorio desde GitHub..."
        git clone https://github.com/Davidmctf/ui-openvpn-linux.git "$repo_dir"
        cd "$repo_dir"
    fi
    
    log_success "Repositorio configurado en $repo_dir"
}

# Compilar aplicación
compile_application() {
    log_step "Compilando aplicación..."
    
    # Asegurarse de que cargo esté en el PATH
    source "$HOME/.cargo/env" 2>/dev/null || true
    
    log_info "Compilando versión CLI optimizada..."
    cargo build --release
    
    log_info "Intentando compilar versión GUI..."
    if cargo build --features gui --release 2>/dev/null; then
        log_success "Versión GUI compilada exitosamente"
        echo "gui_available=true" > .build_info
    else
        log_warning "GUI no disponible (dependencias GTK4 faltantes)"
        echo "gui_available=false" > .build_info
    fi
    
    log_success "Aplicación compilada exitosamente"
}

# Configurar directorios VPN
setup_vpn_config() {
    log_step "Configurando directorio VPN..."
    
    local config_dir="$HOME/.connectvpn.conf"
    mkdir -p "$config_dir"
    
    # Copiar archivos de ejemplo si existen
    if [ -d "vpn_configs" ] && [ "$(ls -A vpn_configs 2>/dev/null)" ]; then
        log_info "Copiando configuraciones VPN de ejemplo..."
        cp vpn_configs/*.ovpn "$config_dir/" 2>/dev/null || true
    fi
    
    log_success "Directorio VPN configurado: $config_dir"
}

# Instalar binario globalmente
install_globally() {
    log_step "Instalando binario globalmente..."
    
    local install_dir="$HOME/.local/bin"
    mkdir -p "$install_dir"
    
    # Copiar binario
    cp target/release/ui-openvpn "$install_dir/"
    chmod +x "$install_dir/ui-openvpn"
    
    # Agregar al PATH si no está
    if ! echo "$PATH" | grep -q "$install_dir"; then
        echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$HOME/.bashrc"
        echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$HOME/.zshrc" 2>/dev/null || true
        export PATH="$HOME/.local/bin:$PATH"
    fi
    
    log_success "Binario instalado en $install_dir/ui-openvpn"
}

# Instalar icono y desktop file
install_desktop_integration() {
    log_step "Instalando integración con el escritorio..."
    
    local apps_dir="$HOME/.local/share/applications"
    local icons_dir="$HOME/.local/share/icons"
    
    mkdir -p "$apps_dir"
    mkdir -p "$icons_dir"
    
    # Instalar icono (usar el PNG que ya tienes)
    if [ -f "assets/2261aaaa-bad7-4426-8cc1-f93cd6c4c067.png" ]; then
        cp "assets/2261aaaa-bad7-4426-8cc1-f93cd6c4c067.png" "$icons_dir/ui-openvpn-linux.png"
        log_success "Icono instalado: $icons_dir/ui-openvpn-linux.png"
    fi
    
    # Instalar desktop file
    if [ -f "ui-openvpn-linux.desktop" ]; then
        cp "ui-openvpn-linux.desktop" "$apps_dir/"
        chmod +x "$apps_dir/ui-openvpn-linux.desktop"
        
        # Actualizar base de datos de aplicaciones
        if command -v update-desktop-database &> /dev/null; then
            update-desktop-database "$apps_dir" 2>/dev/null || true
        fi
        
        log_success "Acceso directo instalado en el menú de aplicaciones"
    fi
    
    log_info "🚀 La aplicación ahora aparecerá en tu menú de aplicaciones"
}

# Crear alias útiles
create_aliases() {
    log_step "Creando aliases útiles..."
    
    local aliases='
# 🚀 UI OpenVPN Linux Aliases
alias vpn="ui-openvpn"
alias vpn-list="ui-openvpn list"
alias vpn-status="ui-openvpn status"
alias vpn-gui="ui-openvpn --gui"
'
    
    echo "$aliases" >> "$HOME/.bash_aliases" 2>/dev/null || echo "$aliases" >> "$HOME/.bashrc"
    
    log_success "Aliases creados (vpn, vpn-list, vpn-status, vpn-gui)"
}

# Verificar instalación
verify_installation() {
    log_step "Verificando instalación..."
    
    # Reiniciar el PATH
    source "$HOME/.cargo/env" 2>/dev/null || true
    export PATH="$HOME/.local/bin:$PATH"
    
    if command -v ui-openvpn &> /dev/null; then
        log_success "✅ ui-openvpn instalado correctamente"
        log_info "Versión: $(ui-openvpn --version 2>/dev/null || echo 'Desarrollo')"
    else
        log_error "❌ Instalación falló - binario no encontrado"
        return 1
    fi
    
    # Verificar OpenVPN
    if command -v openvpn &> /dev/null; then
        log_success "✅ OpenVPN disponible: $(openvpn --version | head -n1)"
    else
        log_warning "⚠️ OpenVPN no encontrado - instálalo manualmente"
    fi
    
    # Verificar GUI
    if [ -f .build_info ] && grep -q "gui_available=true" .build_info; then
        log_success "✅ Interfaz gráfica GTK4 disponible"
    else
        log_warning "⚠️ Interfaz gráfica no disponible"
    fi
}

# Mostrar instrucciones finales
show_completion_message() {
    echo -e "${GREEN}"
    echo "🚀 ¡INSTALACIÓN COMPLETADA EXITOSAMENTE! 🚀"
    echo "════════════════════════════════════════════"
    echo -e "${NC}"
    
    echo -e "${CYAN}Comandos disponibles:${NC}"
    echo "  ui-openvpn --help          # Ver ayuda completa"
    echo "  ui-openvpn list            # Listar VPNs disponibles"
    echo "  ui-openvpn connect <vpn>   # Conectar VPN"
    echo "  ui-openvpn status          # Ver estado"
    echo "  ui-openvpn --gui           # Interfaz gráfica (si disponible)"
    echo ""
    
    echo -e "${CYAN}Aliases útiles:${NC}"
    echo "  vpn --help                 # Comando corto"
    echo "  vpn-list                   # Listar VPNs"
    echo "  vpn-status                 # Ver estado"
    echo "  vpn-gui                    # Abrir GUI"
    echo ""
    
    echo -e "${YELLOW}Para usar VPNs:${NC}"
    echo "1. Coloca tus archivos .ovpn en: ~/.connectvpn.conf/"
    echo "2. Ejecuta: vpn list"
    echo "3. Conecta: vpn connect <nombre>"
    echo ""
    
    echo -e "${PURPLE}¡Reinicia tu terminal o ejecuta:${NC}"
    echo "source ~/.bashrc"
    echo ""
    
    echo -e "${GREEN}🌌 ¡Que tengas un buen viaje por el ciberespacio! ✨${NC}"
}

# Función principal
main() {
    print_space_logo
    
    log_info "Iniciando instalación automática de UI OpenVPN Linux..."
    
    # Verificar prerequisitos
    check_sudo
    
    # Proceso de instalación
    install_rust
    install_system_deps
    setup_repository
    compile_application
    setup_vpn_config
    install_globally
    install_desktop_integration
    create_aliases
    verify_installation
    
    show_completion_message
    
    log_success "🚀 Instalación completada en $(date)"
}

# Manejar señales para limpieza
trap 'log_error "Instalación interrumpida"; exit 1' INT TERM

# Ejecutar instalación si el script se ejecuta directamente
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main "$@"
fi