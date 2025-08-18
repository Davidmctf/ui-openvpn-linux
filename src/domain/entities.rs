use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConnectionState {
    Disconnected,
    Connecting,
    Connected,
    Disconnecting,
    Error(String),
}

impl Default for ConnectionState {
    fn default() -> Self {
        Self::Disconnected
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VpnStatus {
    state: ConnectionState,
    ip_address: String,
    connected_since: Option<std::time::SystemTime>,
}

impl VpnStatus {
    pub fn new(state: ConnectionState, ip_address: String) -> Self {
        let connected_since = match state {
            ConnectionState::Connected => Some(std::time::SystemTime::now()),
            _ => None,
        };

        Self {
            state,
            ip_address,
            connected_since,
        }
    }

    pub fn state(&self) -> &ConnectionState {
        &self.state
    }

    pub fn ip_address(&self) -> &str {
        &self.ip_address
    }

    pub fn connected_since(&self) -> Option<std::time::SystemTime> {
        self.connected_since
    }

    pub fn set_state(&mut self, state: ConnectionState) {
        self.connected_since = match state {
            ConnectionState::Connected => Some(std::time::SystemTime::now()),
            _ => None,
        };
        self.state = state;
    }

    pub fn set_ip_address(&mut self, ip: String) {
        self.ip_address = ip;
    }
}

impl Default for VpnStatus {
    fn default() -> Self {
        Self {
            state: ConnectionState::Disconnected,
            ip_address: String::new(),
            connected_since: None,
        }
    }
}

#[derive(Error, Debug)]
pub enum VpnError {
    #[error("VPN ID cannot be empty")]
    EmptyId,
    #[error("Config path cannot be empty")]
    EmptyConfigPath,
    #[error("Invalid config file: {0}")]
    InvalidConfigFile(String),
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Vpn {
    id: String,
    display_name: String,
    config_path: String,
    status: VpnStatus,
}

impl Vpn {
    pub fn new(id: String, display_name: String, config_path: String) -> Self {
        Self {
            id,
            display_name,
            config_path,
            status: VpnStatus::default(),
        }
    }

    pub fn try_new(id: String, display_name: String, config_path: String) -> Result<Self, VpnError> {
        if id.trim().is_empty() {
            return Err(VpnError::EmptyId);
        }
        
        if config_path.trim().is_empty() {
            return Err(VpnError::EmptyConfigPath);
        }

        Ok(Self::new(id, display_name, config_path))
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn display_name(&self) -> &str {
        &self.display_name
    }

    pub fn config_path(&self) -> &str {
        &self.config_path
    }

    pub fn status(&self) -> &VpnStatus {
        &self.status
    }

    pub fn update_status(&mut self, status: VpnStatus) {
        self.status = status;
    }

    pub fn is_connected(&self) -> bool {
        matches!(self.status.state, ConnectionState::Connected)
    }

    pub fn is_connecting(&self) -> bool {
        matches!(self.status.state, ConnectionState::Connecting)
    }

    pub fn is_disconnected(&self) -> bool {
        matches!(self.status.state, ConnectionState::Disconnected)
    }
}