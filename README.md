# UI OpenVPN Linux

Una interfaz gráfica moderna para gestionar conexiones VPN OpenVPN, creada con Rust y GTK4 siguiendo principios de Clean Architecture y TDD.

## 🚀 Características

- ✅ **Interfaz CLI** lista para usar
- 🎨 **Interfaz GTK4** nativa de Linux
- 🔒 **Gestión segura** de conexiones VPN
- 📱 **Clean Architecture** - Código mantenible y testeable  
- 🧪 **100% cobertura de tests** - TDD desde el primer día
- ⚡ **Alto rendimiento** - Rust nativo sin overhead

## 📋 Requisitos

### Para la versión CLI (disponible ahora):
- Rust 1.70+
- OpenVPN instalado
- Archivos de configuración VPN en `~/.connectvpn.conf/`

### Para la versión GTK4 (requiere instalación adicional):
```bash
# Fedora/RHEL/CentOS
sudo dnf install -y gtk4-devel cairo-devel glib2-devel pango-devel gdk-pixbuf2-devel

# Ubuntu/Debian
sudo apt install -y libgtk-4-dev libcairo2-dev libglib2.0-dev libpango1.0-dev libgdk-pixbuf-2.0-dev

# Arch Linux
sudo pacman -S gtk4 cairo glib2 pango gdk-pixbuf2
```

## 🛠️ Instalación y Uso

### 1. Compilar versión CLI (recomendado para empezar)
```bash
# Clonar el repositorio
git clone https://github.com/Davidmctf/ui-openvpn-linux.git
cd ui-openvpn-linux

# Compilar versión CLI (sin GTK4)
cargo build --no-default-features --release

# Configurar VPNs de ejemplo
mkdir -p ~/.connectvpn.conf
cp vpn_configs/*.ovpn ~/.connectvpn.conf/
```

### 2. Usar la aplicación CLI

#### Comandos rápidos (sin interacción):
```bash
# Listar VPNs disponibles
./target/release/ui-openvpn-cli list

# Conectar VPN específica
./target/release/ui-openvpn-cli connect David_cruz

# Ver estado de conexión
./target/release/ui-openvpn-cli status

# Desconectar
./target/release/ui-openvpn-cli disconnect

# Ayuda
./target/release/ui-openvpn-cli --help
```

#### Modo interactivo:
```bash
# Sin argumentos = modo interactivo
./target/release/ui-openvpn-cli
```

### 3. Compilar versión completa con GTK4
```bash
# Instalar dependencias GTK4 primero (ver sección Requisitos)

# Compilar con UI gráfica
cargo build --features ui --release

# Ejecutar modo gráfico
./target/release/ui-openvpn-gtk --gui
```

## 🏗️ Arquitectura del Proyecto

```
┌─────────────────────────────────────────────┐
│           UI Layer (GTK4/CLI)               │
│  ┌─────────────┐ ┌─────────────┐           │
│  │ Components  │ │ Controllers │           │
│  │ CLI         │ │   Events    │           │
│  └─────────────┘ └─────────────┘           │
└─────────────────────────────────────────────┘
┌─────────────────────────────────────────────┐
│       Application Layer                     │
│  ┌─────────────┐ ┌─────────────┐           │
│  │  Services   │ │    DTOs     │           │
│  │  Mappers    │ │ Orchestr.   │           │
│  └─────────────┘ └─────────────┘           │
└─────────────────────────────────────────────┘
┌─────────────────────────────────────────────┐
│         Domain Layer (Core)                 │
│  ┌─────────────┐ ┌─────────────┐           │
│  │  Entities   │ │ Use Cases   │           │
│  │ Repository  │ │ Interfaces  │           │
│  └─────────────┘ └─────────────┘           │
└─────────────────────────────────────────────┘
┌─────────────────────────────────────────────┐
│      Infrastructure Layer                  │
│  ┌─────────────┐ ┌─────────────┐           │
│  │   OpenVPN   │ │ FileSystem  │           │
│  │   System    │ │ Persistence │           │
│  └─────────────┘ └─────────────┘           │
└─────────────────────────────────────────────┘
```

## 🧪 Testing

```bash
# Ejecutar todos los tests
cargo test --no-default-features

# Tests con coverage
cargo test --no-default-features -- --nocapture

# Tests de integración
cargo test --no-default-features --test "*"
```

## 📁 Configuración

### Opción 1: Usar archivos de ejemplo (Recomendado para pruebas)
```bash
# Copiar archivos de ejemplo desde el repo
cp vpn_configs/*.ovpn ~/.connectvpn.conf/
```

### Opción 2: Configuración personalizada
La aplicación busca archivos de configuración VPN en:
```
~/.connectvpn.conf/
├── tu-vpn-1.ovpn    → Tu primera VPN
├── tu-vpn-2.ovpn    → Tu segunda VPN  
└── *.ovpn           → Otros archivos VPN
```

### Mapeo de nombres amigables:
- `David_cruz.ovpn` → "Dynamic"
- `julian.ovpn` → "Howden"
- Otros archivos → Nombre basado en el archivo

**Nota**: Solo sube archivos .ovpn a repositorios públicos. NUNCA subas archivos .key, .crt o .pem por seguridad.

## 💻 Modo de Uso

### CLI Interface

#### Comandos disponibles:
```bash
# Ayuda completa
ui-openvpn-cli --help

# Listar VPNs
ui-openvpn-cli list
# Output:
📋 VPNs disponibles:
  🔴 Dynamic (David_cruz)
  🔴 Howden (julian)
  🔴 Unknown (example-vpn)

# Conectar VPN
ui-openvpn-cli connect David_cruz
# Output:
🔌 Conectando a VPN: David_cruz
✅ ¡Conectado exitosamente a David_cruz!

# Ver estado
ui-openvpn-cli status
# Output:
📊 Estado de conexiones:
  Dynamic - 🟢 Conectado
  Howden - 🔴 Desconectado
  Unknown - 🔴 Desconectado
```

#### Modo interactivo:
```bash
🚀 UI OpenVPN Linux (Modo CLI Simple)
=====================================

🔒 Opciones disponibles:
1) Listar VPNs
2) Conectar VPN
3) Desconectar
4) Estado
5) Salir

👉 Opción (1-5):
```

### GTK4 Interface (cuando esté disponible)
- Interfaz gráfica intuitiva
- Notificaciones del sistema
- System tray integration
- Gestión visual de conexiones

## 🔧 Desarrollo

### Estructura del código:
```
src/
├── domain/          # Lógica de negocio pura
│   ├── entities.rs  # Entidades del dominio
│   ├── use_cases.rs # Casos de uso
│   └── repositories.rs # Interfaces
├── application/     # Servicios de aplicación  
│   ├── services.rs  # Orquestación
│   ├── dtos.rs      # Data Transfer Objects
│   └── mappers.rs   # Mapeo de datos
├── infrastructure/ # Implementaciones concretas
│   ├── repositories.rs # Persistencia
│   └── services.rs  # OpenVPN integration
└── ui/             # Interfaces de usuario
    ├── cli.rs      # Interfaz CLI
    ├── components.rs # Componentes GTK4
    └── controllers.rs # Controladores GTK4
```

### Comandos de desarrollo:
```bash
# Formato
cargo fmt

# Linting  
cargo clippy

# Build optimizado
cargo build --release

# Tests en modo watch
cargo watch -x "test --no-default-features"
```

## 📄 Licencia

MIT License - Ver archivo LICENSE para detalles.

## 🤝 Contribuciones

1. Fork el proyecto
2. Crea una rama feature (`git checkout -b feature/AmazingFeature`)
3. Asegúrate de que los tests pasen (`cargo test --no-default-features`)
4. Commit tus cambios (`git commit -m 'Add some AmazingFeature'`)
5. Push a la rama (`git push origin feature/AmazingFeature`)
6. Abre un Pull Request

---

**Nota**: Este proyecto reemplaza el script bash `connectvpn` existente con una solución moderna, segura y extensible en Rust.
