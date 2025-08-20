# 🚀 Guía de Instalación - UI OpenVPN Linux

<div align="center">

![Astronauta VPN](./assets/2261aaaa-bad7-4426-8cc1-f93cd6c4c067.png)

**Navigate through secure connections like an astronaut exploring cyberspace!**

</div>

## 📋 Requisitos del Sistema

### Sistemas Operativos Soportados
- **Fedora** 35+ (recomendado)
- **Ubuntu/Debian** 20.04+
- **Arch Linux**
- **openSUSE**
- Cualquier distribución Linux con NetworkManager

### Dependencias Requeridas
- **NetworkManager** >= 1.20
- **OpenVPN** >= 2.4
- **NetworkManager-openvpn** plugin
- **Rust** >= 1.70 (para compilar)

## 🎯 Métodos de Instalación

### Método 1: 🚀 Instalación Ultra-Rápida (Recomendado)

```bash
# Un solo comando que instala TODO automáticamente
curl -sSL https://raw.githubusercontent.com/Davidmctf/ui-openvpn-linux/main/install.sh | bash
```

**¡Eso es todo!** El script automáticamente:
- ✅ Instala Rust y Cargo
- ✅ Instala todas las dependencias del sistema
- ✅ Clona y compila la aplicación
- ✅ Instala el binario globalmente
- ✅ Crea icono en el escritorio y menú
- ✅ Configura aliases útiles

### Método 2: 📦 Paquetes Precompilados (Próximamente)

#### Para Fedora/RHEL:
```bash
# Agregar repositorio COPR
sudo dnf copr enable davidmctf/openvpn-manager
sudo dnf install ui-openvpn-linux
```

#### Para Ubuntu/Debian:
```bash
# Descargar .deb desde releases
wget https://github.com/Davidmctf/ui-openvpn-linux/releases/latest/download/ui-openvpn-linux_amd64.deb
sudo dpkg -i ui-openvpn-linux_amd64.deb
sudo apt-get install -f  # Resolver dependencias
```

#### AppImage Universal:
```bash
# Descargar AppImage
wget https://github.com/Davidmctf/ui-openvpn-linux/releases/latest/download/UI-OpenVPN-Linux.AppImage
chmod +x UI-OpenVPN-Linux.AppImage
./UI-OpenVPN-Linux.AppImage
```

### Método 3: 🔧 Instalación Manual

#### Paso 1: Instalar Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

#### Paso 2: Instalar Dependencias del Sistema

**Fedora/RHEL:**
```bash
sudo dnf install -y NetworkManager openvpn NetworkManager-openvpn NetworkManager-openvpn-gnome
```

**Ubuntu/Debian:**
```bash
sudo apt update
sudo apt install -y network-manager openvpn network-manager-openvpn network-manager-openvpn-gnome
```

**Arch Linux:**
```bash
sudo pacman -S networkmanager openvpn networkmanager-openvpn
```

#### Paso 3: Clonar y Compilar
```bash
git clone https://github.com/Davidmctf/ui-openvpn-linux.git
cd ui-openvpn-linux
cargo build --release --features web
```

#### Paso 4: Instalar
```bash
# Copiar binario
sudo cp target/release/openvpn-manager /usr/local/bin/

# Instalar aplicación de escritorio
./target/release/openvpn-manager install
```

## 🚀 Interfaz Tauri Nativa (Recomendado)

### Instalación de Tauri CLI
```bash
cargo install tauri-cli
```

### Compilar App Nativa
```bash
cd src-tauri
cargo tauri build
```

### Instalar Paquetes Generados
```bash
# Se generan automáticamente en src-tauri/target/release/bundle/
# - .deb para Ubuntu/Debian
# - .rpm para Fedora/RHEL
# - .AppImage universal

# Instalar .deb
sudo dpkg -i src-tauri/target/release/bundle/deb/ui-openvpn-linux_*.deb

# Instalar .rpm
sudo rpm -i src-tauri/target/release/bundle/rpm/ui-openvpn-linux-*.rpm
```

## 🎮 Modos de Uso

### 1. 🌌 **Aplicación Tauri Nativa** (Recomendado)
```bash
# Después de instalar el paquete, busca en el menú:
# "UI OpenVPN Linux" 

# O ejecuta desde terminal:
ui-openvpn-linux-tauri
```

### 2. 🌐 **Interfaz Web**
```bash
openvpn-manager web
# Abre automáticamente http://localhost:8081
```

### 3. 🖥️ **Aplicación de Escritorio** (Wrapper Web)
```bash
openvpn-manager desktop
# Se abre en modo aplicación (sin barra del navegador)
```

### 4. 💻 **Interfaz CLI**
```bash
openvpn-manager --help           # Ver todos los comandos
openvpn-manager list             # Listar VPNs
openvpn-manager connect <nombre> # Conectar
openvpn-manager disconnect       # Desconectar
openvpn-manager status          # Ver estado
```

## ⚙️ Configuración

### Directorio de Configuración VPN
```bash
mkdir -p ~/.connectvpn.conf
# Copiar archivos .ovpn aquí
cp mi-vpn.ovpn ~/.connectvpn.conf/
```

### Aliases Útiles (Instalados automáticamente)
```bash
vpn list                 # = openvpn-manager list
vpn connect <nombre>     # = openvpn-manager connect
vpn disconnect          # = openvpn-manager disconnect  
vpn status             # = openvpn-manager status
vpn-gui               # = openvpn-manager desktop
```

## 🔧 Solución de Problemas

### Error: "NetworkManager not available"
```bash
# Verificar que NetworkManager esté ejecutándose
sudo systemctl status NetworkManager
sudo systemctl start NetworkManager

# Agregar usuario al grupo networkmanager
sudo usermod -a -G networkmanager $USER
# Cerrar sesión y volver a iniciar
```

### Error: "Permission denied"
```bash
# Verificar permisos de NetworkManager
ls -la /etc/NetworkManager/system-connections/

# Si es necesario, ejecutar con sudo
sudo openvpn-manager connect <nombre>
```

### GUI no se abre
```bash
# Verificar dependencias gráficas (para Tauri)
# Fedora:
sudo dnf install webkit2gtk4.0-devel

# Ubuntu:
sudo apt install libwebkit2gtk-4.0-dev

# Si usa interfaz web, verificar puerto libre
netstat -tlnp | grep :8081
```

### Importación de archivos .ovpn falla
```bash
# Verificar formato del archivo
head -5 archivo.ovpn  # Debe empezar con "client" o similar

# Verificar permisos
chmod 644 archivo.ovpn

# Importar manualmente
sudo nmcli connection import type openvpn file archivo.ovpn
```

## 📊 Comparación de Métodos

| Método | Tamaño | Rendimiento | Facilidad | Nativo |
|--------|--------|-------------|-----------|--------|
| **Tauri Nativo** | ~5MB | Excelente | Alta | ✅ |
| Web + Desktop | ~8MB | Bueno | Media | ⚠️ |
| Solo CLI | ~3MB | Excelente | Baja | ✅ |
| Web Browser | ~0MB | Bueno | Alta | ❌ |

## 🔄 Actualización

### Método Automático (Tauri)
```bash
# La app Tauri verifica actualizaciones automáticamente
# y notifica cuando hay nuevas versiones disponibles
```

### Método Manual
```bash
cd ui-openvpn-linux
git pull origin main
cargo build --release --features web
sudo cp target/release/openvpn-manager /usr/local/bin/
```

## 🗑️ Desinstalación

### Desinstalar Paquete
```bash
# Fedora/RHEL
sudo rpm -e ui-openvpn-linux

# Ubuntu/Debian  
sudo apt remove ui-openvpn-linux
```

### Desinstalación Manual
```bash
sudo rm -f /usr/local/bin/openvpn-manager
rm -f ~/.local/share/applications/ui-openvpn-linux.desktop
rm -rf ~/.connectvpn.conf  # ⚠️ Elimina configuraciones VPN
```

## 🆘 Soporte

### Reportar Problemas
- **GitHub Issues**: https://github.com/Davidmctf/ui-openvpn-linux/issues
- **Documentación**: https://github.com/Davidmctf/ui-openvpn-linux/wiki

### Logs de Diagnóstico
```bash
# Ejecutar con logs detallados
RUST_LOG=debug openvpn-manager web

# Ver logs de NetworkManager
journalctl -u NetworkManager -f
```

---

**🌟 ¿Necesitas ayuda?** Abre un issue en GitHub con:
- Tu distribución Linux y versión
- Logs de error completos  
- Pasos para reproducir el problema

**🚀 ¡Que tengas un buen viaje por el ciberespacio! ✨**