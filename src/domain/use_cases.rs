use crate::domain::{
    entities::{Vpn, VpnStatus, ConnectionState},
    repositories::VpnRepository,
};
use std::sync::Arc;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum UseCaseError {
    #[error("VPN not found: {0}")]
    VpnNotFound(String),
    #[error("Repository error: {0}")]
    RepositoryError(String),
    #[error("Connection error: {0}")]
    ConnectionError(String),
}

pub struct ConnectVpnUseCase {
    repository: Arc<dyn VpnRepository>,
}

impl ConnectVpnUseCase {
    pub fn new(repository: Arc<dyn VpnRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, vpn_id: &str) -> Result<Vpn, UseCaseError> {
        let mut vpn = self
            .repository
            .find_by_id(vpn_id)
            .await
            .map_err(|e| UseCaseError::RepositoryError(e.to_string()))?
            .ok_or_else(|| UseCaseError::VpnNotFound(vpn_id.to_string()))?;

        let new_status = VpnStatus::new(ConnectionState::Connecting, String::new());
        vpn.update_status(new_status);

        self.repository
            .save(&vpn)
            .await
            .map_err(|e| UseCaseError::RepositoryError(e.to_string()))?;

        Ok(vpn)
    }
}

pub struct DisconnectVpnUseCase {
    repository: Arc<dyn VpnRepository>,
}

impl DisconnectVpnUseCase {
    pub fn new(repository: Arc<dyn VpnRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, vpn_id: &str) -> Result<Vpn, UseCaseError> {
        let mut vpn = self
            .repository
            .find_by_id(vpn_id)
            .await
            .map_err(|e| UseCaseError::RepositoryError(e.to_string()))?
            .ok_or_else(|| UseCaseError::VpnNotFound(vpn_id.to_string()))?;

        let new_status = VpnStatus::new(ConnectionState::Disconnecting, String::new());
        vpn.update_status(new_status);

        self.repository
            .save(&vpn)
            .await
            .map_err(|e| UseCaseError::RepositoryError(e.to_string()))?;

        Ok(vpn)
    }
}

pub struct ListVpnsUseCase {
    repository: Arc<dyn VpnRepository>,
}

impl ListVpnsUseCase {
    pub fn new(repository: Arc<dyn VpnRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self) -> Result<Vec<Vpn>, UseCaseError> {
        self.repository
            .list_all()
            .await
            .map_err(|e| UseCaseError::RepositoryError(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::Vpn;
    use async_trait::async_trait;
    use std::collections::HashMap;
    use std::sync::Mutex;

    pub struct MockVpnRepository {
        vpns: Mutex<HashMap<String, Vpn>>,
    }

    impl MockVpnRepository {
        pub fn new() -> Self {
            Self {
                vpns: Mutex::new(HashMap::new()),
            }
        }

        pub fn add_vpn(&self, vpn: Vpn) {
            let mut vpns = self.vpns.lock().unwrap();
            vpns.insert(vpn.id().to_string(), vpn);
        }
    }

    #[async_trait]
    impl VpnRepository for MockVpnRepository {
        async fn find_by_id(&self, id: &str) -> Result<Option<Vpn>, Box<dyn std::error::Error>> {
            let vpns = self.vpns.lock().unwrap();
            Ok(vpns.get(id).cloned())
        }

        async fn save(&self, vpn: &Vpn) -> Result<(), Box<dyn std::error::Error>> {
            let mut vpns = self.vpns.lock().unwrap();
            vpns.insert(vpn.id().to_string(), vpn.clone());
            Ok(())
        }

        async fn list_all(&self) -> Result<Vec<Vpn>, Box<dyn std::error::Error>> {
            let vpns = self.vpns.lock().unwrap();
            Ok(vpns.values().cloned().collect())
        }
    }
}