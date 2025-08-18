#[cfg(test)]
mod vpn_entity_tests {
    use ui_openvpn_linux::domain::entities::{Vpn, VpnStatus, ConnectionState};

    #[test]
    fn should_create_vpn_with_valid_data() {
        let vpn = Vpn::new(
            "test-vpn".to_string(),
            "Dynamic".to_string(),
            "/path/to/config.ovpn".to_string(),
        );

        assert_eq!(vpn.id(), "test-vpn");
        assert_eq!(vpn.display_name(), "Dynamic");
        assert_eq!(vpn.config_path(), "/path/to/config.ovpn");
        assert_eq!(vpn.status(), &VpnStatus::default());
    }

    #[test] 
    fn should_update_vpn_status() {
        let mut vpn = Vpn::new(
            "test-vpn".to_string(),
            "Dynamic".to_string(),
            "/path/to/config.ovpn".to_string(),
        );

        let new_status = VpnStatus::new(ConnectionState::Connected, "192.168.1.100".to_string());
        vpn.update_status(new_status.clone());

        assert_eq!(vpn.status(), &new_status);
    }

    #[test]
    fn should_validate_empty_id_returns_error() {
        let result = Vpn::try_new("".to_string(), "Display".to_string(), "/path".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn should_validate_empty_config_path_returns_error() {
        let result = Vpn::try_new("id".to_string(), "Display".to_string(), "".to_string());
        assert!(result.is_err());
    }
}

#[cfg(test)]
mod vpn_status_tests {
    use ui_openvpn_linux::domain::entities::{VpnStatus, ConnectionState};

    #[test]
    fn should_create_default_status() {
        let status = VpnStatus::default();
        assert_eq!(status.state(), &ConnectionState::Disconnected);
        assert_eq!(status.ip_address(), "");
    }

    #[test]
    fn should_create_connected_status() {
        let status = VpnStatus::new(ConnectionState::Connected, "10.0.0.1".to_string());
        assert_eq!(status.state(), &ConnectionState::Connected);
        assert_eq!(status.ip_address(), "10.0.0.1");
    }

    #[test]
    fn should_transition_states_correctly() {
        let mut status = VpnStatus::default();
        
        status.set_state(ConnectionState::Connecting);
        assert_eq!(status.state(), &ConnectionState::Connecting);
        
        status.set_state(ConnectionState::Connected);
        assert_eq!(status.state(), &ConnectionState::Connected);
        
        status.set_state(ConnectionState::Disconnected);
        assert_eq!(status.state(), &ConnectionState::Disconnected);
    }
}

#[cfg(test)]
mod connection_use_case_tests {
    use ui_openvpn_linux::domain::{
        entities::{Vpn, ConnectionState},
        use_cases::ConnectVpnUseCase,
        repositories::VpnRepository,
    };
    use std::sync::Arc;
    use mockall::mock;

    mock! {
        TestVpnRepository {}
        
        #[async_trait::async_trait]
        impl VpnRepository for TestVpnRepository {
            async fn find_by_id(&self, id: &str) -> Result<Option<Vpn>, Box<dyn std::error::Error>>;
            async fn save(&self, vpn: &Vpn) -> Result<(), Box<dyn std::error::Error>>;
            async fn list_all(&self) -> Result<Vec<Vpn>, Box<dyn std::error::Error>>;
        }
    }

    #[tokio::test]
    async fn should_connect_vpn_successfully() {
        let mut mock_repo = MockTestVpnRepository::new();
        let vpn = Vpn::new("test-vpn".to_string(), "Test".to_string(), "/path".to_string());
        
        mock_repo
            .expect_find_by_id()
            .with(mockall::predicate::eq("test-vpn"))
            .times(1)
            .returning(move |_| Ok(Some(vpn.clone())));
            
        mock_repo
            .expect_save()
            .times(1)
            .returning(|_| Ok(()));

        let use_case = ConnectVpnUseCase::new(Arc::new(mock_repo));
        let result = use_case.execute("test-vpn").await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn should_return_error_when_vpn_not_found() {
        let mut mock_repo = MockTestVpnRepository::new();
        
        mock_repo
            .expect_find_by_id()
            .with(mockall::predicate::eq("non-existent"))
            .times(1)
            .returning(|_| Ok(None));

        let use_case = ConnectVpnUseCase::new(Arc::new(mock_repo));
        let result = use_case.execute("non-existent").await;

        assert!(result.is_err());
    }
}