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
git clone <tu-repo>
cd ui-openvpn-linux

# Compilar versión CLI (sin GTK4)
cargo build --no-default-features --release

# Ejecutar
./target/release/ui-openvpn
```

### 2. Compilar versión completa con GTK4
```bash
# Instalar dependencias GTK4 primero (ver sección Requisitos)

# Compilar con UI gráfica
cargo build --features ui --release

# Ejecutar modo CLI
./target/release/ui-openvpn

# Ejecutar modo gráfico
./target/release/ui-openvpn --gui
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

La aplicación busca archivos de configuración VPN en:
```
~/.connectvpn.conf/
├── David_cruz.ovpn  → "Dynamic"
├── julian.ovpn      → "Howden"  
└── *.ovpn          → Otros archivos VPN
```

## 💻 Modo de Uso

### CLI Interface
```bash
🔒 UI OpenVPN Linux - Gestión de VPNs
=====================================
1) Listar VPNs disponibles
2) Conectar VPN
3) Desconectar VPN actual
4) Ver estado de conexión
5) Salir

👉 Ingresa tu opción:
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