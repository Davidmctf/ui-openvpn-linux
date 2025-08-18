use std::process::Stdio;
use tokio::process::{Child, Command};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug)]
pub struct OpenVpnService {
    process: Arc<Mutex<Option<Child>>>,
    connected: Arc<Mutex<bool>>,
}

impl OpenVpnService {
    pub fn new() -> Self {
        Self {
            process: Arc::new(Mutex::new(None)),
            connected: Arc::new(Mutex::new(false)),
        }
    }

    pub async fn is_connected(&self) -> bool {
        *self.connected.lock().await
    }

    pub fn build_openvpn_args(&self, config_path: &str) -> Vec<String> {
        vec!["--config".to_string(), config_path.to_string()]
    }

    pub async fn connect(&self, config_path: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Kill existing connection if any
        self.disconnect().await?;

        let mut cmd = Command::new("sudo");
        cmd.arg("openvpn")
           .args(self.build_openvpn_args(config_path))
           .stdin(Stdio::null())
           .stdout(Stdio::piped())
           .stderr(Stdio::piped());

        let child = cmd.spawn()?;
        
        {
            let mut process = self.process.lock().await;
            *process = Some(child);
        }
        
        {
            let mut connected = self.connected.lock().await;
            *connected = true;
        }

        Ok(())
    }

    pub async fn disconnect(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut process_guard = self.process.lock().await;
        if let Some(mut child) = process_guard.take() {
            // Try to kill the process gracefully
            if let Err(e) = child.kill().await {
                eprintln!("Failed to kill OpenVPN process: {}", e);
            }
            
            // Wait for the process to exit
            if let Err(e) = child.wait().await {
                eprintln!("Failed to wait for OpenVPN process: {}", e);
            }
        }

        {
            let mut connected = self.connected.lock().await;
            *connected = false;
        }

        Ok(())
    }

    pub async fn get_status(&self) -> ConnectionStatus {
        let connected = self.is_connected().await;
        let process_guard = self.process.lock().await;
        let has_process = process_guard.is_some();

        match (connected, has_process) {
            (true, true) => ConnectionStatus::Connected,
            (false, false) => ConnectionStatus::Disconnected,
            (true, false) => ConnectionStatus::Error("Connected flag set but no process found".to_string()),
            (false, true) => ConnectionStatus::Connecting,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ConnectionStatus {
    Disconnected,
    Connecting,
    Connected,
    Error(String),
}

impl Default for OpenVpnService {
    fn default() -> Self {
        Self::new()
    }
}