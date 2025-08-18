use ui_openvpn_linux::{
    application::services::VpnApplicationService,
    infrastructure::{repositories::FileVpnRepository, services::OpenVpnService},
};
use std::sync::Arc;
use std::io::{self, Write};

#[cfg(feature = "ui")]
use ui_openvpn_linux::ui::controllers::MainController;
#[cfg(feature = "ui")]
use gtk4::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize dependencies using Dependency Injection
    let vpn_repository = Arc::new(FileVpnRepository::from_home_dir());
    let openvpn_service = Arc::new(OpenVpnService::new());
    let vpn_app_service = Arc::new(VpnApplicationService::new(
        vpn_repository,
        openvpn_service,
    ));

    // Check command line arguments
    let args: Vec<String> = std::env::args().collect();
    
    // Handle command line arguments
    if args.len() > 1 {
        match args[1].as_str() {
            "--gui" | "-g" => {
                #[cfg(feature = "ui")]
                return run_gtk_ui(vpn_app_service).await;
                
                #[cfg(not(feature = "ui"))]
                {
                    eprintln!("❌ GTK4 UI no está disponible.");
                    eprintln!("Instala GTK4 y recompila con:");
                    eprintln!("  cargo build --features ui --release");
                    std::process::exit(1);
                }
            },
            "list" | "ls" => {
                return run_list_command(vpn_app_service).await;
            },
            "connect" => {
                if args.len() < 3 {
                    eprintln!("❌ Uso: {} connect <vpn_id>", args[0]);
                    std::process::exit(1);
                }
                return run_connect_command(vpn_app_service, &args[2]).await;
            },
            "disconnect" => {
                return run_disconnect_command(vpn_app_service).await;
            },
            "status" => {
                return run_status_command(vpn_app_service).await;
            },
            "--help" | "-h" => {
                print_help(&args[0]);
                std::process::exit(0);
            },
            _ => {
                eprintln!("❌ Comando desconocido: {}", args[1]);
                print_help(&args[0]);
                std::process::exit(1);
            }
        }
    }

    // Default to interactive CLI
    run_interactive_cli(vpn_app_service).await
}

fn print_help(program_name: &str) {
    println!("🚀 UI OpenVPN Linux - Gestión moderna de VPNs");
    println!("===============================================");
    println!();
    println!("USO:");
    println!("  {}                     # Modo interactivo", program_name);
    println!("  {} list               # Listar VPNs disponibles", program_name);
    println!("  {} connect <vpn_id>   # Conectar VPN específica", program_name);
    println!("  {} disconnect         # Desconectar VPN actual", program_name);
    println!("  {} status             # Ver estado de conexión", program_name);
    println!("  {} --gui              # Interfaz gráfica (requiere GTK4)", program_name);
    println!("  {} --help             # Mostrar esta ayuda", program_name);
    println!();
    println!("EJEMPLOS:");
    println!("  {} list", program_name);
    println!("  {} connect David_cruz", program_name);
    println!("  {} disconnect", program_name);
}

async fn run_list_command(
    vpn_app_service: Arc<VpnApplicationService>,
) -> Result<(), Box<dyn std::error::Error>> {
    match vpn_app_service.list_vpns().await {
        Ok(vpns) => {
            if vpns.is_empty() {
                println!("❌ No se encontraron VPNs en ~/.connectvpn.conf/");
            } else {
                println!("📋 VPNs disponibles:");
                for vpn in vpns {
                    let status = if vpn.is_connected() { "🟢" } else { "🔴" };
                    println!("  {} {} ({})", status, vpn.display_name(), vpn.id());
                }
            }
        },
        Err(e) => {
            eprintln!("❌ Error: {}", e);
            std::process::exit(1);
        }
    }
    Ok(())
}

async fn run_connect_command(
    vpn_app_service: Arc<VpnApplicationService>,
    vpn_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔌 Conectando a VPN: {}", vpn_id);
    match vpn_app_service.connect_vpn(vpn_id).await {
        Ok(()) => {
            println!("✅ ¡Conectado exitosamente a {}!", vpn_id);
        },
        Err(e) => {
            eprintln!("❌ Error al conectar: {}", e);
            std::process::exit(1);
        }
    }
    Ok(())
}

async fn run_disconnect_command(
    vpn_app_service: Arc<VpnApplicationService>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔌 Desconectando VPN...");
    match vpn_app_service.disconnect_current().await {
        Ok(()) => {
            println!("✅ Desconectado exitosamente.");
        },
        Err(e) => {
            eprintln!("❌ Error al desconectar: {}", e);
            std::process::exit(1);
        }
    }
    Ok(())
}

async fn run_status_command(
    vpn_app_service: Arc<VpnApplicationService>,
) -> Result<(), Box<dyn std::error::Error>> {
    match vpn_app_service.get_connection_status().await {
        Ok(vpns) => {
            println!("📊 Estado de conexiones:");
            for vpn in vpns {
                let status_text = if vpn.is_connected() {
                    "🟢 Conectado"
                } else {
                    "🔴 Desconectado"
                };
                println!("  {} - {}", vpn.display_name(), status_text);
            }
        },
        Err(e) => {
            eprintln!("❌ Error: {}", e);
            std::process::exit(1);
        }
    }
    Ok(())
}

async fn run_interactive_cli(
    vpn_app_service: Arc<VpnApplicationService>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 UI OpenVPN Linux (Modo CLI Simple)");
    println!("=====================================");
    
    loop {
        println!("\n🔒 Opciones disponibles:");
        println!("1) Listar VPNs");
        println!("2) Conectar VPN");
        println!("3) Desconectar");
        println!("4) Estado");
        println!("5) Salir");
        
        print!("👉 Opción (1-5): ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let choice = input.trim();
                
                match choice {
                    "1" => {
                        println!("\n📋 VPNs disponibles:");
                        match vpn_app_service.list_vpns().await {
                            Ok(vpns) => {
                                if vpns.is_empty() {
                                    println!("❌ No se encontraron VPNs.");
                                    println!("Verifica ~/.connectvpn.conf/");
                                } else {
                                    for (i, vpn) in vpns.iter().enumerate() {
                                        let status = if vpn.is_connected() { "🟢" } else { "🔴" };
                                        println!("  {}) {} {} ({})", i + 1, status, vpn.display_name(), vpn.id());
                                    }
                                }
                            },
                            Err(e) => println!("❌ Error: {}", e),
                        }
                    },
                    "2" => {
                        println!("\n🔌 Conectar VPN:");
                        match vpn_app_service.list_vpns().await {
                            Ok(vpns) => {
                                if vpns.is_empty() {
                                    println!("❌ No hay VPNs disponibles.");
                                } else {
                                    for (i, vpn) in vpns.iter().enumerate() {
                                        println!("  {}) {}", i + 1, vpn.display_name());
                                    }
                                    
                                    print!("Número de VPN: ");
                                    io::stdout().flush()?;
                                    
                                    let mut vpn_input = String::new();
                                    if io::stdin().read_line(&mut vpn_input).is_ok() {
                                        if let Ok(index) = vpn_input.trim().parse::<usize>() {
                                            if index > 0 && index <= vpns.len() {
                                                let vpn = &vpns[index - 1];
                                                println!("✅ Conectando a: {}", vpn.display_name());
                                                match vpn_app_service.connect_vpn(vpn.id()).await {
                                                    Ok(()) => println!("🎉 ¡Conectado!"),
                                                    Err(e) => println!("❌ Error: {}", e),
                                                }
                                            } else {
                                                println!("❌ Número inválido.");
                                            }
                                        } else {
                                            println!("❌ Por favor ingresa un número.");
                                        }
                                    }
                                }
                            },
                            Err(e) => println!("❌ Error: {}", e),
                        }
                    },
                    "3" => {
                        println!("\n🔌 Desconectando...");
                        match vpn_app_service.disconnect_current().await {
                            Ok(()) => println!("✅ Desconectado."),
                            Err(e) => println!("❌ Error: {}", e),
                        }
                    },
                    "4" => {
                        println!("\n📊 Estado de conexiones:");
                        match vpn_app_service.get_connection_status().await {
                            Ok(vpns) => {
                                for vpn in vpns {
                                    let status_text = if vpn.is_connected() {
                                        "🟢 Conectado"
                                    } else {
                                        "🔴 Desconectado"
                                    };
                                    println!("  {} - {}", vpn.display_name(), status_text);
                                }
                            },
                            Err(e) => println!("❌ Error: {}", e),
                        }
                    },
                    "5" => {
                        println!("¡Hasta luego! 👋");
                        break;
                    },
                    "" => {
                        // Entrada vacía, ignorar
                        continue;
                    },
                    _ => {
                        println!("❌ Opción inválida. Usa 1-5.");
                    }
                }
            },
            Err(e) => {
                println!("❌ Error leyendo entrada: {}", e);
                break;
            }
        }
    }
    
    Ok(())
}

#[cfg(feature = "ui")]
async fn run_gtk_ui(
    vpn_app_service: Arc<VpnApplicationService>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Iniciando UI OpenVPN Linux (Modo GTK4)");
    
    let app = gtk4::Application::builder()
        .application_id("com.osmiodev.ui-openvpn-linux")
        .build();

    app.connect_activate(move |app| {
        let controller = MainController::new(app, Arc::clone(&vpn_app_service));
        
        // Initialize the UI asynchronously
        let controller_clone = controller.clone();
        tokio::spawn(async move {
            if let Err(e) = controller_clone.initialize().await {
                eprintln!("Error initializing UI: {}", e);
            }
        });
        
        controller.show();
    });

    app.run();
    Ok(())
}

#[cfg(not(feature = "ui"))]
#[allow(unused_variables)]
async fn run_gtk_ui(
    _vpn_app_service: Arc<VpnApplicationService>,
) -> Result<(), Box<dyn std::error::Error>> {
    eprintln!("❌ GTK4 UI no está disponible.");
    eprintln!("Instala GTK4 y recompila con:");
    eprintln!("  cargo build --features ui --release");
    std::process::exit(1);
}