use ui_openvpn_linux::{
    application::services::VpnApplicationService,
    infrastructure::{repositories::FileVpnRepository, services::OpenVpnService},
    ui::cli::CliInterface,
};
use std::sync::Arc;

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

    // Check command line arguments for UI mode
    let args: Vec<String> = std::env::args().collect();
    let use_gtk = args.contains(&"--gui".to_string()) || args.contains(&"-g".to_string());

    #[cfg(feature = "ui")]
    if use_gtk {
        return run_gtk_ui(vpn_app_service).await;
    }

    // Default to CLI interface
    run_cli_ui(vpn_app_service).await
}

async fn run_cli_ui(
    vpn_app_service: Arc<VpnApplicationService>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Iniciando UI OpenVPN Linux (Modo CLI)");
    println!("Tip: Usa --gui o -g para la interfaz gráfica");
    println!();

    let cli = CliInterface::new(vpn_app_service);
    cli.run().await
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
async fn run_gtk_ui(
    _vpn_app_service: Arc<VpnApplicationService>,
) -> Result<(), Box<dyn std::error::Error>> {
    eprintln!("❌ GTK4 UI no está disponible. Recompila con la feature 'ui' habilitada.");
    eprintln!("Ejecuta: cargo build --features ui");
    std::process::exit(1);
}