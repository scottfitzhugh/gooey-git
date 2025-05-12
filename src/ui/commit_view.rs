use libadwaita as adw;
use adw::prelude::*;
use gtk4::{Box, Label, ListBox, Orientation, ScrolledWindow, SelectionMode};

pub struct CommitView {
	container: Box,
}

impl CommitView {
	pub fn new() -> Self {
		// Create main container
		let container = Box::new(Orientation::Vertical, 0);
		
		// Create section title
		let title = Label::builder()
			.label("Commit History")
			.halign(gtk4::Align::Start)
			.margin_start(12)
			.margin_top(6)
			.margin_bottom(6)
			.build();
		
		container.append(&title);
		
		// Create commits list
		let commits_list = ListBox::builder()
			.selection_mode(SelectionMode::Single)
			.build();
		
		// Add scrollable container for commits
		let commits_scroll = ScrolledWindow::builder()
			.hscrollbar_policy(gtk4::PolicyType::Never)
			.vscrollbar_policy(gtk4::PolicyType::Automatic)
			.child(&commits_list)
			.build();
		
		container.append(&commits_scroll);
		
		Self { container }
	}
	
	pub fn widget(&self) -> Box {
		self.container.clone()
	}
} 