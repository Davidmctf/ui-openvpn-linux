# 🔧 Setup Guide - UI OpenVPN Linux

## Configuración Inicial

### 1. Preparar directorio de configuraciones VPN

```bash
# Crear directorio si no existe
mkdir -p ~/.connectvpn.conf

# Dar permisos apropiados
chmod 700 ~/.connectvpn.conf
```

### 2. Agregar tus archivos VPN

#### Opción A: Copiar archivos de ejemplo
```bash
# Desde el directorio del proyecto
cp vpn_configs/*.ovpn ~/.connectvpn.conf/
```

#### Opción B: Agregar tus propias configuraciones
```bash
# Copiar tus archivos .ovpn
cp /path/to/your/vpn-config.ovpn ~/.connectvpn.conf/

# Asegurar permisos correctos
chmod 600 ~/.connectvpn.conf/*.ovpn
```

### 3. Estructura recomendada

```
~/.connectvpn.conf/
├── mi-vpn-trabajo.ovpn      # VPN del trabajo
├── mi-vpn-personal.ovpn     # VPN personal
├── servidor-casa.ovpn       # VPN de casa
├── ca.crt                   # Certificado CA (si es necesario)
├── client.crt               # Certificado cliente (si es necesario)
└── client.key               # Clave privada (si es necesario)
```

## Configuración de archivos .ovpn

### Ejemplo de archivo .ovpn completo:

```
client
dev tun
proto udp
remote mi-servidor-vpn.com 1194
resolv-retry infinite
nobind
persist-key
persist-tun

# Certificados embebidos (recomendado)
<ca>
-----BEGIN CERTIFICATE-----
[contenido del certificado CA]
-----END CERTIFICATE-----
</ca>

<cert>
-----BEGIN CERTIFICATE-----
[contenido del certificado cliente]
-----END CERTIFICATE-----
</cert>

<key>
-----BEGIN PRIVATE KEY-----
[contenido de la clave privada]
-----END PRIVATE KEY-----
</key>

# Configuración adicional
cipher AES-256-CBC
auth SHA256
comp-lzo
verb 3
```

### Archivos separados (alternativa):

```
client
dev tun
proto udp
remote mi-servidor-vpn.com 1194
resolv-retry infinite
nobind
persist-key
persist-tun
ca ca.crt
cert client.crt
key client.key
cipher AES-256-CBC
auth SHA256
comp-lzo
verb 3
```

## Personalización

### Nombres amigables automáticos:

El sistema mapea automáticamente ciertos nombres de archivo:
- `David_cruz.ovpn` → se muestra como "Dynamic"
- `julian.ovpn` → se muestra como "Howden"

### Para agregar tus propios nombres:

Edita el archivo `src/infrastructure/repositories.rs` en la función `map_display_name`:

```rust
fn map_display_name(filename: &str) -> String {
    match filename {
        "David_cruz.ovpn" => "Dynamic".to_string(),
        "julian.ovpn" => "Howden".to_string(),
        "mi-vpn-trabajo.ovpn" => "Oficina Central".to_string(),
        "casa.ovpn" => "VPN Casa".to_string(),
        _ => "VPN Personalizada".to_string(),
    }
}
```

## Seguridad

### ⚠️ Importante - Archivos sensibles:

**NUNCA subas a repositorios públicos:**
- `*.key` - Claves privadas
- `*.crt` - Certificados (pueden contener info sensible)
- `*.pem` - Archivos de certificados/claves

**Seguro para repos públicos:**
- `*.ovpn` - Solo archivos de configuración (sin certificados embebidos)

### Permisos recomendados:

```bash
# Directorio de configuración
chmod 700 ~/.connectvpn.conf

# Archivos de configuración
chmod 600 ~/.connectvpn.conf/*.ovpn

# Certificados y claves (si existen por separado)
chmod 600 ~/.connectvpn.conf/*.key
chmod 600 ~/.connectvpn.conf/*.crt
```

## Troubleshooting

### Error: "No se encontraron archivos .ovpn"
```bash
# Verificar que existan archivos
ls -la ~/.connectvpn.conf/

# Verificar permisos
ls -la ~/.connectvpn.conf/*.ovpn
```

### Error de conexión OpenVPN
```bash
# Probar manualmente la conexión
sudo openvpn --config ~/.connectvpn.conf/tu-archivo.ovpn

# Verificar logs
journalctl -u openvpn
```

### Error de permisos sudo
```bash
# El usuario debe tener permisos sudo para openvpn
sudo -l | grep openvpn
```

## Instalación de dependencias del sistema

### OpenVPN
```bash
# Fedora/RHEL/CentOS
sudo dnf install -y openvpn

# Ubuntu/Debian
sudo apt install -y openvpn

# Arch Linux
sudo pacman -S openvpn
```

### Para interfaz gráfica GTK4 (opcional)
```bash
# Fedora/RHEL/CentOS
sudo dnf install -y gtk4-devel cairo-devel glib2-devel pango-devel gdk-pixbuf2-devel

# Ubuntu/Debian
sudo apt install -y libgtk-4-dev libcairo2-dev libglib2.0-dev libpango1.0-dev libgdk-pixbuf-2.0-dev

# Arch Linux
sudo pacman -S gtk4 cairo glib2 pango gdk-pixbuf2
```