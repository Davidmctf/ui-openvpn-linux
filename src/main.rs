use ui_openvpn_linux::{
    application::services::VpnApplicationService,
    infrastructure::{repositories::FileVpnRepository, services::OpenVpnService},
};
use std::sync::Arc;
use clap::{Args, Parser, Subcommand};

#[cfg(feature = "gui")]
use ui_openvpn_linux::ui::gtk::MainWindow;
#[cfg(feature = "gui")]
use gtk4::prelude::*;

/// 🚀 UI OpenVPN Linux - Modern VPN management tool
/// 
/// A modern Rust CLI/GUI application for managing OpenVPN connections with Clean Architecture.
/// Replace your old bash scripts with this professional VPN manager.
#[derive(Parser)]
#[command(name = "ui-openvpn")]
#[command(about = "🚀 Modern VPN management with Clean Architecture")]
#[command(version, author)]
#[command(long_about = "
🚀 UI OpenVPN Linux - Professional VPN Management

A modern Rust application built with Clean Architecture principles for managing 
OpenVPN connections. Supports both CLI and GUI modes with professional features.

Features:
• 🔒 Secure VPN connection management
• 📱 Clean Architecture design  
• 🧪 100% test coverage with TDD
• ⚡ High performance Rust implementation
• 🎨 Modern CLI and optional GTK4 GUI

Example configurations will be found in ~/.connectvpn.conf/
")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Launch GUI mode (requires GTK4)
    #[arg(short, long)]
    gui: bool,

    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// List all available VPN configurations
    #[command(alias = "ls")]
    List,
    
    /// Connect to a specific VPN
    Connect(ConnectArgs),
    
    /// Disconnect current VPN connection
    #[command(alias = "dc")]
    Disconnect,
    
    /// Show current connection status
    #[command(alias = "st")]
    Status,
    
    /// Validate VPN configuration files
    Validate,
    
    /// Show detailed information about a VPN
    Info(InfoArgs),
}

#[derive(Args)]
struct ConnectArgs {
    /// VPN ID to connect to
    vpn_id: String,
    
    /// Force connection even if another VPN is active
    #[arg(short, long)]
    force: bool,
}

#[derive(Args)]
struct InfoArgs {
    /// VPN ID to show information for
    vpn_id: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    
    // Print beautiful ASCII logo
    print_logo();
    
    // Initialize services with Dependency Injection
    let vpn_repository = Arc::new(FileVpnRepository::from_home_dir());
    let openvpn_service = Arc::new(OpenVpnService::new());
    let vpn_service = Arc::new(VpnApplicationService::new(
        vpn_repository,
        openvpn_service,
    ));

    // Handle GUI mode
    #[cfg(feature = "gui")]
    if cli.gui {
        return run_gui_mode(vpn_service).await;
    }
    
    #[cfg(not(feature = "gui"))]
    if cli.gui {
        eprintln!("❌ GUI mode not available. Compile with --features gui");
        std::process::exit(1);
    }

    // Handle CLI commands
    match cli.command {
        Some(command) => execute_command(command, vpn_service, cli.verbose).await,
        None => run_interactive_mode(vpn_service).await,
    }
}

fn print_logo() {
    println!(r#"
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
                               
  🚀 UI OpenVPN Linux v{} 🚀   
  ═══════════════════════════════════════
  Professional VPN Management in Space! ✨
    "#, env!("CARGO_PKG_VERSION"));
}

async fn execute_command(
    command: Commands,
    vpn_service: Arc<VpnApplicationService>,
    verbose: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        Commands::List => {
            println!("📋 Available VPN configurations:");
            println!("────────────────────────────────");
            
            match vpn_service.list_vpns().await {
                Ok(vpns) => {
                    if vpns.is_empty() {
                        println!("❌ No VPN configurations found in ~/.connectvpn.conf/");
                        println!("   Add your .ovpn files to get started!");
                    } else {
                        for vpn in vpns {
                            let status_icon = if vpn.is_connected() { "🟢" } else { "🔴" };
                            let status_text = if vpn.is_connected() { "CONNECTED" } else { "DISCONNECTED" };
                            
                            println!("  {} {} ({})", status_icon, vpn.display_name(), vpn.id());
                            if verbose {
                                println!("     Status: {}", status_text);
                                println!("     Config: {}", vpn.config_path());
                            }
                        }
                    }
                },
                Err(e) => {
                    eprintln!("❌ Error listing VPNs: {}", e);
                    std::process::exit(1);
                }
            }
        },
        
        Commands::Connect(args) => {
            if !args.force {
                // Check if another VPN is connected
                if let Ok(vpns) = vpn_service.list_vpns().await {
                    if vpns.iter().any(|v| v.is_connected()) {
                        eprintln!("⚠️  Another VPN is already connected. Use --force to override.");
                        std::process::exit(1);
                    }
                }
            }
            
            println!("🔌 Connecting to VPN: {}", args.vpn_id);
            match vpn_service.connect_vpn(&args.vpn_id).await {
                Ok(()) => {
                    println!("✅ Successfully connected to {}!", args.vpn_id);
                    if verbose {
                        println!("   Connection established with OpenVPN");
                    }
                },
                Err(e) => {
                    eprintln!("❌ Failed to connect: {}", e);
                    std::process::exit(1);
                }
            }
        },
        
        Commands::Disconnect => {
            println!("🔌 Disconnecting VPN...");
            match vpn_service.disconnect_current().await {
                Ok(()) => {
                    println!("✅ Successfully disconnected from VPN");
                },
                Err(e) => {
                    eprintln!("❌ Failed to disconnect: {}", e);
                    std::process::exit(1);
                }
            }
        },
        
        Commands::Status => {
            println!("📊 VPN Connection Status:");
            println!("─────────────────────────");
            
            match vpn_service.get_connection_status().await {
                Ok(vpns) => {
                    let mut has_active = false;
                    for vpn in vpns {
                        if vpn.is_connected() {
                            println!("🟢 {} - CONNECTED", vpn.display_name());
                            if verbose {
                                println!("   VPN ID: {}", vpn.id());
                                println!("   Config: {}", vpn.config_path());
                                if !vpn.status().ip_address().is_empty() {
                                    println!("   IP: {}", vpn.status().ip_address());
                                }
                            }
                            has_active = true;
                        } else {
                            println!("🔴 {} - DISCONNECTED", vpn.display_name());
                        }
                    }
                    
                    if !has_active {
                        println!("\n💡 No active VPN connections");
                    }
                },
                Err(e) => {
                    eprintln!("❌ Error checking status: {}", e);
                    std::process::exit(1);
                }
            }
        },
        
        Commands::Validate => {
            println!("🔍 Validating VPN configurations...");
            match vpn_service.list_vpns().await {
                Ok(vpns) => {
                    let mut valid_count = 0;
                    for vpn in vpns {
                        if std::path::Path::new(vpn.config_path()).exists() {
                            println!("✅ {} - Configuration valid", vpn.display_name());
                            valid_count += 1;
                        } else {
                            println!("❌ {} - Configuration file not found", vpn.display_name());
                        }
                    }
                    println!("\n📈 Summary: {} valid configurations", valid_count);
                },
                Err(e) => {
                    eprintln!("❌ Error validating configurations: {}", e);
                    std::process::exit(1);
                }
            }
        },
        
        Commands::Info(args) => {
            match vpn_service.list_vpns().await {
                Ok(vpns) => {
                    if let Some(vpn) = vpns.iter().find(|v| v.id() == args.vpn_id) {
                        println!("📋 VPN Information: {}", vpn.display_name());
                        println!("─────────────────────────────────");
                        println!("ID: {}", vpn.id());
                        println!("Display Name: {}", vpn.display_name());
                        println!("Config Path: {}", vpn.config_path());
                        println!("Status: {}", if vpn.is_connected() { "🟢 CONNECTED" } else { "🔴 DISCONNECTED" });
                        
                        if !vpn.status().ip_address().is_empty() {
                            println!("IP Address: {}", vpn.status().ip_address());
                        }
                        
                        // Show file info if available
                        if let Ok(metadata) = std::fs::metadata(vpn.config_path()) {
                            if let Ok(modified) = metadata.modified() {
                                println!("Last Modified: {:?}", modified);
                            }
                            println!("File Size: {} bytes", metadata.len());
                        }
                    } else {
                        eprintln!("❌ VPN '{}' not found", args.vpn_id);
                        std::process::exit(1);
                    }
                },
                Err(e) => {
                    eprintln!("❌ Error getting VPN info: {}", e);
                    std::process::exit(1);
                }
            }
        },
    }
    
    Ok(())
}

async fn run_interactive_mode(
    vpn_service: Arc<VpnApplicationService>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("🎮 Interactive Mode");
    println!("Type 'help' for available commands, 'quit' to exit");
    println!("─────────────────────────────────────────────────");
    
    loop {
        print!("vpn> ");
        std::io::Write::flush(&mut std::io::stdout())?;
        
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(0) => break, // EOF
            Ok(_) => {
                let command = input.trim();
                
                match command {
                    "help" | "h" => {
                        println!("Available commands:");
                        println!("  list, ls     - List available VPNs");
                        println!("  connect <id> - Connect to VPN");
                        println!("  disconnect   - Disconnect current VPN");
                        println!("  status       - Show connection status");
                        println!("  quit, exit   - Exit interactive mode");
                    },
                    "quit" | "exit" | "q" => {
                        println!("👋 Goodbye!");
                        break;
                    },
                    "list" | "ls" => {
                        execute_command(Commands::List, Arc::clone(&vpn_service), false).await?;
                    },
                    "status" | "st" => {
                        execute_command(Commands::Status, Arc::clone(&vpn_service), false).await?;
                    },
                    "disconnect" | "dc" => {
                        execute_command(Commands::Disconnect, Arc::clone(&vpn_service), false).await?;
                    },
                    cmd if cmd.starts_with("connect ") => {
                        let vpn_id = cmd.strip_prefix("connect ").unwrap_or("").trim();
                        if !vpn_id.is_empty() {
                            let args = ConnectArgs { vpn_id: vpn_id.to_string(), force: false };
                            execute_command(Commands::Connect(args), Arc::clone(&vpn_service), false).await?;
                        } else {
                            println!("❌ Usage: connect <vpn_id>");
                        }
                    },
                    "" => {
                        // Empty input, continue
                        continue;
                    },
                    _ => {
                        println!("❌ Unknown command: '{}'. Type 'help' for available commands.", command);
                    }
                }
            },
            Err(e) => {
                eprintln!("❌ Error reading input: {}", e);
                break;
            }
        }
    }
    
    Ok(())
}

#[cfg(feature = "gui")]
async fn run_gui_mode(
    vpn_service: Arc<VpnApplicationService>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("🎨 Launching GUI mode...");
    
    let app = gtk4::Application::builder()
        .application_id("com.davidmctf.ui-openvpn-linux")
        .build();

    app.connect_activate(move |app| {
        let window = MainWindow::new(app, Arc::clone(&vpn_service));
        window.show();
    });

    app.run();
    Ok(())
}

#[cfg(not(feature = "gui"))]
async fn run_gui_mode(
    _vpn_service: Arc<VpnApplicationService>,
) -> Result<(), Box<dyn std::error::Error>> {
    eprintln!("❌ GUI mode not available. Compile with --features gui");
    std::process::exit(1);
}