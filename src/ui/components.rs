#[cfg(feature = "ui")]
mod gtk_components {
    use gtk4::prelude::*;
    use gtk4::{Application, ApplicationWindow, Box, Button, Label, ListBox, ListBoxRow, Orientation};

    pub struct VpnListWidget {
        container: Box,
        list_box: ListBox,
    }

    impl VpnListWidget {
        pub fn new() -> Self {
            let container = Box::new(Orientation::Vertical, 12);
            container.set_margin_top(20);
            container.set_margin_bottom(20);
            container.set_margin_start(20);
            container.set_margin_end(20);

            let title = Label::new(Some("🔒 VPNs Disponibles"));
            title.set_markup("<span size='large' weight='bold'>🔒 VPNs Disponibles</span>");
            container.append(&title);

            let list_box = ListBox::new();
            list_box.set_selection_mode(gtk4::SelectionMode::Single);
            list_box.add_css_class("boxed-list");
            
            container.append(&list_box);

            Self {
                container,
                list_box,
            }
        }

        pub fn container(&self) -> &Box {
            &self.container
        }

        pub fn add_vpn_item(&self, vpn_id: &str, display_name: &str, is_connected: bool) {
            let row = ListBoxRow::new();
            let row_box = Box::new(Orientation::Horizontal, 12);
            row_box.set_margin_top(12);
            row_box.set_margin_bottom(12);
            row_box.set_margin_start(12);
            row_box.set_margin_end(12);

            let status_icon = if is_connected { "🟢" } else { "🔴" };
            let status_label = Label::new(Some(status_icon));
            
            let name_label = Label::new(Some(display_name));
            name_label.set_halign(gtk4::Align::Start);
            name_label.set_hexpand(true);

            let connect_button = Button::with_label(if is_connected { "Desconectar" } else { "Conectar" });
            connect_button.add_css_class("suggested-action");

            row_box.append(&status_label);
            row_box.append(&name_label);
            row_box.append(&connect_button);

            row.set_child(Some(&row_box));
            self.list_box.append(&row);
        }

        pub fn clear(&self) {
            while let Some(child) = self.list_box.first_child() {
                self.list_box.remove(&child);
            }
        }
    }

    pub struct StatusWidget {
        container: Box,
        status_label: Label,
        ip_label: Label,
    }

    impl StatusWidget {
        pub fn new() -> Self {
            let container = Box::new(Orientation::Vertical, 6);
            container.set_margin_top(12);
            container.set_margin_bottom(12);
            container.set_margin_start(12);
            container.set_margin_end(12);

            let status_label = Label::new(Some("🔴 Desconectado"));
            status_label.set_halign(gtk4::Align::Start);
            
            let ip_label = Label::new(Some("IP: No disponible"));
            ip_label.set_halign(gtk4::Align::Start);
            ip_label.add_css_class("dim-label");

            container.append(&status_label);
            container.append(&ip_label);

            Self {
                container,
                status_label,
                ip_label,
            }
        }

        pub fn container(&self) -> &Box {
            &self.container
        }

        pub fn update_status(&self, is_connected: bool, vpn_name: Option<&str>, ip: Option<&str>) {
            let status_text = if is_connected {
                if let Some(name) = vpn_name {
                    format!("🟢 Conectado a {}", name)
                } else {
                    "🟢 Conectado".to_string()
                }
            } else {
                "🔴 Desconectado".to_string()
            };

            let ip_text = if let Some(ip_addr) = ip {
                format!("IP: {}", ip_addr)
            } else {
                "IP: No disponible".to_string()
            };

            self.status_label.set_text(&status_text);
            self.ip_label.set_text(&ip_text);
        }
    }
}