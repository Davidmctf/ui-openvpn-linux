#[cfg(feature = "gui")]
pub mod gtk;
#[cfg(feature = "gui")]
pub mod simple;

#[cfg(feature = "web")]
pub mod web;

#[cfg(feature = "gui")]
pub use gtk::run_gui;
#[cfg(feature = "gui")]
pub use simple::run_simple_gui;

#[cfg(feature = "web")]
pub use web::run_web_gui;