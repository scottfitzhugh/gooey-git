use libadwaita as adw;
use adw::HeaderBar;
use gtk4::prelude::*;
use gtk4::{Button, FileChooserAction, FileChooserDialog};
use std::rc::Rc;
use std::cell::RefCell;
use crate::models::repository_model::RepositoryModel;

pub struct Header {
	header_bar: HeaderBar,
	repo_model: Rc<RefCell<RepositoryModel>>,
}

impl Header {
	pub fn new(repo_model: Rc<RefCell<RepositoryModel>>) -> Self {
		// Create a header bar
		let header_bar = HeaderBar::new();
		
		// Create open repository button
		let open_btn = Button::builder()
			.icon_name("folder-open-symbolic")
			.tooltip_text("Open Repository")
			.build();
		
		// Clone the repo_model for the closure
		let repo_model_clone = repo_model.clone();
		
		// Connect the open button to the file chooser dialog
		open_btn.connect_clicked(move |_| {
			let dialog = FileChooserDialog::new(
				Some("Open Git Repository"),
				None::<&adw::Window>,
				FileChooserAction::SelectFolder,
				&[("Cancel", gtk4::ResponseType::Cancel), ("Open", gtk4::ResponseType::Accept)]
			);
			
			let repo_model_dialog = repo_model_clone.clone();
			dialog.connect_response(move |dialog, response| {
				if response == gtk4::ResponseType::Accept {
					if let Some(path) = dialog.file().and_then(|file| file.path()) {
						match repo_model_dialog.borrow_mut().open(&path) {
							Ok(_) => println!("Opened repository at {:?}", path),
							Err(err) => println!("Error opening repository: {}", err),
						}
					}
				}
				dialog.destroy();
			});
			
			dialog.show();
		});
		
		// Add open button to the start of the header
		header_bar.pack_start(&open_btn);
		
		// Create repository menu button
		let menu_btn = Button::builder()
			.icon_name("view-more-symbolic")
			.tooltip_text("Repository Actions")
			.build();
		
		// Add menu button to the end of the header
		header_bar.pack_end(&menu_btn);
		
		Self { 
			header_bar,
			repo_model
		}
	}
	
	pub fn widget(&self) -> HeaderBar {
		self.header_bar.clone()
	}
} 