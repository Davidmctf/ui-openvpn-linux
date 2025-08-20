# 🚀 Guía de Instalación - UI OpenVPN Linux

## 🌟 Instalación Automática (Recomendada)

### Un Solo Comando para Instalar Todo:

```bash
curl -sSL https://raw.githubusercontent.com/Davidmctf/ui-openvpn-linux/main/install.sh | bash
```

**¿Qué hace este comando?**
1. ✅ **Detecta tu distribución** (Fedora, Ubuntu, Arch, openSUSE)
2. ✅ **Instala Rust y Cargo** automáticamente
3. ✅ **Instala dependencias del sistema**:
   - GTK4 para interfaz gráfica
   - OpenSSL para conexiones seguras
   - OpenVPN para gestión VPN
   - Herramientas de compilación
4. ✅ **Clona el repositorio** desde GitHub
5. ✅ **Compila la aplicación** (CLI + GUI si es posible)
6. ✅ **Instala globalmente** en ~/.local/bin
7. ✅ **Crea aliases útiles**: vpn, vpn-list, vpn-status, vpn-gui
8. ✅ **Configura directorio VPN** en ~/.connectvpn.conf/

### Después de la Instalación:

```bash
# Reiniciar terminal o ejecutar:
source ~/.bashrc

# Verificar instalación:
vpn --help
vpn --version

# Usar la aplicación:
vpn list                    # Listar VPNs
vpn connect <nombre>        # Conectar
vpn status                  # Ver estado
vpn-gui                     # Interfaz gráfica
```

---

## 🛠️ Instalación Manual

### Prerequisitos por Distribución:

#### **Fedora / RHEL / CentOS:**
```bash
sudo dnf install -y gcc openssl-devel pkg-config gtk4-devel cairo-devel glib2-devel pango-devel gdk-pixbuf2-devel openvpn git curl
```

#### **Ubuntu / Debian:**
```bash
sudo apt update
sudo apt install -y build-essential libssl-dev pkg-config libgtk-4-dev libcairo2-dev libglib2.0-dev libpango1.0-dev libgdk-pixbuf-2.0-dev openvpn git curl
```

#### **Arch Linux:**
```bash
sudo pacman -S base-devel openssl pkg-config gtk4 cairo glib2 pango gdk-pixbuf2 openvpn git curl
```

#### **openSUSE:**
```bash
sudo zypper install gcc openssl-devel pkg-config gtk4-devel cairo-devel glib2-devel pango-devel gdk-pixbuf-devel openvpn git curl
```

### Instalar Rust:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

### Clonar y Compilar:
```bash
git clone https://github.com/Davidmctf/ui-openvpn-linux.git
cd ui-openvpn-linux

# Solo CLI (rápido):
cargo build --release

# Con GUI (requiere GTK4):
cargo build --features gui --release
```

### Instalación Global:
```bash
# Copiar binario:
mkdir -p ~/.local/bin
cp target/release/ui-openvpn ~/.local/bin/
chmod +x ~/.local/bin/ui-openvpn

# Agregar al PATH:
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

---

## 🔧 Configuración Post-Instalación

### Configurar Directorio VPN:
```bash
mkdir -p ~/.connectvpn.conf
# Copia tus archivos .ovpn aquí
```

### Verificar Instalación:
```bash
ui-openvpn --version
ui-openvpn list
openvpn --version  # Verificar OpenVPN
```

### Crear Alias Personalizados:
```bash
echo 'alias vpn="ui-openvpn"' >> ~/.bashrc
echo 'alias vpn-connect="ui-openvpn connect"' >> ~/.bashrc
echo 'alias vpn-status="ui-openvpn status"' >> ~/.bashrc
source ~/.bashrc
```

---

## 🚑 Resolución de Problemas

### Error: "GTK4 not found"
```bash
# Instalar GTK4 según tu distribución (ver secciones arriba)
# Luego recompilar:
cargo build --features gui --release
```

### Error: "Rust not found"
```bash
# Asegurarse de que Rust esté en el PATH:
source ~/.cargo/env
# O reinstalar:
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Error: "Permission denied" con sudo
```bash
# Configurar sudoers para OpenVPN (opcional):
echo 'your_username ALL=(ALL) NOPASSWD: /usr/sbin/openvpn' | sudo tee /etc/sudoers.d/openvpn
```

### VPNs no aparecen en la lista:
```bash
# Verificar directorio:
ls -la ~/.connectvpn.conf/
# Debe contener archivos .ovpn

# Verificar permisos:
chmod 600 ~/.connectvpn.conf/*.ovpn
```

### Error de compilación por dependencias:
```bash
# Limpiar caché y rebuild:
cargo clean
cargo build --release

# Actualizar Rust:
rustup update
```

---

## 📋 Lista de Verificación de Instalación

- [ ] ✅ Rust y Cargo instalados (`cargo --version`)
- [ ] ✅ OpenVPN instalado (`openvpn --version`)
- [ ] ✅ Dependencias de compilación instaladas
- [ ] ✅ GTK4 instalado para GUI (opcional)
- [ ] ✅ Repositorio clonado y compilado
- [ ] ✅ Binario instalado globalmente
- [ ] ✅ PATH actualizado y aliases creados
- [ ] ✅ Directorio ~/.connectvpn.conf/ creado
- [ ] ✅ Archivos .ovpn copiados
- [ ] ✅ Comando `vpn --help` funciona
- [ ] ✅ `vpn list` muestra VPNs disponibles

---

## 🆘 Soporte

Si tienes problemas:

1. **Revisa los logs de compilación** para errores específicos
2. **Verifica las dependencias** según tu distribución
3. **Prueba la instalación manual** paso a paso
4. **Consulta el README.md** para comandos de uso
5. **Crea un issue** en GitHub con detalles del error

---

**🚀 ¡Disfruta navegando por el ciberespacio de forma segura! ✨**