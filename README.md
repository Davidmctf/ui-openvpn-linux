# 🚀 UI OpenVPN Linux - Navegador Espacial de Conexiones VPN

<div align="center">

<img src="./assets/2261aaaa-bad7-4426-8cc1-f93cd6c4c067.png" alt="Stella UA OPENVPN" width="300">

**Navigate through secure connections like an astronaut exploring cyberspace!**

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Architecture](https://img.shields.io/badge/Architecture-Clean-blue.svg)](https://github.com/Davidmctf/ui-openvpn-linux)

</div>

Una aplicación moderna de gestión VPN desarrollada con Rust y Clean Architecture. Reemplaza scripts bash obsoletos con un gestor profesional de conexiones OpenVPN con interfaz espacial. ✨

## 🌟 Características Principales

### 🎮 **4 Interfaces Diferentes:**
1. **🌌 Aplicación Tauri Nativa** - App de escritorio ultra-liviana (3-5MB)
2. **🌐 Interfaz Web Espacial** - Tema espacial con drag & drop para archivos .ovpn
3. **🖥️ Aplicación de Escritorio** - Wrapper que abre como app nativa  
4. **💻 CLI Profesional** - Comandos modernos con modo interactivo

### ⚡ **Ultra Rápido & Seguro:**
- ✅ **Binarios nativos Rust** - Rendimiento máximo, memoria mínima
- ✅ **Integración NetworkManager** - Usa la API nativa del sistema
- ✅ **Tema espacial completo** - Diseño inspirado en navegación cósmica
- ✅ **Drag & Drop VPN files** - Arrastra archivos .ovpn directamente
- ✅ **Paquetes automáticos** - .deb, .rpm, AppImage generados automáticamente

## 🚀 Instalación Ultra-Rápida (Un Solo Comando)

```bash
# Instala TODO automáticamente (Rust, dependencias, compila, configura)
curl -sSL https://raw.githubusercontent.com/Davidmctf/ui-openvpn-linux/main/install.sh | bash
```

**¡Eso es todo!** 🎉 Después busca **"UI OpenVPN Linux"** en tu menú de aplicaciones.

## 🎯 Uso Rápido

### 🌌 **Aplicación Nativa (Recomendado):**
```bash
# Buscar en menú: "UI OpenVPN Linux"
# O ejecutar desde terminal después de la instalación
ui-openvpn-linux
```

### 🌐 **Interfaz Web Espacial:**
```bash
openvpn-manager web    # http://localhost:8081
```

### 🖥️ **Como App de Escritorio:**
```bash
openvpn-manager desktop    # Se abre como aplicación nativa
```

### 💻 **Comandos CLI:**
```bash
openvpn-manager list              # 📋 Ver todas las estaciones VPN
openvpn-manager connect julian    # 🚀 Conectar a estación "julian"  
openvpn-manager status            # 📊 Ver estado de conexión
openvpn-manager disconnect        # 🛑 Desconectar de todas las estaciones
```

## 📋 Métodos de Instalación

| Método | Facilidad | Tamaño | Recomendado |
|--------|-----------|--------|-------------|
| **Script automático** | ⭐⭐⭐⭐⭐ | ~8MB | ✅ Sí |
| **Paquetes .deb/.rpm** | ⭐⭐⭐⭐ | ~5MB | ✅ Próximamente |
| **AppImage** | ⭐⭐⭐ | ~8MB | ⚠️ Próximamente |
| **Compilación manual** | ⭐⭐ | ~5MB | 🔧 Desarrolladores |

Ver **[INSTALL.md](./INSTALL.md)** para instrucciones detalladas.

## 🎨 Capturas de Pantalla

### 🌌 Interfaz Web Espacial
- **Tema cósmico** con gradientes espaciales
- **Drag & Drop** para archivos .ovpn
- **Estado en tiempo real** con iconos 🚀🔴🟢
- **Botones espaciales** con efectos glow

### 🖥️ Aplicación Tauri Nativa
- **Ultra-liviana** - Solo 3-5MB vs 120MB de Electron
- **File picker nativo** para importar VPNs
- **Rendimiento nativo** usando WebKit del sistema
- **Paquetes automáticos** .deb/.rpm/.AppImage

## 🛰️ Comandos Disponibles

### CLI Principal:
```bash
openvpn-manager <SUBCOMMAND>

SUBCOMMANDS:
    list        📋 Listar todas las estaciones VPN disponibles
    connect     🚀 Conectar a una estación específica  
    disconnect  🛑 Desconectar de todas las estaciones
    status      📊 Mostrar estado detallado de conexiones
    remove      🗑️ Eliminar una configuración VPN
    import      📥 Importar archivo .ovpn
    web         🌐 Lanzar interfaz web espacial  
    desktop     🖥️ Lanzar como aplicación de escritorio
    install     ⚙️ Crear entrada en menú de aplicaciones
```

### Aliases Automáticos:
```bash
vpn list                 # = openvpn-manager list
vpn connect julian       # = openvpn-manager connect julian
vpn disconnect          # = openvpn-manager disconnect  
vpn status             # = openvpn-manager status
vpn-web               # = openvpn-manager web
```

## 🧬 Arquitectura del Proyecto

```
┌─────────────────────────────────────────────┐
│     🎮 UI Layer (Multi-Interface)          │
│  ┌─────────────┐ ┌─────────────┐           │
│  │ CLI Modern  │ │ Web Spatial │           │  
│  │ Interface   │ │Desktop Wrap │           │
│  └─────────────┘ └─────────────┘           │
└─────────────────────────────────────────────┘
┌─────────────────────────────────────────────┐
│      🚀 Application Layer (Services)       │
│  ┌─────────────┐ ┌─────────────┐           │
│  │ VPN Service │ │    DTOs     │           │
│  │   Mappers   │ │ Web Server  │           │
│  └─────────────┘ └─────────────┘           │
└─────────────────────────────────────────────┘
┌─────────────────────────────────────────────┐
│       🛰️ Domain Layer (Core Logic)        │
│  ┌─────────────┐ ┌─────────────┐           │
│  │ VPN Entity  │ │ Use Cases   │           │
│  │ Repository  │ │ Interfaces  │           │
│  └─────────────┘ └─────────────┘           │
└─────────────────────────────────────────────┘
┌─────────────────────────────────────────────┐
│    🔧 Infrastructure (NetworkManager)     │
│  ┌─────────────┐ ┌─────────────┐           │
│  │   nmcli     │ │ File System │           │
│  │   Process   │ │ Config Mgmt │           │
│  └─────────────┘ └─────────────┘           │
└─────────────────────────────────────────────┘
```

## 🌌 Configuración de Base Espacial

### Directorio VPN:
```bash
~/.connectvpn.conf/
├── David_cruz.ovpn     → "Dynamic Station"
├── julian.ovpn         → "Howden Outpost"  
└── *.ovpn              → Otras estaciones espaciales
```

### Agregar nuevas estaciones:
```bash
# Método 1: Drag & Drop en interfaz web
# Arrastra archivo .ovpn a la zona de upload

# Método 2: Comando CLI
openvpn-manager import mi-nueva-estacion.ovpn

# Método 3: Copia manual
cp mi-nueva-estacion.ovpn ~/.connectvpn.conf/
```

## 🚀 Instalación

### 🚀 Instalación Automática (Recomendada):
```bash
git clone https://github.com/Davidmctf/ui-openvpn-linux.git
cd ui-openvpn-linux
./install-clean.sh
```

### 🗑️ Desinstalar Completamente:
```bash
./uninstall.sh
```
**Elimina todo:** binarios, desktop entries, aliases, configuraciones, cache

### Instalación Manual:

#### Prerequisitos:
```bash
# Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Dependencias sistema (Fedora/RHEL)
sudo dnf install -y NetworkManager openvpn NetworkManager-openvpn

# Dependencias sistema (Ubuntu/Debian) 
sudo apt install -y network-manager openvpn network-manager-openvpn
```

#### Compilar e Instalar:
```bash
git clone https://github.com/Davidmctf/ui-openvpn-linux.git
cd ui-openvpn-linux

# Solo CLI
cargo build --release

# CLI + Web interface
cargo build --release --features web

# Instalar globalmente
sudo cp target/release/openvpn-manager /usr/local/bin/
```

## 🎨 Logo y Recursos

El logo de la aplicación se encuentra en `assets/openvpn-manager-logo.png` y se instala automáticamente con el script de instalación, apareciendo en el menú de aplicaciones con esquinas redondeadas.

## 🔧 Desarrollo

### Testing y Development:
```bash
# Ejecutar CLI directamente  
cargo run -- list
cargo run -- connect mi-vpn

# Ejecutar web interface
cargo run --features web -- web

# Tests y linting
cargo test                    # Unit tests
cargo clippy                  # Linting  
cargo fmt                     # Format code
```

## 📦 Distribución

### Script de Instalación:
```bash
# El script install.sh maneja todo automáticamente:
# - Compila con features web
# - Instala en /usr/local/bin
# - Crea desktop entry
# - Verifica dependencias
```

### RPM Manual:
```bash
cargo install cargo-generate-rpm
cargo generate-rpm
```

## 🌟 Comparación: Script Bash vs Nave Espacial Rust

| Aspecto | Script Bash Antiguo | 🚀 UI OpenVPN Linux |
|---------|---------------------|---------------------|
| Interfaz | Menú básico numerado | 4 interfaces modernas + tema espacial |
| Arquitectura | Script monolítico | Clean Architecture + Tauri nativa |
| Seguridad | Básica | Rust memory-safe + sandboxing |
| Testing | Sin tests | TDD con cobertura completa |
| Performance | Lento (bash) | Ultra-rápido (Rust nativo) |
| Paquetes | Manual | .deb/.rpm/.AppImage automáticos |
| Usabilidad | Tedioso | Drag & Drop + un click |
| Tamaño | ~1KB script | 3-8MB (vs 120MB Electron) |

## 📄 Documentación

- **[📋 Guía de Instalación](./INSTALL.md)** - Instalación detallada paso a paso
- **[🚀 Funcionalidades](./FEATURES.md)** - Lista completa de características  
- **[🧠 Arquitectura](./CLAUDE.md)** - Documentación técnica interna
- **[🔧 Configuración](./SETUP.md)** - Configuración avanzada

## 🛸 Contribuir a la Misión

1. **Fork** la nave espacial
2. **Crea** una rama de características (`git checkout -b feature/AmazingSpaceFeature`)
3. **Commit** tus mejoras (`git commit -m 'Add some AmazingSpaceFeature'`)
4. **Push** a la rama (`git push origin feature/AmazingSpaceFeature`)
5. **Abre** un Pull Request a la estación principal

### 🧪 Testing:
```bash
cargo test --all-features    # Ejecutar todos los tests
cargo clippy                 # Linting code
cargo fmt                    # Format code  
```

## 📄 Licencia Espacial

MIT License - Libertad total para explorar el cosmos digital.

## 🌌 Créditos y Agradecimientos

- **🚀 Astronauta Logo**: Inspirado en la exploración espacial y navegación segura
- **🏗️ Clean Architecture**: Para mantener el código organizado como una estación espacial  
- **⚡ Rust Language**: Por la velocidad y seguridad de un cohete espacial
- **🔐 OpenVPN**: Por las conexiones seguras a través del universo digital
- **🎨 Tauri Framework**: Por hacer posible apps nativas ultra-livianas

## 📊 Estadísticas del Proyecto

- **Lenguajes**: 95% Rust, 5% JavaScript/HTML/CSS
- **Líneas de código**: ~2,000 líneas principales
- **Dependencias**: <50 crates cuidadosamente seleccionadas  
- **Tamaño binario**: 3-8MB según características compiladas
- **Tiempo compilación**: 2-5 minutos en hardware moderno
- **Soporte**: 6+ distribuciones Linux principales

---

<div align="center">

**🚀 ¡Que tengas un buen viaje por el ciberespacio! ✨**

*Desarrollado con amor espacial por [@Davidmctf](https://github.com/Davidmctf)*

[⭐ Dar estrella](https://github.com/Davidmctf/ui-openvpn-linux/stargazers) • [🐛 Reportar bug](https://github.com/Davidmctf/ui-openvpn-linux/issues) • [💡 Sugerir función](https://github.com/Davidmctf/ui-openvpn-linux/issues/new)

</div>