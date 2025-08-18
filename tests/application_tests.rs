#[cfg(test)]
mod vpn_application_service_tests {
    use ui_openvpn_linux::{
        domain::{entities::Vpn, repositories::VpnRepository},
        application::services::VpnApplicationService,
        infrastructure::services::OpenVpnService,
    };
    use async_trait::async_trait;
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};

    struct MockVpnRepository {
        vpns: Mutex<HashMap<String, Vpn>>,
    }

    impl MockVpnRepository {
        fn new() -> Self {
            Self {
                vpns: Mutex::new(HashMap::new()),
            }
        }

        fn add_vpn(&self, vpn: Vpn) {
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

    #[tokio::test]
    async fn should_list_vpns() {
        let repo = Arc::new(MockVpnRepository::new());
        let openvpn_service = Arc::new(OpenVpnService::new());
        
        let vpn1 = Vpn::new("vpn1".to_string(), "VPN 1".to_string(), "/path1".to_string());
        let vpn2 = Vpn::new("vpn2".to_string(), "VPN 2".to_string(), "/path2".to_string());
        
        repo.add_vpn(vpn1.clone());
        repo.add_vpn(vpn2.clone());
        
        let service = VpnApplicationService::new(repo, openvpn_service);
        let result = service.list_vpns().await;
        
        assert!(result.is_ok());
        let vpns = result.unwrap();
        assert_eq!(vpns.len(), 2);
    }

    #[tokio::test]
    async fn should_return_error_when_connecting_nonexistent_vpn() {
        let repo = Arc::new(MockVpnRepository::new());
        let openvpn_service = Arc::new(OpenVpnService::new());
        
        let service = VpnApplicationService::new(repo, openvpn_service);
        let result = service.connect_vpn("nonexistent").await;
        
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn should_disconnect_current_connection() {
        let repo = Arc::new(MockVpnRepository::new());
        let openvpn_service = Arc::new(OpenVpnService::new());
        
        let service = VpnApplicationService::new(repo, openvpn_service);
        let result = service.disconnect_current().await;
        
        // Should succeed even if no connection exists
        assert!(result.is_ok());
    }
}

#[cfg(test)]
mod vpn_mapper_tests {
    use ui_openvpn_linux::{
        domain::entities::{Vpn, VpnStatus, ConnectionState},
        application::mappers::VpnMapper,
    };

    #[test]
    fn should_map_vpn_to_dto() {
        let vpn = Vpn::new(
            "test-vpn".to_string(),
            "Test VPN".to_string(),
            "/path/to/config.ovpn".to_string(),
        );

        let dto = VpnMapper::to_dto(&vpn);

        assert_eq!(dto.id, "test-vpn");
        assert_eq!(dto.display_name, "Test VPN");
        assert_eq!(dto.config_path, "/path/to/config.ovpn");
    }

    #[test]
    fn should_map_dto_to_vpn() {
        use ui_openvpn_linux::application::dtos::{VpnDto, VpnStatusDto, ConnectionStateDto};

        let dto = VpnDto {
            id: "test-vpn".to_string(),
            display_name: "Test VPN".to_string(),
            config_path: "/path/to/config.ovpn".to_string(),
            status: VpnStatusDto {
                state: ConnectionStateDto::Disconnected,
                ip_address: String::new(),
                connected_since: None,
            },
        };

        let vpn = VpnMapper::from_dto(&dto);

        assert_eq!(vpn.id(), "test-vpn");
        assert_eq!(vpn.display_name(), "Test VPN");
        assert_eq!(vpn.config_path(), "/path/to/config.ovpn");
    }

    #[test]
    fn should_map_vpn_list_to_dto_list() {
        let vpns = vec![
            Vpn::new("vpn1".to_string(), "VPN 1".to_string(), "/path1".to_string()),
            Vpn::new("vpn2".to_string(), "VPN 2".to_string(), "/path2".to_string()),
        ];

        let dtos = VpnMapper::to_dto_list(&vpns);

        assert_eq!(dtos.len(), 2);
        assert_eq!(dtos[0].id, "vpn1");
        assert_eq!(dtos[1].id, "vpn2");
    }
}