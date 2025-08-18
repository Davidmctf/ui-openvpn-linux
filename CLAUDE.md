# UI OpenVPN Linux - Análisis y Documentación del Proyecto

## RESUMEN DEL PROYECTO

**Objetivo**: Crear una interfaz gráfica moderna para gestionar conexiones VPN OpenVPN, reemplazando el comando `connectvpn` existente con una solución visual intuitiva.

**Problema identificado**: El usuario debe escribir comandos para gestionar sus 2 credenciales VPN (Dynamic/David_cruz.ovpn y Howden/julian.ovpn), lo cual es tedioso y propenso a errores.

**Solución propuesta**: Aplicación desktop nativa en Rust con GTK4 que permita gestionar conexiones VPN con clicks en lugar de comandos.

## ANÁLISIS DEL COMANDO ACTUAL

### Funcionalidad del script `connectvpn` (/usr/local/bin/connectvpn):
- Lee archivos .ovpn desde `$HOME/.connectvpn.conf/`
- Detecta 2 VPNs configuradas:
  - `David_cruz.ovpn` → "Dynamic" 
  - `julian.ovpn` → "Howden"
- Muestra lista numerada para selección
- Ejecuta `sudo openvpn --config [archivo_seleccionado]`
- Manejo básico de errores y validación de entrada

### Configuración actual:
```bash
CONFIG_DIR="$HOME/.connectvpn.conf/"
VPN_FILES: David_cruz.ovpn, julian.ovpn
```

## ARQUITECTURA DEL PROYECTO

### Tecnología elegida: **Rust + GTK4**

**Justificación de la elección**:
- **Seguridad**: Rust previene vulnerabilidades de memoria, crítico para software VPN
- **Performance**: Binario nativo sin overhead de runtime
- **GTK4**: UI nativa moderna siguiendo Human Interface Guidelines
- **Integración**: Llamadas eficientes a OpenVPN y comandos del sistema  
- **Dependencias mínimas**: Solo GTK4 (preinstalado en la mayoría de distros)

### Clean Architecture - Estructura por Capas:

```
┌─────────────────────────────────────────────┐
│           UI Layer (GTK4)                   │
│  ┌─────────────┐ ┌─────────────┐           │
│  │ Components  │ │ Controllers │           │
│  │ ViewModels  │ │   Events    │           │
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

## REGLAS DE DESARROLLO OBLIGATORIAS

### Metodología y Calidad:
- ✅ **TDD (Test-Driven Development)** - Escribir tests antes que el código
- ✅ **Clean Architecture** - Separación clara de responsabilidades y capas
- ✅ **100% cobertura de tests** - Todo código debe estar probado
- ✅ **DDD (Domain-Driven Design)** - Modelado basado en el dominio del negocio

### Dependencias y Librerías:
- ✅ **Priorizar soluciones nativas** del lenguaje/framework
- ✅ **Evitar librerías externas** siempre que sea posible
- ✅ **Justificación explícita** requerida antes de instalar cualquier librería externa

### Librerías permitidas (justificadas):
1. **gtk4** (gtk-rs) - UI nativa Linux, estándar de facto
2. **tokio** - Async runtime para operaciones I/O no bloqueantes
3. **serde** - Serialización/deserialización de configuraciones
4. **thiserror** - Manejo de errores ergonómico

## ESTADO ACTUAL Y OBJETIVOS

### Estado Inicial:
- ❌ Proyecto nuevo, sin código existente
- ✅ Script bash funcional identificado y analizado
- ✅ Configuraciones VPN existentes ubicadas
- ✅ Arquitectura y tecnología definidas

### Objetivos del Proyecto:
1. **Funcionalidad Principal**:
   - Listar VPNs disponibles con nombres amigables
   - Conectar/desconectar VPNs con un click
   - Mostrar estado de conexión en tiempo real
   - Notificaciones del sistema para cambios de estado

2. **Experiencia de Usuario**:
   - Interfaz intuitiva siguiendo principios de UX
   - Feedback visual inmediato
   - Manejo de errores user-friendly
   - Integración con el sistema (system tray)

3. **Seguridad y Robustez**:
   - Manejo seguro de privilegios sudo
   - Validación de archivos de configuración
   - Logging de operaciones para debugging
   - Recuperación elegante de errores

## COMANDOS DE REINICIALIZACIÓN

### Desarrollo:
```bash
# Compilar en modo desarrollo
cargo build

# Ejecutar tests
cargo test

# Ejecutar con logging
RUST_LOG=debug cargo run

# Formato de código
cargo fmt

# Linting
cargo clippy
```

### Producción:
```bash
# Compilar optimizado
cargo build --release

# Instalar localmente
cargo install --path .
```

### Estructura del proyecto a crear:
```
ui-openvpn-linux/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── domain/          # Entities, Use Cases
│   ├── application/     # Services, DTOs
│   ├── infrastructure/  # OpenVPN, FileSystem
│   └── ui/             # GTK4 Components
├── tests/
└── resources/
```

## PRÓXIMOS PASOS

1. Crear estructura de directorios Rust
2. Configurar Cargo.toml con dependencias mínimas
3. Implementar Domain Layer (TDD)
4. Desarrollar Infrastructure Layer 
5. Crear Application Services
6. Implementar UI GTK4
7. Testing integral y optimización

---
*Generado automáticamente por Claude Code - Análisis completo del proyecto UI OpenVPN Linux*