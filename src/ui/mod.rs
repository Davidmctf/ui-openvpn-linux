#[cfg(feature = "web")]
pub mod web;

#[cfg(feature = "web")]
pub use web::run_web_gui;