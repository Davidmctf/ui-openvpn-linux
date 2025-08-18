use crate::domain::{entities::Vpn, repositories::VpnRepository};
use async_trait::async_trait;
use std::error::Error;
use std::path::{Path, PathBuf};
use tokio::fs;

pub struct FileVpnRepository {
    config_dir: PathBuf,
}

impl FileVpnRepository {
    pub fn new(config_dir: PathBuf) -> Self {
        Self { config_dir }
    }

    pub fn from_home_dir() -> Self {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
        let config_dir = PathBuf::from(home).join(".connectvpn.conf");
        Self::new(config_dir)
    }

    fn map_display_name(filename: &str) -> String {
        match filename {
            "David_cruz.ovpn" => "Dynamic".to_string(),
            "julian.ovpn" => "Howden".to_string(),
            _ => "Unknown".to_string(),
        }
    }

    fn extract_id_from_filename(filename: &str) -> String {
        filename.trim_end_matches(".ovpn").to_string()
    }
}

#[async_trait]
impl VpnRepository for FileVpnRepository {
    async fn find_by_id(&self, id: &str) -> Result<Option<Vpn>, Box<dyn Error>> {
        let config_path = self.config_dir.join(format!("{}.ovpn", id));
        
        if !config_path.exists() {
            return Ok(None);
        }

        let filename = format!("{}.ovpn", id);
        let display_name = Self::map_display_name(&filename);
        
        let vpn = Vpn::new(
            id.to_string(),
            display_name,
            config_path.to_string_lossy().to_string(),
        );

        Ok(Some(vpn))
    }

    async fn save(&self, vpn: &Vpn) -> Result<(), Box<dyn Error>> {
        // For now, we only update in-memory state
        // In a real implementation, this might update a metadata file
        Ok(())
    }

    async fn list_all(&self) -> Result<Vec<Vpn>, Box<dyn Error>> {
        if !self.config_dir.exists() {
            return Ok(Vec::new());
        }

        let mut vpns = Vec::new();
        let mut entries = fs::read_dir(&self.config_dir).await?;

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if let Some(extension) = path.extension() {
                if extension == "ovpn" {
                    if let Some(filename) = path.file_name() {
                        let filename_str = filename.to_string_lossy().to_string();
                        let id = Self::extract_id_from_filename(&filename_str);
                        let display_name = Self::map_display_name(&filename_str);
                        
                        let vpn = Vpn::new(
                            id,
                            display_name,
                            path.to_string_lossy().to_string(),
                        );
                        vpns.push(vpn);
                    }
                }
            }
        }

        Ok(vpns)
    }
}