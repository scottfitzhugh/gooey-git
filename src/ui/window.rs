use libadwaita as adw;
use adw::prelude::*;
use adw::ApplicationWindow;
use gtk4::{Box, Orientation, Paned};

use super::header::Header;
use super::repository_view::RepositoryView;
use super::status_view::StatusView;

pub struct Window {
	window: ApplicationWindow,
}

impl Window {
	pub fn new(app: &gtk4::Application) -> Self {
		// Create the main window
		let window = ApplicationWindow::builder()
			.application(app)
			.title("GooeyGit")
			.default_width(900)
			.default_height(600)
			.build();

		// Create the header
		let header = Header::new();
		
		// Create the main layout container (vertical box)
		let main_box = Box::new(Orientation::Vertical, 0);
		
		// Add the header to the main box
		main_box.append(&header.widget());
		
		// Create a horizontal paned widget to divide the UI
		let paned = Paned::new(Orientation::Horizontal);
		paned.set_position(250); // Default position for the separator
		
		// Create repository view for the left side
		let repo_view = RepositoryView::new();
		paned.set_start_child(Some(&repo_view.widget()));
		
		// Create status view for the right side
		let status_view = StatusView::new();
		paned.set_end_child(Some(&status_view.widget()));
		
		// Add the paned widget to the main box
		main_box.append(&paned);
		
		// Set the main box as the content of the window
		window.set_content(Some(&main_box));
		
		Self { window }
	}
	
	pub fn present(&self) {
		self.window.present();
	}
} 