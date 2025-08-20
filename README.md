# 🚀 UI OpenVPN Linux

<div align="center">

![Stella UA OPENVPN](./2261aaaa-bad7-4426-8cc1-f93cd6c4c067.png)

```
      🚀       🌟      🪐      
         \       |     /       
          \      |    /        
           \     |   /         
     🌟     ╔═══════╗      ⭐  
            ║  👨‍🚀   ║           
            ║ ┌─────┐║           
     ⭐     ║ │💻🔐 │║      🌟  
            ║ └─────┘║           
            ╚═══════╝           
             ╱     ╲            
            ╱ 🚀🔧 ╲           
           ╱_________╲          
          🪐          🌟        
                               
  🚀 UI OpenVPN Linux v0.2.0 🚀   
  ═══════════════════════════════════════
  Professional VPN Management in Space! ✨
```

**Navigate through secure connections like an astronaut exploring cyberspace!**

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Architecture](https://img.shields.io/badge/Architecture-Clean-blue.svg)](https://github.com/Davidmctf/ui-openvpn-linux)

</div>

Una aplicación moderna de gestión VPN desarrollada con Rust y Clean Architecture. Reemplaza scripts bash obsoletos con un gestor profesional de conexiones OpenVPN.

## 🌟 Características Espaciales

- 🚀 **Interfaz CLI profesional** con argumentos modernos y modo interactivo
- 🛰️ **Arquitectura limpia** con separación de capas y principios SOLID
- 🌌 **TDD al 100%** con tests unitarios e integración completos
- ⭐ **Performance nativa** sin overhead de runtime
- 🪐 **Interfaz GTK4** opcional para exploración visual
- 🔐 **Seguridad espacial** con manejo seguro de conexiones VPN

## 🚀 Instalación Ultra-Rápida (Un Solo Comando)

### 🌟 Instalación Automática Completa:
```bash
# Instala TODO (Rust, dependencias, compila y configura)
curl -sSL https://raw.githubusercontent.com/Davidmctf/ui-openvpn-linux/main/install.sh | bash
```

**¡Eso es todo! El script instala automáticamente:**
- ✅ Rust y Cargo
- ✅ Todas las dependencias del sistema (GTK4, OpenVPN, etc.)
- ✅ Clona y compila la aplicación
- ✅ Instala el binario globalmente
- ✅ Crea aliases útiles (vpn, vpn-list, etc.)
- ✅ Configura directorios VPN

### 🛠️ Instalación Manual (si prefieres control total):
```bash
# Clonar desde el espacio (GitHub)
git clone https://github.com/Davidmctf/ui-openvpn-linux.git
cd ui-openvpn-linux

# Compilar el cohete
cargo build --release

# Configurar base espacial VPN
mkdir -p ~/.connectvpn.conf
cp vpn_configs/*.ovpn ~/.connectvpn.conf/
```

### ¡Despegar! 🚀
```bash
# Después de instalación automática (comandos globales):
ui-openvpn --help                # Ver todos los comandos
vpn list                         # Explorar VPNs (alias corto)
vpn connect julian               # Conectar a estación julian
vpn status                       # Verificar estado de la misión
vpn disconnect                   # Desconectar de la estación
vpn-gui                          # Abrir interfaz gráfica

# Si instalaste manualmente:
./target/release/ui-openvpn --help
./target/release/ui-openvpn list
./target/release/ui-openvpn connect julian
```

## 🛸 Comandos de Navegación Espacial

### 🌌 **Exploración de VPNs:**
```bash
# Listar todas las estaciones VPN
ui-openvpn list

# Información detallada de una estación
ui-openvpn info julian

# Validar configuraciones espaciales
ui-openvpn validate
```

### 🚀 **Control de Misión:**
```bash
# Conectar a estación específica
ui-openvpn connect David_cruz

# Conectar con fuerza (override conexiones existentes)
ui-openvpn connect julian --force

# Desconectar de todas las estaciones
ui-openvpn disconnect

# Estado completo de la misión
ui-openvpn status --verbose
```

### 🎮 **Modo Centro de Control (Interactivo):**
```bash
# Lanzar centro de control
ui-openvpn

# Comandos disponibles en el centro:
vpn> help           # Ver comandos disponibles  
vpn> list           # Listar VPNs
vpn> connect julian # Conectar a julian
vpn> status         # Ver estado
vpn> disconnect     # Desconectar
vpn> quit           # Salir del centro
```

### 🎨 **Interfaz Gráfica Espacial (GTK4):**
```bash
# Instalar sistemas de navegación visual
sudo dnf install -y gtk4-devel cairo-devel glib2-devel pango-devel gdk-pixbuf2-devel

# Compilar con sistemas gráficos
cargo build --features gui --release

# Lanzar interfaz espacial
./target/release/ui-openvpn --gui
```

## 🏗️ Arquitectura de la Nave Espacial

```
┌─────────────────────────────────────────────┐
│     🎮 UI Layer (CLI/GTK4 Cockpit)         │
│  ┌─────────────┐ ┌─────────────┐           │
│  │ CLI Commands│ │ GTK4 Windows│           │  
│  │ Interactive │ │  Components │           │
│  └─────────────┘ └─────────────┘           │
└─────────────────────────────────────────────┘
┌─────────────────────────────────────────────┐
│      🚀 Application Layer (Mission Control)│
│  ┌─────────────┐ ┌─────────────┐           │
│  │ VPN Service │ │    DTOs     │           │
│  │   Mappers   │ │ Orchestr.   │           │
│  └─────────────┘ └─────────────┘           │
└─────────────────────────────────────────────┘
┌─────────────────────────────────────────────┐
│       🛰️ Domain Layer (Core Systems)       │
│  ┌─────────────┐ ┌─────────────┐           │
│  │ VPN Entity  │ │ Use Cases   │           │
│  │ Repository  │ │ Interfaces  │           │
│  └─────────────┘ └─────────────┘           │
└─────────────────────────────────────────────┘
┌─────────────────────────────────────────────┐
│    🔧 Infrastructure Layer (Engines)       │
│  ┌─────────────┐ ┌─────────────┐           │
│  │   OpenVPN   │ │ File System │           │
│  │   Process   │ │ Config Mgmt │           │
│  └─────────────┘ └─────────────┘           │
└─────────────────────────────────────────────┘
```

## 🌌 Configuración de Base Espacial

### Configurar estaciones VPN:
```bash
# Estructura de la base espacial
~/.connectvpn.conf/
├── David_cruz.ovpn     → "Dynamic Station"
├── julian.ovpn         → "Howden Outpost"  
└── *.ovpn              → Otras estaciones espaciales
```

### Agregar nuevas estaciones:
```bash
# Copiar configuraciones de misión
cp mi-nueva-estacion.ovpn ~/.connectvpn.conf/

# Verificar que la estación esté operativa
ui-openvpn validate

# Explorar nueva estación
ui-openvpn info mi-nueva-estacion
```

## 🧪 Centro de Pruebas Espaciales

```bash
# Ejecutar pruebas de sistemas
cargo test

# Ejecutar tests con reporte detallado
cargo test --verbose

# Validar arquitectura completa
cargo clippy

# Formatear código según estándares espaciales
cargo fmt
```

## 📡 Comandos de Ejemplo de Misión

### Misión típica de conexión:
```bash
# 1. Ver estaciones disponibles
ui-openvpn list

# Output:
📋 Available VPN configurations:
────────────────────────────────
  🔴 Dynamic (David_cruz)
  🔴 Howden (julian)  
  🔴 Unknown (example-vpn)

# 2. Conectar a estación Howden
ui-openvpn connect julian

# Output:
🔌 Connecting to VPN: julian
✅ Successfully connected to julian!

# 3. Verificar estado de la misión
ui-openvpn status

# Output:  
📊 VPN Connection Status:
─────────────────────────
🔴 Dynamic - DISCONNECTED
🟢 Howden - CONNECTED
🔴 Unknown - DISCONNECTED
```

## 🌟 Comparación: Script Bash vs Nave Espacial Rust

| Aspecto | Script Bash Antiguo | 🚀 UI OpenVPN Linux |
|---------|---------------------|---------------------|
| Interfaz | Menú básico numerado | CLI moderno + GUI opcional |
| Arquitectura | Script monolítico | Clean Architecture en capas |
| Seguridad | Básica | Rust memory-safe + validación |
| Testing | Sin tests | TDD al 100% cobertura |
| Performance | Lento (bash) | Nativo Rust ultra-rápido |
| Escalabilidad | Limitada | Extensible profesionalmente |
| Usabilidad | Tedioso | Comandos rápidos + interactivo |
| Mantenimiento | Difícil | Código limpio y documentado |

## 🚀 Requisitos del Sistema Espacial

### Para Operación CLI:
- Rust 1.70+
- OpenVPN instalado
- Configuraciones VPN en `~/.connectvpn.conf/`

### Para Sistemas de Navegación Visual (GTK4):
```bash
# Fedora/RHEL/CentOS
sudo dnf install -y gtk4-devel cairo-devel glib2-devel pango-devel

# Ubuntu/Debian  
sudo apt install -y libgtk-4-dev libcairo2-dev libglib2.0-dev libpango1.0-dev

# Arch Linux
sudo pacman -S gtk4 cairo glib2 pango
```

## 🛸 Contribuciones a la Misión

1. **Fork** la nave espacial
2. **Crea** una rama de características (`git checkout -b feature/AmazingSpaceFeature`)
3. **Commit** tus mejoras (`git commit -m 'Add some AmazingSpaceFeature'`)
4. **Push** a la rama (`git push origin feature/AmazingSpaceFeature`)
5. **Abre** un Pull Request a la estación principal

## 📄 Licencia Espacial

MIT License - Libertad total para explorar el cosmos digital.

## 🌌 Agradecimientos

- **Astronauta Logo**: Inspirado en la exploración espacial y la navegación segura por el ciberespacio
- **Clean Architecture**: Para mantener el código organizado como una estación espacial
- **Rust Language**: Por la velocidad y seguridad de un cohete espacial
- **OpenVPN**: Por las conexiones seguras a través del universo digital

---

**🚀 ¡Que tengas un buen viaje por el ciberespacio! ✨**

*Desarrollado con amor espacial por [@Davidmctf](https://github.com/Davidmctf)*
