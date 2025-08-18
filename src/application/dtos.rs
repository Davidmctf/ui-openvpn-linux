use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VpnDto {
    pub id: String,
    pub display_name: String,
    pub config_path: String,
    pub status: VpnStatusDto,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VpnStatusDto {
    pub state: ConnectionStateDto,
    pub ip_address: String,
    pub connected_since: Option<String>, // ISO 8601 timestamp
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConnectionStateDto {
    Disconnected,
    Connecting,
    Connected,
    Disconnecting,
    Error(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConnectVpnRequest {
    pub vpn_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DisconnectVpnRequest {
    pub vpn_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnListResponse {
    pub vpns: Vec<VpnDto>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpnConnectionResponse {
    pub success: bool,
    pub message: String,
    pub vpn: Option<VpnDto>,
}