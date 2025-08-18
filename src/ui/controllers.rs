#[cfg(feature = "ui")]
mod gtk_controllers {
    use crate::application::services::VpnApplicationService;
    use crate::ui::components::{VpnListWidget, StatusWidget};
    use gtk4::prelude::*;
    use gtk4::{Application, ApplicationWindow, Box, HeaderBar, Orientation};
    use std::sync::Arc;

    pub struct MainController {
        window: ApplicationWindow,
        vpn_service: Arc<VpnApplicationService>,
        vpn_list: VpnListWidget,
        status_widget: StatusWidget,
    }

    impl MainController {
        pub fn new(app: &Application, vpn_service: Arc<VpnApplicationService>) -> Self {
            let window = ApplicationWindow::builder()
                .application(app)
                .title("UI OpenVPN Linux")
                .default_width(600)
                .default_height(500)
                .build();

            // Create header bar
            let header_bar = HeaderBar::new();
            header_bar.set_title_widget(Some(&gtk4::Label::new(Some("🔒 OpenVPN Manager"))));
            window.set_titlebar(Some(&header_bar));

            // Create main container
            let main_box = Box::new(Orientation::Vertical, 0);
            window.set_child(Some(&main_box));

            // Create status widget
            let status_widget = StatusWidget::new();
            main_box.append(status_widget.container());

            // Add separator
            let separator = gtk4::Separator::new(Orientation::Horizontal);
            main_box.append(&separator);

            // Create VPN list widget
            let vpn_list = VpnListWidget::new();
            main_box.append(vpn_list.container());

            Self {
                window,
                vpn_service,
                vpn_list,
                status_widget,
            }
        }

        pub async fn initialize(&self) -> Result<(), Box<dyn std::error::Error>> {
            self.refresh_vpn_list().await?;
            self.update_status().await?;
            Ok(())
        }

        pub async fn refresh_vpn_list(&self) -> Result<(), Box<dyn std::error::Error>> {
            let vpns = self.vpn_service.list_vpns().await?;
            
            self.vpn_list.clear();
            
            for vpn in vpns {
                let is_connected = vpn.is_connected();
                self.vpn_list.add_vpn_item(
                    vpn.id(),
                    vpn.display_name(),
                    is_connected,
                );
            }
            
            Ok(())
        }

        pub async fn update_status(&self) -> Result<(), Box<dyn std::error::Error>> {
            let vpns = self.vpn_service.get_connection_status().await?;
            
            let connected_vpn = vpns.iter().find(|v| v.is_connected());
            
            if let Some(vpn) = connected_vpn {
                self.status_widget.update_status(
                    true,
                    Some(vpn.display_name()),
                    Some(vpn.status().ip_address()),
                );
            } else {
                self.status_widget.update_status(false, None, None);
            }
            
            Ok(())
        }

        pub fn show(&self) {
            self.window.present();
        }
    }
}