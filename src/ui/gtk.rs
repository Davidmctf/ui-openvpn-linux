use gtk4::prelude::*;
use gtk4::{glib, Application, ApplicationWindow, Box, Button, Label, ListBox, ScrolledWindow, Orientation};
use std::rc::Rc;
use std::cell::RefCell;
use std::process::Command;

const APP_ID: &str = "com.osmiodev.openvpn-manager";

pub fn run_gui() {
    let app = Application::builder()
        .application_id(APP_ID)
        .build();
    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    // Main window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("🔐 OpenVPN Manager")
        .default_width(500)
        .default_height(400)
        .build();

    // Main container
    let main_box = Box::new(Orientation::Vertical, 10);
    main_box.set_margin_top(10);
    main_box.set_margin_bottom(10);
    main_box.set_margin_start(10);
    main_box.set_margin_end(10);

    // Title
    let title = Label::new(Some("🔐 OpenVPN Manager"));
    title.add_css_class("title-1");
    main_box.append(&title);

    // Status label
    let status_label = Label::new(Some("🔴 Desconectado"));
    status_label.add_css_class("heading");
    main_box.append(&status_label);

    // VPN list container
    let list_frame = gtk4::Frame::new(Some("Conexiones VPN Disponibles"));
    let scrolled_window = ScrolledWindow::new();
    scrolled_window.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);
    scrolled_window.set_min_content_height(200);
    
    let vpn_list = ListBox::new();
    vpn_list.add_css_class("rich-list");
    scrolled_window.set_child(Some(&vpn_list));
    list_frame.set_child(Some(&scrolled_window));
    main_box.append(&list_frame);

    // Button container
    let button_box = Box::new(Orientation::Horizontal, 10);
    button_box.set_halign(gtk4::Align::Center);

    // Refresh button
    let refresh_btn = Button::with_label("🔄 Actualizar");
    refresh_btn.add_css_class("suggested-action");
    
    // Connect button
    let connect_btn = Button::with_label("🔌 Conectar");
    connect_btn.add_css_class("suggested-action");
    connect_btn.set_sensitive(false);
    
    // Disconnect button
    let disconnect_btn = Button::with_label("❌ Desconectar");
    disconnect_btn.add_css_class("destructive-action");
    
    // Import button
    let import_btn = Button::with_label("📥 Importar VPN");
    import_btn.add_css_class("suggested-action");

    button_box.append(&refresh_btn);
    button_box.append(&connect_btn);
    button_box.append(&disconnect_btn);
    button_box.append(&import_btn);
    
    main_box.append(&button_box);

    // Update status function
    let update_status = |status_label: &Label| {
        if let Ok(output) = Command::new("nmcli")
            .args(&["connection", "show", "--active"])
            .output()
        {
            let connections = String::from_utf8_lossy(&output.stdout);
            let mut found_active = false;
            
            for line in connections.lines().skip(1) {
                if !line.trim().is_empty() {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 3 && parts[2] == "vpn" {
                        status_label.set_text(&format!("🟢 Conectado a: {}", parts[0]));
                        found_active = true;
                        break;
                    }
                }
            }
            
            if !found_active {
                status_label.set_text("🔴 Desconectado");
            }
        }
    };

    // Store selected VPN name
    let selected_vpn: Rc<RefCell<Option<String>>> = Rc::new(RefCell::new(None));

    // Clone references for closures
    let vpn_list_clone = vpn_list.clone();
    let status_label_clone = status_label.clone();
    let connect_btn_clone = connect_btn.clone();
    let selected_vpn_clone = selected_vpn.clone();

    // Refresh VPN list function
    let refresh_vpns = {
        let vpn_list = vpn_list_clone.clone();
        let status_label = status_label_clone.clone();
        let connect_btn = connect_btn_clone.clone();
        let selected_vpn = selected_vpn_clone.clone();
        let update_status = update_status.clone();
        
        move || {
            // Clear existing list
            while let Some(child) = vpn_list.first_child() {
                vpn_list.remove(&child);
            }
            
            // Get VPN connections using nmcli
            if let Ok(output) = Command::new("nmcli")
                .args(&["connection", "show"])
                .output() 
            {
                let connections = String::from_utf8_lossy(&output.stdout);
                for line in connections.lines().skip(1) {
                    if !line.trim().is_empty() {
                        let parts: Vec<&str> = line.split_whitespace().collect();
                        if parts.len() >= 3 && parts[2] == "vpn" {
                            let vpn_name = parts[0].to_string();
                            
                            // Create list row
                            let row = gtk4::ListBoxRow::new();
                            let row_box = Box::new(Orientation::Horizontal, 10);
                            row_box.set_margin_top(5);
                            row_box.set_margin_bottom(5);
                            row_box.set_margin_start(10);
                            row_box.set_margin_end(10);
                            
                            let vpn_label = Label::new(Some(&format!("📋 {}", vpn_name)));
                            vpn_label.set_halign(gtk4::Align::Start);
                            row_box.append(&vpn_label);
                            
                            row.set_child(Some(&row_box));
                            vpn_list.append(&row);
                        }
                    }
                }
            }
            
            // Update status
            update_status(&status_label);
        }
    };

    // VPN list selection handler
    vpn_list.connect_row_selected({
        let connect_btn = connect_btn.clone();
        let selected_vpn = selected_vpn.clone();
        
        move |_, row| {
            if let Some(row) = row {
                connect_btn.set_sensitive(true);
                
                // Extract VPN name from the selected row
                if let Some(child) = row.child() {
                    if let Ok(row_box) = child.downcast::<Box>() {
                        if let Some(label_widget) = row_box.first_child() {
                            if let Ok(label) = label_widget.downcast::<Label>() {
                                let text = label.text();
                                if let Some(name) = text.strip_prefix("📋 ") {
                                    *selected_vpn.borrow_mut() = Some(name.to_string());
                                }
                            }
                        }
                    }
                }
            } else {
                connect_btn.set_sensitive(false);
                *selected_vpn.borrow_mut() = None;
            }
        }
    });

    // Refresh button handler
    refresh_btn.connect_clicked({
        let refresh_vpns = refresh_vpns.clone();
        move |_| {
            refresh_vpns();
        }
    });

    // Connect button handler
    connect_btn.connect_clicked({
        let selected_vpn = selected_vpn.clone();
        let status_label = status_label.clone();
        
        move |_| {
            if let Some(vpn_name) = selected_vpn.borrow().as_ref() {
                // Connect to VPN
                let _ = Command::new("nmcli")
                    .args(&["connection", "up", vpn_name])
                    .output();
                
                // Update status after connection attempt
                glib::timeout_add_seconds_local(1, {
                    let status_label = status_label.clone();
                    move || {
                        update_status(&status_label);
                        glib::ControlFlow::Break
                    }
                });
            }
        }
    });

    // Disconnect button handler
    disconnect_btn.connect_clicked({
        let status_label = status_label.clone();
        
        move |_| {
            // Get active VPN connections and disconnect them
            if let Ok(output) = Command::new("nmcli")
                .args(&["connection", "show", "--active"])
                .output()
            {
                let connections = String::from_utf8_lossy(&output.stdout);
                for line in connections.lines().skip(1) {
                    if !line.trim().is_empty() {
                        let parts: Vec<&str> = line.split_whitespace().collect();
                        if parts.len() >= 3 && parts[2] == "vpn" {
                            let _ = Command::new("nmcli")
                                .args(&["connection", "down", parts[0]])
                                .output();
                        }
                    }
                }
            }
            
            // Update status after disconnection
            glib::timeout_add_seconds_local(1, {
                let status_label = status_label.clone();
                move || {
                    update_status(&status_label);
                    glib::ControlFlow::Break
                }
            });
        }
    });

    // Import button handler
    import_btn.connect_clicked({
        let window = window.clone();
        let refresh_vpns = refresh_vpns.clone();
        
        move |_| {
            // Use FileChooserDialog for compatibility with older GTK4 versions
            let file_chooser = gtk4::FileChooserDialog::new(
                Some("Seleccionar archivo .ovpn"),
                Some(&window),
                gtk4::FileChooserAction::Open,
                &[("Cancelar", gtk4::ResponseType::Cancel), ("Abrir", gtk4::ResponseType::Accept)],
            );
            
            let filter = gtk4::FileFilter::new();
            filter.add_pattern("*.ovpn");
            filter.set_name(Some("Archivos OpenVPN (*.ovpn)"));
            file_chooser.add_filter(&filter);
            
            file_chooser.connect_response({
                let refresh_vpns = refresh_vpns.clone();
                move |dialog, response| {
                    if response == gtk4::ResponseType::Accept {
                        if let Some(file) = dialog.file() {
                            if let Some(path) = file.path() {
                                // Import the VPN file
                                let _ = Command::new("nmcli")
                                    .args(&["connection", "import", "type", "openvpn", "file", &path.to_string_lossy()])
                                    .output();
                                
                                // Refresh the list after import
                                glib::timeout_add_seconds_local(1, {
                                    let refresh_vpns = refresh_vpns.clone();
                                    move || {
                                        refresh_vpns();
                                        glib::ControlFlow::Break
                                    }
                                });
                            }
                        }
                    }
                    dialog.close();
                }
            });
            
            file_chooser.show();
        }
    });

    // Set up window content
    window.set_child(Some(&main_box));
    
    // Show window - this is critical for actually displaying the GUI
    window.present();
    
    // Initial refresh after window is shown
    refresh_vpns();
}