use gtk4::prelude::*;
use gtk4::Application;
use libadwaita as adw;

mod ui;
mod git;
mod models;

use crate::ui::window::Window;

const APP_ID: &str = "org.gooey.git";

fn main() -> adw::glib::ExitCode {
	// Initialize GTK and libadwaita
	gtk4::init().expect("Failed to initialize GTK.");
	adw::init().expect("Failed to initialize libadwaita.");
	
	// Create a new application
	let app = Application::builder()
		.application_id(APP_ID)
		.build();
	
	// Connect to the activate signal
	app.connect_activate(|app| {
		let window = Window::new(app);
		window.present();
	});
	
	// Run the application
	app.run()
} 