use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

pub async fn launch_desktop_app() -> anyhow::Result<()> {
    println!("🚀 Lanzando UI OpenVPN Linux como aplicación de escritorio...");
    
    // Iniciar servidor web en segundo plano
    let web_server = tokio::spawn(async {
        crate::ui::run_web_gui().await;
    });
    
    // Esperar un momento para que el servidor se inicie
    thread::sleep(Duration::from_millis(2000));
    
    // Intentar abrir con diferentes navegadores en modo app
    let browsers = [
        ("google-chrome", vec!["--app=http://localhost:8081", "--no-toolbar", "--no-location-bar", "--no-address-bar", "--no-status-bar", "--disable-web-security"]),
        ("chromium", vec!["--app=http://localhost:8081", "--no-toolbar", "--no-location-bar", "--no-address-bar", "--no-status-bar"]),
        ("firefox", vec!["--new-window", "http://localhost:8081"]),
        ("epiphany", vec!["--application-mode", "http://localhost:8081"]),
    ];
    
    let mut app_launched = false;
    
    for (browser, args) in &browsers {
        if let Ok(_) = Command::new(browser)
            .args(args)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
        {
            println!("✅ Aplicación lanzada con {}", browser);
            app_launched = true;
            break;
        }
    }
    
    if !app_launched {
        println!("⚠️  No se encontró un navegador compatible.");
        println!("   Abre manualmente: http://localhost:8081");
        
        // Fallback: abrir navegador por defecto
        let _ = Command::new("xdg-open")
            .arg("http://localhost:8081")
            .spawn();
    }
    
    // Esperar a que termine el servidor web
    web_server.await?;
    
    Ok(())
}

pub fn create_desktop_entry() -> anyhow::Result<()> {
    let home = std::env::var("HOME")?;
    let desktop_dir = format!("{}/.local/share/applications", home);
    std::fs::create_dir_all(&desktop_dir)?;
    
    let current_exe = std::env::current_exe()?;
    let exe_path = current_exe.to_string_lossy();
    
    let desktop_content = format!(r#"[Desktop Entry]
Version=1.0
Type=Application
Name=UI OpenVPN Linux
Comment=🚀 Navegador Espacial de Conexiones VPN
Exec={} desktop
Icon={}
Terminal=false
Categories=Network;System;Security;
MimeType=application/x-openvpn-profile;
Keywords=vpn;openvpn;network;security;connection;
StartupNotify=true
"#, exe_path, get_icon_path());
    
    let desktop_file = format!("{}/ui-openvpn-linux.desktop", desktop_dir);
    std::fs::write(&desktop_file, desktop_content)?;
    
    // Hacer ejecutable
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&desktop_file)?.permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&desktop_file, perms)?;
    }
    
    println!("✅ Archivo .desktop creado en: {}", desktop_file);
    println!("   Ahora puedes encontrar 'UI OpenVPN Linux' en tu menú de aplicaciones");
    
    Ok(())
}

fn get_icon_path() -> String {
    // Intentar usar el icono del proyecto
    let current_dir = std::env::current_dir().unwrap_or_default();
    let icon_path = current_dir.join("assets/2261aaaa-bad7-4426-8cc1-f93cd6c4c067.png");
    
    if icon_path.exists() {
        icon_path.to_string_lossy().to_string()
    } else {
        // Fallback a icono del sistema
        "network-vpn".to_string()
    }
}