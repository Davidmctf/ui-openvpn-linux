use crate::domain::entities::Vpn;
use async_trait::async_trait;
use std::error::Error;

#[async_trait]
pub trait VpnRepository: Send + Sync {
    async fn find_by_id(&self, id: &str) -> Result<Option<Vpn>, Box<dyn Error>>;
    async fn save(&self, vpn: &Vpn) -> Result<(), Box<dyn Error>>;
    async fn list_all(&self) -> Result<Vec<Vpn>, Box<dyn Error>>;
}