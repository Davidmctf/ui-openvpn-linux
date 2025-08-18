use crate::domain::entities::{Vpn, VpnStatus, ConnectionState};
use crate::application::dtos::{VpnDto, VpnStatusDto, ConnectionStateDto};

pub struct VpnMapper;

impl VpnMapper {
    pub fn to_dto(vpn: &Vpn) -> VpnDto {
        VpnDto {
            id: vpn.id().to_string(),
            display_name: vpn.display_name().to_string(),
            config_path: vpn.config_path().to_string(),
            status: Self::status_to_dto(vpn.status()),
        }
    }

    pub fn from_dto(dto: &VpnDto) -> Vpn {
        let mut vpn = Vpn::new(
            dto.id.clone(),
            dto.display_name.clone(),
            dto.config_path.clone(),
        );
        
        let status = Self::status_from_dto(&dto.status);
        vpn.update_status(status);
        vpn
    }

    pub fn to_dto_list(vpns: &[Vpn]) -> Vec<VpnDto> {
        vpns.iter().map(Self::to_dto).collect()
    }

    fn status_to_dto(status: &VpnStatus) -> VpnStatusDto {
        VpnStatusDto {
            state: Self::connection_state_to_dto(status.state()),
            ip_address: status.ip_address().to_string(),
            connected_since: status.connected_since()
                .map(|time| {
                    // Convert SystemTime to ISO 8601 string
                    match time.duration_since(std::time::UNIX_EPOCH) {
                        Ok(duration) => {
                            format!("{}", duration.as_secs())
                        },
                        Err(_) => "unknown".to_string(),
                    }
                }),
        }
    }

    fn status_from_dto(dto: &VpnStatusDto) -> VpnStatus {
        let state = Self::connection_state_from_dto(&dto.state);
        VpnStatus::new(state, dto.ip_address.clone())
    }

    fn connection_state_to_dto(state: &ConnectionState) -> ConnectionStateDto {
        match state {
            ConnectionState::Disconnected => ConnectionStateDto::Disconnected,
            ConnectionState::Connecting => ConnectionStateDto::Connecting,
            ConnectionState::Connected => ConnectionStateDto::Connected,
            ConnectionState::Disconnecting => ConnectionStateDto::Disconnecting,
            ConnectionState::Error(msg) => ConnectionStateDto::Error(msg.clone()),
        }
    }

    fn connection_state_from_dto(dto: &ConnectionStateDto) -> ConnectionState {
        match dto {
            ConnectionStateDto::Disconnected => ConnectionState::Disconnected,
            ConnectionStateDto::Connecting => ConnectionState::Connecting,
            ConnectionStateDto::Connected => ConnectionState::Connected,
            ConnectionStateDto::Disconnecting => ConnectionState::Disconnecting,
            ConnectionStateDto::Error(msg) => ConnectionState::Error(msg.clone()),
        }
    }
}