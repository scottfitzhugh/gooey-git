use libadwaita as adw;
use adw::prelude::*;
use gtk4::{Box, Label, ListBox, Orientation, ScrolledWindow, SelectionMode};

pub struct RepositoryView {
	container: Box,
}

impl RepositoryView {
	pub fn new() -> Self {
		// Create main container
		let container = Box::new(Orientation::Vertical, 0);
		
		// Create section title
		let title = Label::builder()
			.label("Repository")
			.halign(gtk4::Align::Start)
			.margin_start(12)
			.margin_top(6)
			.margin_bottom(6)
			.build();
		
		container.append(&title);
		
		// Create branches list
		let branches_box = Box::new(Orientation::Vertical, 0);
		
		// Create branch section label
		let branch_label = Label::builder()
			.label("Branches")
			.halign(gtk4::Align::Start)
			.margin_start(12)
			.margin_top(6)
			.margin_bottom(6)
			.build();
		
		branches_box.append(&branch_label);
		
		// Create branches list
		let branches_list = ListBox::builder()
			.selection_mode(SelectionMode::Single)
			.build();
		
		// Add scrollable container for branches
		let branches_scroll = ScrolledWindow::builder()
			.hscrollbar_policy(gtk4::PolicyType::Never)
			.vscrollbar_policy(gtk4::PolicyType::Automatic)
			.min_content_height(100)
			.child(&branches_list)
			.build();
			
		branches_box.append(&branches_scroll);
		container.append(&branches_box);
		
		// Create remotes section
		let remotes_box = Box::new(Orientation::Vertical, 0);
		
		// Create remotes section label
		let remotes_label = Label::builder()
			.label("Remotes")
			.halign(gtk4::Align::Start)
			.margin_start(12)
			.margin_top(18)
			.margin_bottom(6)
			.build();
			
		remotes_box.append(&remotes_label);
		
		// Create remotes list
		let remotes_list = ListBox::builder()
			.selection_mode(SelectionMode::Single)
			.build();
		
		// Add scrollable container for remotes
		let remotes_scroll = ScrolledWindow::builder()
			.hscrollbar_policy(gtk4::PolicyType::Never)
			.vscrollbar_policy(gtk4::PolicyType::Automatic)
			.min_content_height(100)
			.child(&remotes_list)
			.build();
			
		remotes_box.append(&remotes_scroll);
		container.append(&remotes_box);
		
		// Add placeholder text for when no repository is open
		let placeholder = Label::builder()
			.label("No repository open")
			.halign(gtk4::Align::Center)
			.valign(gtk4::Align::Center)
			.margin_top(20)
			.margin_bottom(20)
			.build();
			
		container.append(&placeholder);
		
		Self { container }
	}
	
	pub fn widget(&self) -> Box {
		self.container.clone()
	}
} 