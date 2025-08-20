use crate::domain::{
    entities::{Vpn, VpnStatus, ConnectionState},
    repositories::VpnRepository,
    use_cases::{ConnectVpnUseCase, DisconnectVpnUseCase, ListVpnsUseCase},
};
use crate::infrastructure::services::OpenVpnService;
use std::sync::Arc;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum VpnServiceError {
    #[error("VPN not found: {0}")]
    VpnNotFound(String),
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),
    #[error("Repository error: {0}")]
    RepositoryError(String),
    #[error("OpenVPN service error: {0}")]
    OpenVpnError(String),
}

pub struct VpnApplicationService {
    vpn_repository: Arc<dyn VpnRepository>,
    openvpn_service: Arc<OpenVpnService>,
    _connect_use_case: ConnectVpnUseCase,
    _disconnect_use_case: DisconnectVpnUseCase,
    list_use_case: ListVpnsUseCase,
}

impl VpnApplicationService {
    pub fn new(
        vpn_repository: Arc<dyn VpnRepository>,
        openvpn_service: Arc<OpenVpnService>,
    ) -> Self {
        let connect_use_case = ConnectVpnUseCase::new(Arc::clone(&vpn_repository));
        let disconnect_use_case = DisconnectVpnUseCase::new(Arc::clone(&vpn_repository));
        let list_use_case = ListVpnsUseCase::new(Arc::clone(&vpn_repository));

        Self {
            vpn_repository,
            openvpn_service,
            _connect_use_case: connect_use_case,
            _disconnect_use_case: disconnect_use_case,
            list_use_case,
        }
    }

    pub async fn list_vpns(&self) -> Result<Vec<Vpn>, VpnServiceError> {
        let mut vpns = self.list_use_case
            .execute()
            .await
            .map_err(|e| VpnServiceError::RepositoryError(e.to_string()))?;

        // Update VPN status based on actual system state
        self.sync_vpn_states(&mut vpns).await?;
        
        Ok(vpns)
    }

    async fn sync_vpn_states(&self, vpns: &mut Vec<Vpn>) -> Result<(), VpnServiceError> {
        // Get the currently connected VPN config file (if any)
        let connected_config = self.openvpn_service.get_connected_vpn_config().await;
        
        for vpn in vpns.iter_mut() {
            let should_be_connected = match &connected_config {
                Some(config_path) => vpn.config_path() == config_path,
                None => false,
            };
            
            // Update VPN status based on actual system state
            let new_state = if should_be_connected {
                ConnectionState::Connected
            } else {
                ConnectionState::Disconnected
            };
            
            // Only update if the state has changed
            if vpn.is_connected() != should_be_connected {
                vpn.update_status(VpnStatus::new(new_state, String::new()));
            }
        }
        
        Ok(())
    }

    pub async fn connect_vpn(&self, vpn_id: &str) -> Result<(), VpnServiceError> {
        // ALWAYS force kill all VPN processes to ensure only one connection
        self.force_kill_all_vpns().await?;
        
        // Small delay to ensure processes are fully terminated
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

        // Get VPN configuration
        let vpn = self
            .vpn_repository
            .find_by_id(vpn_id)
            .await
            .map_err(|e| VpnServiceError::RepositoryError(e.to_string()))?
            .ok_or_else(|| VpnServiceError::VpnNotFound(vpn_id.to_string()))?;

        // Update VPN status to connecting
        let mut updated_vpn = vpn.clone();
        updated_vpn.update_status(VpnStatus::new(ConnectionState::Connecting, String::new()));
        self.vpn_repository
            .save(&updated_vpn)
            .await
            .map_err(|e| VpnServiceError::RepositoryError(e.to_string()))?;

        // Connect using OpenVPN service
        self.openvpn_service
            .connect(vpn.config_path())
            .await
            .map_err(|e| VpnServiceError::OpenVpnError(e.to_string()))?;

        // Update VPN status to connected
        updated_vpn.update_status(VpnStatus::new(ConnectionState::Connected, String::new()));
        self.vpn_repository
            .save(&updated_vpn)
            .await
            .map_err(|e| VpnServiceError::RepositoryError(e.to_string()))?;

        Ok(())
    }

    pub async fn disconnect_vpn(&self, vpn_id: &str) -> Result<(), VpnServiceError> {
        // Get VPN configuration
        let vpn = self
            .vpn_repository
            .find_by_id(vpn_id)
            .await
            .map_err(|e| VpnServiceError::RepositoryError(e.to_string()))?
            .ok_or_else(|| VpnServiceError::VpnNotFound(vpn_id.to_string()))?;

        // Update VPN status to disconnecting
        let mut updated_vpn = vpn.clone();
        updated_vpn.update_status(VpnStatus::new(ConnectionState::Disconnecting, String::new()));
        self.vpn_repository
            .save(&updated_vpn)
            .await
            .map_err(|e| VpnServiceError::RepositoryError(e.to_string()))?;

        // Disconnect using OpenVPN service
        self.openvpn_service
            .disconnect()
            .await
            .map_err(|e| VpnServiceError::OpenVpnError(e.to_string()))?;

        // Update VPN status to disconnected
        updated_vpn.update_status(VpnStatus::new(ConnectionState::Disconnected, String::new()));
        self.vpn_repository
            .save(&updated_vpn)
            .await
            .map_err(|e| VpnServiceError::RepositoryError(e.to_string()))?;

        Ok(())
    }

    pub async fn disconnect_current(&self) -> Result<(), VpnServiceError> {
        if !self.openvpn_service.is_connected().await {
            return Ok(());
        }

        // Disconnect using OpenVPN service
        self.openvpn_service
            .disconnect()
            .await
            .map_err(|e| VpnServiceError::OpenVpnError(e.to_string()))?;

        // Update all VPNs status to disconnected
        let vpns = self.list_vpns().await?;
        for mut vpn in vpns {
            if vpn.is_connected() || vpn.is_connecting() {
                vpn.update_status(VpnStatus::new(ConnectionState::Disconnected, String::new()));
                self.vpn_repository
                    .save(&vpn)
                    .await
                    .map_err(|e| VpnServiceError::RepositoryError(e.to_string()))?;
            }
        }

        Ok(())
    }

    pub async fn get_connection_status(&self) -> Result<Vec<Vpn>, VpnServiceError> {
        let vpns = self.list_vpns().await?;
        Ok(vpns)
    }

    pub async fn force_kill_all_vpns(&self) -> Result<(), VpnServiceError> {
        // Force kill all OpenVPN processes using system commands
        self.openvpn_service
            .force_kill_all()
            .await
            .map_err(|e| VpnServiceError::OpenVpnError(e.to_string()))?;

        // Update all VPNs status to disconnected
        let vpns = self.list_use_case
            .execute()
            .await
            .map_err(|e| VpnServiceError::RepositoryError(e.to_string()))?;
            
        for mut vpn in vpns {
            if vpn.is_connected() || vpn.is_connecting() {
                vpn.update_status(VpnStatus::new(ConnectionState::Disconnected, String::new()));
                self.vpn_repository
                    .save(&vpn)
                    .await
                    .map_err(|e| VpnServiceError::RepositoryError(e.to_string()))?;
            }
        }

        Ok(())
    }
}