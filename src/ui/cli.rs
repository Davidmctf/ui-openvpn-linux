use crate::application::services::VpnApplicationService;
use crate::application::mappers::VpnMapper;
use std::io::{self, Write};
use std::sync::Arc;

pub struct CliInterface {
    service: Arc<VpnApplicationService>,
}

impl CliInterface {
    pub fn new(service: Arc<VpnApplicationService>) -> Self {
        Self { service }
    }

    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            self.show_menu();
            
            let choice = self.get_user_input("👉 Ingresa tu opción: ")?;
            
            match choice.trim() {
                "1" => self.list_vpns().await?,
                "2" => self.connect_vpn().await?,
                "3" => self.disconnect_current().await?,
                "4" => self.show_status().await?,
                "5" => {
                    println!("¡Hasta luego!");
                    break;
                },
                _ => println!("❌ Opción inválida. Por favor intenta de nuevo."),
            }
            
            println!(); // Empty line for readability
        }
        
        Ok(())
    }

    fn show_menu(&self) {
        println!("🔒 UI OpenVPN Linux - Gestión de VPNs");
        println!("=====================================");
        println!("1) Listar VPNs disponibles");
        println!("2) Conectar VPN");
        println!("3) Desconectar VPN actual");
        println!("4) Ver estado de conexión");
        println!("5) Salir");
        println!();
    }

    async fn list_vpns(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("📋 VPNs disponibles:");
        println!("===================");
        
        let vpns = self.service.list_vpns().await?;
        
        if vpns.is_empty() {
            println!("❌ No se encontraron VPNs configuradas.");
            println!("Verifica que existan archivos .ovpn en ~/.connectvpn.conf/");
            return Ok(());
        }
        
        for (i, vpn) in vpns.iter().enumerate() {
            let status_emoji = match vpn.status().state() {
                crate::domain::entities::ConnectionState::Connected => "🟢",
                crate::domain::entities::ConnectionState::Connecting => "🟡",
                crate::domain::entities::ConnectionState::Disconnecting => "🟡",
                crate::domain::entities::ConnectionState::Disconnected => "🔴",
                crate::domain::entities::ConnectionState::Error(_) => "⚠️",
            };
            
            println!("  {}) {} {} ({})", 
                i + 1, 
                status_emoji, 
                vpn.display_name(), 
                vpn.id()
            );
        }
        
        Ok(())
    }

    async fn connect_vpn(&self) -> Result<(), Box<dyn std::error::Error>> {
        let vpns = self.service.list_vpns().await?;
        
        if vpns.is_empty() {
            println!("❌ No hay VPNs disponibles para conectar.");
            return Ok(());
        }
        
        println!("🔌 Conectar VPN:");
        println!("================");
        
        for (i, vpn) in vpns.iter().enumerate() {
            println!("  {}) {}", i + 1, vpn.display_name());
        }
        
        let choice = self.get_user_input("👉 Selecciona el número de VPN: ")?;
        
        if let Ok(index) = choice.trim().parse::<usize>() {
            if index > 0 && index <= vpns.len() {
                let vpn = &vpns[index - 1];
                println!("✅ Conectando a: {}", vpn.display_name());
                
                match self.service.connect_vpn(vpn.id()).await {
                    Ok(()) => {
                        println!("🎉 ¡Conectado exitosamente a {}!", vpn.display_name());
                    },
                    Err(e) => {
                        println!("❌ Error al conectar: {}", e);
                    }
                }
            } else {
                println!("❌ Selección inválida.");
            }
        } else {
            println!("❌ Por favor ingresa un número válido.");
        }
        
        Ok(())
    }

    async fn disconnect_current(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🔌 Desconectando VPN actual...");
        
        match self.service.disconnect_current().await {
            Ok(()) => {
                println!("✅ Desconectado exitosamente.");
            },
            Err(e) => {
                println!("❌ Error al desconectar: {}", e);
            }
        }
        
        Ok(())
    }

    async fn show_status(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("📊 Estado de conexiones:");
        println!("========================");
        
        let vpns = self.service.get_connection_status().await?;
        
        for vpn in vpns {
            let dto = VpnMapper::to_dto(&vpn);
            let status_text = match dto.status.state {
                crate::application::dtos::ConnectionStateDto::Connected => {
                    format!("🟢 Conectado - IP: {}", dto.status.ip_address)
                },
                crate::application::dtos::ConnectionStateDto::Connecting => "🟡 Conectando...".to_string(),
                crate::application::dtos::ConnectionStateDto::Disconnecting => "🟡 Desconectando...".to_string(),
                crate::application::dtos::ConnectionStateDto::Disconnected => "🔴 Desconectado".to_string(),
                crate::application::dtos::ConnectionStateDto::Error(ref msg) => {
                    format!("⚠️ Error: {}", msg)
                },
            };
            
            println!("  {} - {}", dto.display_name, status_text);
        }
        
        Ok(())
    }

    fn get_user_input(&self, prompt: &str) -> Result<String, io::Error> {
        print!("{}", prompt);
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        Ok(input)
    }
}