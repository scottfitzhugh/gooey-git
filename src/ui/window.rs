use libadwaita as adw;
use adw::prelude::*;
use adw::ApplicationWindow;
use gtk4::{Box, Orientation, Paned};
use std::rc::Rc;
use std::cell::RefCell;
use gtk4::glib;

use crate::models::repository_model::RepositoryModel;
use super::header::Header;
use super::repository_view::RepositoryView;
use super::status_view::StatusView;

pub struct Window {
	window: ApplicationWindow,
	repo_model: Rc<RefCell<RepositoryModel>>,
	repo_view: Rc<RefCell<RepositoryView>>,
	status_view: Rc<RefCell<StatusView>>,
}

impl Window {
	pub fn new(app: &gtk4::Application) -> Self {
		// Create the repository model
		let repo_model = Rc::new(RefCell::new(RepositoryModel::new()));
		
		// Create the main window
		let window = ApplicationWindow::builder()
			.application(app)
			.title("GooeyGit")
			.default_width(900)
			.default_height(600)
			.build();

		// Create the header and pass the repository model
		let header = Header::new(repo_model.clone());
		
		// Create the main layout container (vertical box)
		let main_box = Box::new(Orientation::Vertical, 0);
		
		// Add the header to the main box
		main_box.append(&header.widget());
		
		// Create a horizontal paned widget to divide the UI
		let paned = Paned::new(Orientation::Horizontal);
		paned.set_position(250); // Default position for the separator
		
		// Create repository view for the left side
		let mut repo_view = RepositoryView::new();
		repo_view.set_repo_model(repo_model.clone());
		let repo_view = Rc::new(RefCell::new(repo_view));
		paned.set_start_child(Some(&repo_view.borrow().widget()));
		
		// Create status view for the right side
		let mut status_view = StatusView::new();
		status_view.set_repo_model(repo_model.clone());
		let status_view = Rc::new(RefCell::new(status_view));
		paned.set_end_child(Some(&status_view.borrow().widget()));
		
		// Add the paned widget to the main box
		main_box.append(&paned);
		
		// Set the main box as the content of the window
		window.set_content(Some(&main_box));
		
		// Store everything in the Window struct
		let win = Self { 
			window: window.clone(),
			repo_model: repo_model.clone(),
			repo_view: repo_view.clone(),
			status_view: status_view.clone(),
		};
		
		// Setup a signal handler for repository changes to update the window title
		{
			let window_weak = glib::WeakRef::new();
			window_weak.set(Some(&window));
			let repo_model_weak = Rc::downgrade(&repo_model);
			
			repo_model.borrow().connect_changed(move || {
				if let Some(window) = window_weak.upgrade() {
					if let Some(repo_model) = repo_model_weak.upgrade() {
						let repo_name = {
							let repo_model_ref = repo_model.borrow();
							repo_model_ref.name().to_string()
						};
						
						let title = if repo_name == "No Repository" {
							"GooeyGit".to_string()
						} else {
							format!("GooeyGit - {}", repo_name)
						};
						window.set_title(Some(&title));
					}
				}
			});
		}
		
		// Setup a signal handler for repository changes to update the repository view
		{
			let repo_view_weak = Rc::downgrade(&repo_view);
			let repo_model_weak = Rc::downgrade(&repo_model);
			
			repo_model.borrow().connect_changed(move || {
				if let Some(repo_view) = repo_view_weak.upgrade() {
					repo_view.borrow_mut().update_view();
				}
			});
		}
		
		// Setup a signal handler for repository changes to update the status view
		{
			let status_view_weak = Rc::downgrade(&status_view);
			let repo_model_weak = Rc::downgrade(&repo_model);
			
			repo_model.borrow().connect_changed(move || {
				if let Some(status_view) = status_view_weak.upgrade() {
					status_view.borrow_mut().update_view();
				}
			});
		}
		
		win
	}
	
	pub fn present(&self) {
		self.window.present();
	}
} 