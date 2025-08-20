use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Button, Box, Orientation, Label};

const APP_ID: &str = "com.osmiodev.openvpn-manager";

pub fn run_simple_gui() {
    // Create application
    let application = Application::builder()
        .application_id(APP_ID)
        .build();

    // Connect to "activate" signal of `application`
    application.connect_activate(build_ui);

    // Run the application
    application.run();
}

fn build_ui(application: &Application) {
    // Create a window and set the application
    let window = ApplicationWindow::builder()
        .application(application)
        .title("🔐 OpenVPN Manager")
        .default_width(600)
        .default_height(400)
        .build();

    // Create a button with label
    let button = Button::with_label("¡Hola! La GUI funciona 🎉");
    button.set_margin_top(20);
    button.set_margin_bottom(20);
    button.set_margin_start(20);
    button.set_margin_end(20);

    // Create label
    let label = Label::new(Some("🔐 OpenVPN Manager GUI está funcionando"));
    
    // Create a vertical box
    let vbox = Box::new(Orientation::Vertical, 10);
    vbox.set_halign(gtk4::Align::Center);
    vbox.set_valign(gtk4::Align::Center);
    
    vbox.append(&label);
    vbox.append(&button);

    // Connect to "clicked" signal of `button`
    button.connect_clicked(move |_| {
        println!("¡Botón presionado!");
    });

    // Add the box to the window
    window.set_child(Some(&vbox));

    // Present window
    window.present();
}