#[cfg(feature = "gui")]
mod gtk_implementation {
    use crate::application::services::VpnApplicationService;
    use gtk4::prelude::*;
    use gtk4::{glib, Application, ApplicationWindow, Box as GtkBox, Button, Label, ListBox, Orientation, ScrolledWindow, FileChooserDialog, FileChooserAction, ResponseType};
    use std::sync::Arc;

    pub struct MainWindow {
        window: ApplicationWindow,
        vpn_service: Arc<VpnApplicationService>,
        vpn_list: ListBox,
        status_label: Label,
    }

    impl MainWindow {
        pub fn new(app: &Application, vpn_service: Arc<VpnApplicationService>) -> Self {
            // Create main window
            let window = ApplicationWindow::builder()
                .application(app)
                .title("🚀 UI OpenVPN Linux")
                .default_width(600)
                .default_height(500)
                .build();

            // Establecer icono de la ventana
            window.set_icon_name(Some("ui-openvpn-linux"));

            // Create header bar
            let header_bar = gtk4::HeaderBar::new();
            header_bar.set_title_widget(Some(&Label::new(Some("🚀 OpenVPN Manager"))));
            window.set_titlebar(Some(&header_bar));

            // Create main container
            let main_box = GtkBox::new(Orientation::Vertical, 12);
            main_box.set_margin_top(20);
            main_box.set_margin_bottom(20);
            main_box.set_margin_start(20);
            main_box.set_margin_end(20);

            // Status label
            let status_label = Label::new(Some("🔴 No VPN connections"));
            status_label.set_halign(gtk4::Align::Start);
            status_label.add_css_class("title-3");
            main_box.append(&status_label);

            // Separator
            let separator = gtk4::Separator::new(Orientation::Horizontal);
            main_box.append(&separator);

            // VPN list section
            let list_label = Label::new(Some("Available VPN Configurations:"));
            list_label.set_halign(gtk4::Align::Start);
            list_label.add_css_class("title-4");
            main_box.append(&list_label);

            // Scrolled window for VPN list
            let scrolled_window = ScrolledWindow::new();
            scrolled_window.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);
            scrolled_window.set_min_content_height(200);

            // VPN list
            let vpn_list = ListBox::new();
            vpn_list.set_selection_mode(gtk4::SelectionMode::None);
            vpn_list.add_css_class("boxed-list");
            scrolled_window.set_child(Some(&vpn_list));
            main_box.append(&scrolled_window);

            // Control buttons
            let button_box = GtkBox::new(Orientation::Horizontal, 12);
            button_box.set_halign(gtk4::Align::Center);

            let load_file_btn = Button::with_label("📁 Load VPN File");
            load_file_btn.add_css_class("suggested-action");

            let refresh_btn = Button::with_label("🔄 Refresh");
            refresh_btn.add_css_class("suggested-action");

            let disconnect_btn = Button::with_label("🔌 Kill All VPNs");
            disconnect_btn.add_css_class("destructive-action");

            button_box.append(&load_file_btn);
            button_box.append(&refresh_btn);
            button_box.append(&disconnect_btn);
            main_box.append(&button_box);

            window.set_child(Some(&main_box));

            let main_window = Self {
                window,
                vpn_service,
                vpn_list,
                status_label,
            };

            // Set up event handlers
            main_window.setup_events(load_file_btn, refresh_btn, disconnect_btn);
            main_window
        }

        fn setup_events(&self, load_file_btn: Button, refresh_btn: Button, disconnect_btn: Button) {
            let vpn_service_clone = Arc::clone(&self.vpn_service);
            let vpn_list_clone = self.vpn_list.clone();
            let status_label_clone = self.status_label.clone();

            // Refresh button event
            refresh_btn.connect_clicked(move |_| {
                let service = Arc::clone(&vpn_service_clone);
                let list = vpn_list_clone.clone();
                let status = status_label_clone.clone();
                
                glib::spawn_future_local(async move {
                    Self::refresh_vpn_list_async(service, list, status).await;
                });
            });

            let vpn_service_clone2 = Arc::clone(&self.vpn_service);
            let status_label_clone2 = self.status_label.clone();

            // Load file button event
            let window_clone = self.window.clone();
            let vpn_service_clone_load = Arc::clone(&self.vpn_service);
            let vpn_list_clone_load = self.vpn_list.clone();
            let status_label_clone_load = self.status_label.clone();

            load_file_btn.connect_clicked(move |_| {
                let dialog = FileChooserDialog::new(
                    Some("Select VPN Configuration File"),
                    Some(&window_clone),
                    FileChooserAction::Open,
                    &[("Cancel", ResponseType::Cancel), ("Open", ResponseType::Accept)],
                );

                // Set file filter for .ovpn files
                let filter = gtk4::FileFilter::new();
                filter.set_name(Some("OpenVPN Configuration Files"));
                filter.add_pattern("*.ovpn");
                dialog.add_filter(&filter);

                let service_load = Arc::clone(&vpn_service_clone_load);
                let list_load = vpn_list_clone_load.clone();
                let status_load = status_label_clone_load.clone();

                dialog.connect_response(move |dialog, response| {
                    if response == ResponseType::Accept {
                        if let Some(file) = dialog.file() {
                            if let Some(path) = file.path() {
                                let service = Arc::clone(&service_load);
                                let list = list_load.clone();
                                let status = status_load.clone();
                                let path_str = path.to_string_lossy().to_string();

                                glib::spawn_future_local(async move {
                                    Self::load_vpn_file_async(service, list, status, path_str).await;
                                });
                            }
                        }
                    }
                    dialog.close();
                });

                dialog.show();
            });

            // Disconnect button event - Force kill all VPN processes
            let vpn_service_clone2 = Arc::clone(&self.vpn_service);
            let status_label_clone2 = self.status_label.clone();
            let vpn_list_clone2 = self.vpn_list.clone();

            disconnect_btn.connect_clicked(move |_| {
                let service = Arc::clone(&vpn_service_clone2);
                let status = status_label_clone2.clone();
                let list = vpn_list_clone2.clone();
                
                glib::spawn_future_local(async move {
                    status.set_text("🔄 Killing all VPN processes...");
                    
                    match service.force_kill_all_vpns().await {
                        Ok(()) => {
                            status.set_text("✅ All VPN processes killed successfully");
                            // Refresh the list
                            Self::refresh_vpn_list_async(service, list, status.clone()).await;
                        },
                        Err(e) => {
                            status.set_text(&format!("❌ Kill failed: {}", e));
                        }
                    }
                });
            });

            // Initial load
            let service = Arc::clone(&self.vpn_service);
            let list = self.vpn_list.clone();
            let status = self.status_label.clone();
            
            glib::spawn_future_local(async move {
                Self::refresh_vpn_list_async(service.clone(), list.clone(), status.clone()).await;
            });

            // Auto-refresh timer (every 5 seconds)
            let service_timer = Arc::clone(&self.vpn_service);
            let list_timer = self.vpn_list.clone();
            let status_timer = self.status_label.clone();
            
            glib::timeout_add_seconds_local(5, move || {
                let service = Arc::clone(&service_timer);
                let list = list_timer.clone();
                let status = status_timer.clone();
                
                glib::spawn_future_local(async move {
                    Self::refresh_vpn_list_async(service, list, status).await;
                });
                
                glib::ControlFlow::Continue
            });
        }

        async fn refresh_vpn_list_async(
            service: Arc<VpnApplicationService>,
            list: ListBox,
            status_label: Label,
        ) {
            // Clear existing items
            while let Some(child) = list.first_child() {
                list.remove(&child);
            }

            match service.list_vpns().await {
                Ok(vpns) => {
                    if vpns.is_empty() {
                        let empty_label = Label::new(Some("No VPN configurations found"));
                        empty_label.set_margin_top(20);
                        empty_label.set_margin_bottom(20);
                        list.append(&empty_label);
                        status_label.set_text("❌ No VPN configurations");
                    } else {
                        let mut connected_count = 0;
                        
                        for vpn in vpns {
                            let row_box = GtkBox::new(Orientation::Horizontal, 12);
                            row_box.set_margin_top(12);
                            row_box.set_margin_bottom(12);
                            row_box.set_margin_start(12);
                            row_box.set_margin_end(12);

                            // Status icon
                            let status_icon = if vpn.is_connected() {
                                connected_count += 1;
                                "🟢"
                            } else {
                                "🔴"
                            };

                            let icon_label = Label::new(Some(status_icon));
                            
                            // VPN info
                            let info_box = GtkBox::new(Orientation::Vertical, 4);
                            let name_label = Label::new(Some(&vpn.display_name()));
                            name_label.set_halign(gtk4::Align::Start);
                            name_label.add_css_class("title-4");
                            
                            let id_label = Label::new(Some(&format!("ID: {}", vpn.id())));
                            id_label.set_halign(gtk4::Align::Start);
                            id_label.add_css_class("dim-label");
                            
                            info_box.append(&name_label);
                            info_box.append(&id_label);
                            info_box.set_hexpand(true);

                            // Connect button
                            let connect_btn = if vpn.is_connected() {
                                let btn = Button::with_label("🔌 Disconnect");
                                btn.add_css_class("destructive-action");
                                btn
                            } else {
                                let btn = Button::with_label("🚀 Connect (Auto-Kill Others)");
                                btn.add_css_class("suggested-action");
                                btn
                            };

                            // Button event handler
                            let vpn_id = vpn.id().to_string();
                            let service_clone = Arc::clone(&service);
                            let status_clone = status_label.clone();
                            let is_connected = vpn.is_connected();

                            let list_clone_for_refresh = list.clone();
                            let service_clone_for_refresh = Arc::clone(&service);
                            
                            connect_btn.connect_clicked(move |_| {
                                let service = Arc::clone(&service_clone);
                                let status = status_clone.clone();
                                let vpn_id = vpn_id.clone();
                                let list_refresh = list_clone_for_refresh.clone();
                                let service_refresh = Arc::clone(&service_clone_for_refresh);
                                
                                glib::spawn_future_local(async move {
                                    if is_connected {
                                        status.set_text(&format!("🔄 Disconnecting {}...", vpn_id));
                                        match service.force_kill_all_vpns().await {
                                            Ok(()) => {
                                                status.set_text(&format!("✅ Disconnected from {}", vpn_id));
                                                // Actualizar la GUI automáticamente
                                                Self::refresh_vpn_list_async(service_refresh, list_refresh, status.clone()).await;
                                            },
                                            Err(e) => {
                                                status.set_text(&format!("❌ Disconnect failed: {}", e));
                                            }
                                        }
                                    } else {
                                        // Mostrar estado de conectando
                                        status.set_text(&format!("🔄 Connecting to {}... (Auto-disconnecting others)", vpn_id));
                                        
                                        match service.connect_vpn(&vpn_id).await {
                                            Ok(()) => {
                                                status.set_text(&format!("✅ Connected to {} (others disconnected)", vpn_id));
                                                // Actualizar la GUI automáticamente
                                                Self::refresh_vpn_list_async(service_refresh, list_refresh, status.clone()).await;
                                            },
                                            Err(e) => {
                                                status.set_text(&format!("❌ Connect failed: {}", e));
                                            }
                                        }
                                    }
                                });
                            });

                            row_box.append(&icon_label);
                            row_box.append(&info_box);
                            row_box.append(&connect_btn);

                            list.append(&row_box);
                        }

                        // Update status
                        if connected_count > 0 {
                            status_label.set_text(&format!("🟢 {} VPN(s) connected", connected_count));
                        } else {
                            status_label.set_text("🔴 No active connections");
                        }
                    }
                },
                Err(e) => {
                    let error_label = Label::new(Some(&format!("Error: {}", e)));
                    error_label.set_margin_top(20);
                    error_label.set_margin_bottom(20);
                    list.append(&error_label);
                    status_label.set_text(&format!("❌ Error: {}", e));
                }
            }
        }

        async fn load_vpn_file_async(
            service: Arc<VpnApplicationService>,
            list: ListBox,
            status_label: Label,
            file_path: String,
        ) {
            status_label.set_text(&format!("📁 Loading VPN file: {}...", file_path));
            
            // Copy file to config directory
            match Self::copy_vpn_file_to_config(&file_path).await {
                Ok(new_path) => {
                    status_label.set_text(&format!("✅ VPN file loaded: {}", new_path));
                    // Refresh the list to show the new VPN
                    Self::refresh_vpn_list_async(service, list, status_label.clone()).await;
                },
                Err(e) => {
                    status_label.set_text(&format!("❌ Failed to load file: {}", e));
                }
            }
        }

        async fn copy_vpn_file_to_config(file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
            use std::path::Path;
            use tokio::fs;

            let source_path = Path::new(file_path);
            let file_name = source_path
                .file_name()
                .ok_or("Invalid file name")?;
            
            let home_dir = std::env::var("HOME").unwrap_or_else(|_| "/home".to_string());
            let config_dir = format!("{}/.connectvpn.conf", home_dir);
            
            // Create config directory if it doesn't exist
            fs::create_dir_all(&config_dir).await?;
            
            let dest_path = format!("{}/{}", config_dir, file_name.to_string_lossy());
            
            // Copy the file
            fs::copy(source_path, &dest_path).await?;
            
            Ok(dest_path)
        }

        pub fn show(&self) {
            self.window.present();
        }
    }
}

#[cfg(feature = "gui")]
pub use gtk_implementation::MainWindow;

#[cfg(not(feature = "gui"))]
pub struct MainWindow;

#[cfg(not(feature = "gui"))]
impl MainWindow {
    pub fn new(_app: &gtk4::Application, _service: std::sync::Arc<crate::application::services::VpnApplicationService>) -> Self {
        Self
    }
    
    pub fn show(&self) {}
}