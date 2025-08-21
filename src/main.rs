use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::Path;
use std::process::Command;
use subprocess::{Exec, Redirection};

#[cfg(feature = "web")]
mod ui;

#[cfg(feature = "web")]
mod desktop_app;


/// Simple OpenVPN Manager for Linux
/// 
/// Manages OpenVPN connections through NetworkManager with a focus on simplicity and security.
/// Uses proper privilege escalation and follows Linux best practices.
#[derive(Parser)]
#[command(name = "openvpn-manager")]
#[command(about = "🔐 Simple OpenVPN Manager for Linux")]
#[command(version, author)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List all available VPN connections
    List,
    /// Import a new VPN configuration file
    Import { 
        /// Path to the .ovpn file
        file: String 
    },
    /// Connect to a VPN
    Connect { 
        /// Name of the VPN connection
        name: String 
    },
    /// Disconnect from current VPN
    Disconnect,
    /// Show status of all VPN connections
    Status,
    /// Remove a VPN connection
    Remove { 
        /// Name of the VPN connection to remove
        name: String 
    },
    /// Launch Web Interface (if available)
    #[cfg(feature = "web")]
    Web,
    /// Launch Desktop App (if available)
    #[cfg(feature = "web")]
    Desktop,
    /// Create Desktop Entry (if available)
    #[cfg(feature = "web")]
    Install,
}

#[cfg(feature = "web")]
#[tokio::main]
async fn main() -> Result<()> {
    run().await
}

#[cfg(not(feature = "web"))]
fn main() -> Result<()> {
    run_sync()
}

#[cfg(feature = "web")]
async fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::List => list_connections()?,
        Commands::Import { file } => import_vpn(&file)?,
        Commands::Connect { name } => connect_vpn(&name)?,
        Commands::Disconnect => disconnect_vpn()?,
        Commands::Status => show_status()?,
        Commands::Remove { name } => remove_vpn(&name)?,
        #[cfg(feature = "web")]
        Commands::Web => launch_web().await?,
        #[cfg(feature = "web")]
        Commands::Desktop => launch_desktop().await?,
        #[cfg(feature = "web")]
        Commands::Install => install_desktop()?,
    }

    Ok(())
}

#[cfg(not(feature = "web"))]
fn run_sync() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::List => list_connections()?,
        Commands::Import { file } => import_vpn(&file)?,
        Commands::Connect { name } => connect_vpn(&name)?,
        Commands::Disconnect => disconnect_vpn()?,
        Commands::Status => show_status()?,
        Commands::Remove { name } => remove_vpn(&name)?,
    }

    Ok(())
}

fn list_connections() -> Result<()> {
    println!("🔐 Available VPN Connections:");
    println!("────────────────────────────");

    // Use nmcli to list all connections, then filter for VPN type
    let output = Command::new("nmcli")
        .args(&["connection", "show"])
        .output()?;

    if output.status.success() {
        let connections = String::from_utf8_lossy(&output.stdout);
        let mut found_vpn = false;
        
        for line in connections.lines().skip(1) {
            if !line.trim().is_empty() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 3 && parts[2] == "vpn" {
                    println!("  📋 {}", parts[0]);
                    found_vpn = true;
                }
            }
        }
        
        if !found_vpn {
            println!("❌ No VPN connections found");
            println!("💡 Use 'openvpn-manager import <file.ovpn>' to add one");
        }
    } else {
        println!("❌ Error: NetworkManager not available or no permissions");
        println!("💡 Make sure NetworkManager is installed and you have permissions");
    }

    Ok(())
}

fn import_vpn(file_path: &str) -> Result<()> {
    println!("📥 Importing VPN configuration: {}", file_path);

    if !Path::new(file_path).exists() {
        println!("❌ File not found: {}", file_path);
        return Ok(());
    }

    // Import using nmcli
    let result = Exec::cmd("nmcli")
        .args(&["connection", "import", "type", "openvpn", "file", file_path])
        .stdout(Redirection::Pipe)
        .stderr(Redirection::Pipe)
        .capture()?;

    if result.success() {
        println!("✅ VPN configuration imported successfully!");
        println!("💡 Use 'openvpn-manager list' to see all connections");
        println!("💡 Use 'openvpn-manager connect <name>' to connect");
    } else {
        println!("❌ Failed to import VPN configuration");
        println!("Error: {}", String::from_utf8_lossy(&result.stderr));
        println!("💡 Make sure the file is a valid .ovpn configuration");
    }

    Ok(())
}

fn connect_vpn(name: &str) -> Result<()> {
    println!("🔌 Connecting to VPN: {}", name);

    // First disconnect any active VPN
    let _ = disconnect_vpn();

    // Connect using nmcli
    let result = Exec::cmd("nmcli")
        .args(&["connection", "up", name])
        .stdout(Redirection::Pipe)
        .stderr(Redirection::Pipe)
        .capture()?;

    if result.success() {
        println!("✅ Successfully connected to {}!", name);
        println!("💡 Use 'openvpn-manager status' to check your connection");
    } else {
        println!("❌ Failed to connect to {}", name);
        println!("Error: {}", String::from_utf8_lossy(&result.stderr));
        println!("💡 Check if the VPN name is correct with 'openvpn-manager list'");
    }

    Ok(())
}

fn disconnect_vpn() -> Result<()> {
    println!("🔌 Disconnecting VPN...");

    // Get active VPN connections
    let output = Command::new("nmcli")
        .args(&["connection", "show", "--active"])
        .output()?;

    if output.status.success() {
        let connections = String::from_utf8_lossy(&output.stdout);
        let mut disconnected_any = false;

        for line in connections.lines().skip(1) {
            if !line.trim().is_empty() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 3 && parts[2] == "vpn" {
                    let name = parts[0];
                    let result = Command::new("nmcli")
                        .args(&["connection", "down", name])
                        .output();
                    
                    if result.is_ok() {
                        println!("✅ Disconnected from {}", name);
                        disconnected_any = true;
                    }
                }
            }
        }

        if !disconnected_any {
            println!("💡 No active VPN connections found");
        }
    }

    Ok(())
}

fn show_status() -> Result<()> {
    println!("📊 VPN Connection Status:");
    println!("─────────────────────────");

    // Show active VPN connections
    let output = Command::new("nmcli")
        .args(&["connection", "show", "--active"])
        .output()?;

    if output.status.success() {
        let connections = String::from_utf8_lossy(&output.stdout);
        let mut found_active_vpn = false;
        
        for line in connections.lines().skip(1) {
            if !line.trim().is_empty() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 3 && parts[2] == "vpn" {
                    println!("🟢 Active VPN: {} - CONNECTED", parts[0]);
                    found_active_vpn = true;
                }
            }
        }
        
        if !found_active_vpn {
            println!("🔴 No active VPN connections");
        }
    }

    // Show all configured VPNs
    let output = Command::new("nmcli")
        .args(&["connection", "show"])
        .output()?;

    if output.status.success() {
        let connections = String::from_utf8_lossy(&output.stdout);
        let mut found_configured_vpn = false;
        
        println!("\n📋 All Configured VPNs:");
        for line in connections.lines().skip(1) {
            if !line.trim().is_empty() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 3 && parts[2] == "vpn" {
                    println!("  📄 {}", parts[0]);
                    found_configured_vpn = true;
                }
            }
        }
        
        if !found_configured_vpn {
            println!("  No VPN configurations found");
        }
    }

    Ok(())
}

fn remove_vpn(name: &str) -> Result<()> {
    println!("🗑️  Removing VPN connection: {}", name);

    let result = Exec::cmd("nmcli")
        .args(&["connection", "delete", name])
        .stdout(Redirection::Pipe)
        .stderr(Redirection::Pipe)
        .capture()?;

    if result.success() {
        println!("✅ VPN connection '{}' removed successfully!", name);
    } else {
        println!("❌ Failed to remove VPN connection '{}'", name);
        println!("Error: {}", String::from_utf8_lossy(&result.stderr));
        println!("💡 Check if the VPN name is correct with 'openvpn-manager list'");
    }

    Ok(())
}


#[cfg(feature = "web")]
async fn launch_web() -> Result<()> {
    ui::run_web_gui().await;
    Ok(())
}

#[cfg(feature = "web")]
async fn launch_desktop() -> Result<()> {
    desktop_app::launch_desktop_app().await?;
    Ok(())
}

#[cfg(feature = "web")]
fn install_desktop() -> Result<()> {
    desktop_app::create_desktop_entry()?;
    println!("✅ Desktop entry created successfully!");
    Ok(())
}

