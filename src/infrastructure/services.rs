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
        // Check both internal state and actual system processes
        let internal_connected = *self.connected.lock().await;
        let system_connected = self.check_system_openvpn_processes().await;
        
        // If system says connected but internal says not, sync the internal state
        if system_connected && !internal_connected {
            let mut connected = self.connected.lock().await;
            *connected = true;
        }
        
        // If system says disconnected but internal says connected, sync the internal state
        if !system_connected && internal_connected {
            let mut connected = self.connected.lock().await;
            *connected = false;
        }
        
        system_connected
    }

    async fn check_system_openvpn_processes(&self) -> bool {
        self.get_connected_vpn_config().await.is_some()
    }

    pub async fn get_connected_vpn_config(&self) -> Option<String> {
        use std::process::Command;
        
        // Check if any OpenVPN processes are running and get their config file
        if let Ok(output) = Command::new("ps")
            .args(&["aux"])
            .output()
        {
            let output_str = String::from_utf8_lossy(&output.stdout);
            // Look for OpenVPN processes (excluding grep itself)
            for line in output_str.lines() {
                if line.contains("/usr/bin/openvpn") && 
                   line.contains("--config") && 
                   !line.contains("grep") {
                    // Extract the config file path from the ps output
                    if let Some(config_start) = line.find("--config") {
                        let config_part = &line[config_start + 8..]; // Skip "--config"
                        if let Some(config_path) = config_part.trim().split_whitespace().next() {
                            return Some(config_path.to_string());
                        }
                    }
                }
            }
        }
        
        None
    }

    pub fn build_openvpn_args(&self, config_path: &str) -> Vec<String> {
        vec!["--config".to_string(), config_path.to_string()]
    }

    pub async fn connect(&self, config_path: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Kill existing connection if any
        self.disconnect().await?;

        // Try pkexec first (GUI-friendly), fallback to sudo if not available
        let mut cmd = if Command::new("pkexec").arg("--version").output().await.is_ok() {
            let mut cmd = Command::new("pkexec");
            cmd.arg("openvpn")
               .args(self.build_openvpn_args(config_path));
            cmd
        } else {
            let mut cmd = Command::new("sudo");
            cmd.arg("openvpn")
               .args(self.build_openvpn_args(config_path));
            cmd
        };
        
        cmd.stdin(Stdio::null())
           .stdout(Stdio::piped())
           .stderr(Stdio::piped());

        let child = cmd.spawn().map_err(|e| {
            format!("Failed to start OpenVPN process: {}. Make sure OpenVPN is installed and you have proper permissions.", e)
        })?;
        
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

    pub async fn force_kill_all(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        use std::process::Command;

        // First try to kill any processes we're tracking
        {
            let mut process_guard = self.process.lock().await;
            if let Some(mut child) = process_guard.take() {
                let _ = child.kill().await; // Ignore errors
            }
        }

        // Force kill all system OpenVPN processes with sudo
        let kill_commands = [
            // Try pkill first (more targeted)
            vec!["pkill", "-f", "openvpn"],
            // Then try killall as fallback
            vec!["killall", "openvpn"],
            // Finally try with sudo for system processes
            vec!["sudo", "pkill", "-9", "-f", "openvpn"],
            vec!["sudo", "killall", "-9", "openvpn"],
        ];

        let mut killed_any = false;

        for cmd_args in &kill_commands {
            if let Ok(output) = Command::new(cmd_args[0])
                .args(&cmd_args[1..])
                .output()
            {
                if output.status.success() {
                    killed_any = true;
                }
            }
            
            // Small delay between attempts
            tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
            
            // Check if any processes are still running
            if !self.check_system_openvpn_processes().await {
                break; // All processes killed
            }
        }

        // Update internal state
        {
            let mut connected = self.connected.lock().await;
            *connected = false;
        }

        if killed_any || !self.check_system_openvpn_processes().await {
            Ok(())
        } else {
            Err("Failed to kill OpenVPN processes. Try running with sudo or check if processes exist.".into())
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