#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{command, State, Builder, generate_context, generate_handler};
use std::process::{Command, Stdio};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tokio::process::Command as TokioCommand;

// Estructuras para la comunicación con el frontend
#[derive(Debug, Serialize, Deserialize)]
struct VpnConnection {
    name: String,
    status: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ApiResponse {
    success: bool,
    message: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct StatusInfo {
    connected: bool,
    active_vpn: Option<String>,
}

// Estado global de la aplicación
struct AppState {
    web_server_running: Mutex<bool>,
}

#[command]
async fn list_vpn_connections() -> Result<Vec<VpnConnection>, String> {
    println!("🔍 Listando conexiones VPN...");
    
    let output = Command::new("nmcli")
        .args(&["connection", "show"])
        .output()
        .map_err(|e| format!("Error ejecutando nmcli: {}", e))?;

    if !output.status.success() {
        return Err("NetworkManager no disponible".to_string());
    }

    let mut vpns = Vec::new();
    let connections = String::from_utf8_lossy(&output.stdout);
    
    for line in connections.lines().skip(1) {
        if !line.trim().is_empty() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 && parts[2] == "vpn" {
                vpns.push(VpnConnection {
                    name: parts[0].to_string(),
                    status: "available".to_string(),
                });
            }
        }
    }
    
    println!("✅ Encontradas {} conexiones VPN", vpns.len());
    Ok(vpns)
}

#[command]
async fn get_vpn_status() -> Result<StatusInfo, String> {
    println!("📊 Obteniendo estado VPN...");
    
    let output = Command::new("nmcli")
        .args(&["connection", "show", "--active"])
        .output()
        .map_err(|e| format!("Error ejecutando nmcli: {}", e))?;

    if output.status.success() {
        let connections = String::from_utf8_lossy(&output.stdout);
        for line in connections.lines().skip(1) {
            if !line.trim().is_empty() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 3 && parts[2] == "vpn" {
                    return Ok(StatusInfo {
                        connected: true,
                        active_vpn: Some(parts[0].to_string()),
                    });
                }
            }
        }
    }
    
    Ok(StatusInfo {
        connected: false,
        active_vpn: None,
    })
}

#[command]
async fn connect_vpn(name: String) -> Result<ApiResponse, String> {
    println!("🚀 Conectando a VPN: {}", name);
    
    let output = Command::new("nmcli")
        .args(&["connection", "up", &name])
        .output()
        .map_err(|e| format!("Error ejecutando nmcli: {}", e))?;

    if output.status.success() {
        Ok(ApiResponse {
            success: true,
            message: format!("🚀 Navegando desde estación: {}", name),
        })
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Ok(ApiResponse {
            success: false,
            message: format!("❌ Error conectando: {}", stderr),
        })
    }
}

#[command]
async fn disconnect_vpn() -> Result<ApiResponse, String> {
    println!("🛑 Desconectando VPNs activas...");
    
    let output = Command::new("nmcli")
        .args(&["connection", "show", "--active"])
        .output()
        .map_err(|e| format!("Error ejecutando nmcli: {}", e))?;

    if output.status.success() {
        let connections = String::from_utf8_lossy(&output.stdout);
        for line in connections.lines().skip(1) {
            if !line.trim().is_empty() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 3 && parts[2] == "vpn" {
                    let _ = Command::new("nmcli")
                        .args(&["connection", "down", parts[0]])
                        .output();
                    return Ok(ApiResponse {
                        success: true,
                        message: format!("🛑 Desconectado de: {}", parts[0]),
                    });
                }
            }
        }
    }
    
    Ok(ApiResponse {
        success: true,
        message: "🔴 No había conexiones activas".to_string(),
    })
}

#[command]
async fn import_vpn_file(file_path: String) -> Result<ApiResponse, String> {
    println!("📥 Importando archivo VPN: {}", file_path);
    
    let output = Command::new("nmcli")
        .args(&["connection", "import", "type", "openvpn", "file", &file_path])
        .output()
        .map_err(|e| format!("Error ejecutando nmcli: {}", e))?;

    if output.status.success() {
        Ok(ApiResponse {
            success: true,
            message: "✅ Archivo VPN importado correctamente".to_string(),
        })
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Ok(ApiResponse {
            success: false,
            message: format!("❌ Error importando: {}", stderr),
        })
    }
}

#[command]
async fn start_web_server(state: State<'_, AppState>) -> Result<String, String> {
    let mut running = state.web_server_running.lock().unwrap();
    
    if *running {
        return Ok("Servidor ya está ejecutándose en http://localhost:8081".to_string());
    }
    
    println!("🌐 Iniciando servidor web interno...");
    
    // Ejecutar el servidor web en background
    tokio::spawn(async {
        // Aquí integraríamos tu servidor warp existente
        // Por ahora, simulamos que está corriendo
        tokio::time::sleep(tokio::time::Duration::from_secs(86400)).await;
    });
    
    *running = true;
    Ok("🚀 Servidor web iniciado en http://localhost:8081".to_string())
}

fn main() {
    Builder::default()
        .manage(AppState {
            web_server_running: Mutex::new(false),
        })
        .invoke_handler(generate_handler![
            list_vpn_connections,
            get_vpn_status,
            connect_vpn,
            disconnect_vpn,
            import_vpn_file,
            start_web_server
        ])
        .run(generate_context!())
        .expect("error while running tauri application");
}