# OpenVPN Manager - GUI Development Guidelines

## Project Overview
Simple OpenVPN Manager for Linux with both CLI and GUI interfaces using NetworkManager integration.

## GUI Development Rules & Standards

### Framework Choice: GTK4
- **Primary GUI Framework**: GTK4-rs (gtk4 crate v0.9+)
- **Rationale**: Native Linux integration, established ecosystem, NetworkManager compatibility
- **Alternative Considered**: egui (pure Rust, simpler) - but GTK4 chosen for better system integration

### Architecture Requirements

#### 1. Window Management
- **MUST** use `Application::builder()` pattern for proper GTK4 initialization
- **MUST** use `window.present()` instead of `window.show()` for reliable window display
- **MUST** handle window lifecycle properly with proper cleanup
- **REQUIRED** application ID format: `com.osmiodev.openvpn-manager`

#### 2. UI Layout Standards
- **Main Window**: 500x400 minimum size
- **Language**: Spanish UI (usuario target is Spanish-speaking)
- **Icons**: Use Unicode emojis for buttons (🔄, 🔌, ❌, 📥, etc.)
- **Status Display**: Real-time connection status (🟢 Conectado / 🔴 Desconectado)

#### 3. Functional Requirements
- **VPN List Management**: Display available NetworkManager VPN connections
- **Real-time Updates**: Auto-refresh connection status
- **File Import**: Support .ovpn file import via file dialog
- **Connection Control**: Connect/disconnect VPN connections
- **Error Handling**: Graceful handling of NetworkManager errors

### Code Organization

#### File Structure
```
src/
├── main.rs           # CLI + GUI launcher
├── ui/
│   ├── mod.rs        # UI module exports  
│   └── gtk.rs        # GTK4 implementation
└── ...
```

#### Module Dependencies
- `gtk4` = { version = "0.9", optional = true }
- Feature flag: `gui = ["gtk4"]`
- Conditional compilation: `#[cfg(feature = "gui")]`

### Implementation Standards

#### 1. GTK4 Best Practices
```rust
// CORRECT initialization pattern
pub fn run_gui() {
    let app = Application::builder()
        .application_id("com.osmiodev.openvpn-manager")
        .build();
    app.connect_activate(build_ui);
    app.run();
}

// CORRECT window creation
fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("🔐 OpenVPN Manager")
        .default_width(500)
        .default_height(400)
        .build();
    
    // ... UI setup ...
    
    window.present(); // NOT .show()
}
```

#### 2. State Management
- Use `Rc<RefCell<T>>` for shared state between closures
- Clone references properly for event handlers
- Maintain selected VPN state for connect button activation

#### 3. NetworkManager Integration
- Use `nmcli` command-line interface via `std::process::Command`
- Parse output for VPN connection listing and status
- Handle authentication and connection errors gracefully

#### 4. File Dialog Standards
- Use `FileChooserDialog` for compatibility (not `FileDialog`)
- Filter for `*.ovpn` files only
- Proper error handling for file import operations

### Testing Requirements

#### System Dependencies
- **Linux Environment**: GTK4 libraries installed
- **Display Server**: X11 or Wayland support required
- **NetworkManager**: Active NetworkManager service
- **Permissions**: User must be able to modify NetworkManager connections

#### Build Commands
```bash
# Debug build with GUI
cargo build --features gui

# Release build with GUI  
cargo build --release --features gui

# Run GUI
./target/release/openvpn-manager gui
```

### Troubleshooting Guide

#### Common Issues
1. **"GTK not initialized"**: Use Application pattern, never manual init
2. **Window not appearing**: Use `window.present()`, check DISPLAY variable
3. **File dialog errors**: Use FileChooserDialog instead of FileDialog
4. **NetworkManager errors**: Check service status and user permissions

#### Environment Requirements
- Linux desktop environment with GTK4 support
- Active X11 or Wayland session
- NetworkManager service running
- Proper user permissions for network configuration

### Development Workflow

1. **Feature Development**: Always test GUI in actual desktop environment
2. **Code Review**: Ensure proper GTK4 patterns and error handling
3. **Testing**: Test with real VPN configurations and various system states
4. **Documentation**: Update this file when changing GUI architecture

### Future Improvements

#### Potential Enhancements
- Keyboard shortcuts for common actions
- System tray integration for background operation
- Configuration file management UI
- Connection history and favorites
- Dark theme support

#### Alternative Frameworks Considered
- **egui**: Simpler pure Rust, but less native integration
- **Tauri**: Web technologies, but overkill for this use case
- **Iced**: Good architecture, but less mature ecosystem

---

Last Updated: 2025-08-20
Maintainer: Claude Code Assistant