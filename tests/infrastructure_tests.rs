#[cfg(test)]
mod file_vpn_repository_tests {
    use ui_openvpn_linux::{
        domain::{entities::Vpn, repositories::VpnRepository},
        infrastructure::repositories::FileVpnRepository,
    };
    use std::path::PathBuf;
    use std::fs;
    use tempfile::TempDir;

    fn create_temp_vpn_config(temp_dir: &TempDir, name: &str, content: &str) -> PathBuf {
        let config_path = temp_dir.path().join(format!("{}.ovpn", name));
        fs::write(&config_path, content).unwrap();
        config_path
    }

    #[tokio::test]
    async fn should_find_vpn_by_id() {
        let temp_dir = TempDir::new().unwrap();
        let _config_path = create_temp_vpn_config(&temp_dir, "test-vpn", "test config content");
        
        let repo = FileVpnRepository::new(temp_dir.path().to_path_buf());
        let result = repo.find_by_id("test-vpn").await;

        assert!(result.is_ok());
        let vpn = result.unwrap();
        assert!(vpn.is_some());
        let vpn = vpn.unwrap();
        assert_eq!(vpn.id(), "test-vpn");
    }

    #[tokio::test]
    async fn should_return_none_when_vpn_not_found() {
        let temp_dir = TempDir::new().unwrap();
        let repo = FileVpnRepository::new(temp_dir.path().to_path_buf());
        
        let result = repo.find_by_id("non-existent").await;
        
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[tokio::test]
    async fn should_list_all_vpns() {
        let temp_dir = TempDir::new().unwrap();
        let _config1 = create_temp_vpn_config(&temp_dir, "David_cruz", "config 1");
        let _config2 = create_temp_vpn_config(&temp_dir, "julian", "config 2");
        
        let repo = FileVpnRepository::new(temp_dir.path().to_path_buf());
        let result = repo.list_all().await;

        assert!(result.is_ok());
        let vpns = result.unwrap();
        assert_eq!(vpns.len(), 2);
        
        let mut vpn_ids: Vec<&str> = vpns.iter().map(|v| v.id()).collect();
        vpn_ids.sort();
        assert_eq!(vpn_ids, vec!["David_cruz", "julian"]);
    }

    #[tokio::test]
    async fn should_map_display_names_correctly() {
        let temp_dir = TempDir::new().unwrap();
        let _config1 = create_temp_vpn_config(&temp_dir, "David_cruz", "config 1");
        let _config2 = create_temp_vpn_config(&temp_dir, "julian", "config 2");
        
        let repo = FileVpnRepository::new(temp_dir.path().to_path_buf());
        let result = repo.list_all().await;

        assert!(result.is_ok());
        let vpns = result.unwrap();
        
        for vpn in vpns {
            match vpn.id() {
                "David_cruz" => assert_eq!(vpn.display_name(), "Dynamic"),
                "julian" => assert_eq!(vpn.display_name(), "Howden"),
                _ => panic!("Unexpected VPN ID"),
            }
        }
    }
}

#[cfg(test)]
mod openvpn_service_tests {
    use ui_openvpn_linux::infrastructure::services::OpenVpnService;

    #[tokio::test]
    async fn should_create_openvpn_service() {
        let service = OpenVpnService::new();
        assert!(!service.is_connected().await);
    }

    #[test]
    fn should_build_openvpn_command() {
        let service = OpenVpnService::new();
        let config_path = "/path/to/config.ovpn";
        let args = service.build_openvpn_args(config_path);
        
        assert_eq!(args, vec!["--config".to_string(), "/path/to/config.ovpn".to_string()]);
    }
}