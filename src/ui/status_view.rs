use libadwaita as adw;
use adw::prelude::*;
use gtk4::{Box, Label, ListBox, Orientation, ScrolledWindow, SelectionMode, Separator};

pub struct StatusView {
	container: Box,
}

impl StatusView {
	pub fn new() -> Self {
		// Create main container
		let container = Box::new(Orientation::Vertical, 0);
		
		// Create section title
		let title = Label::builder()
			.label("Status")
			.halign(gtk4::Align::Start)
			.margin_start(12)
			.margin_top(6)
			.margin_bottom(6)
			.build();
		
		container.append(&title);
		
		// Create staged files section
		let staged_box = Box::new(Orientation::Vertical, 0);
		
		// Create staged files label
		let staged_label = Label::builder()
			.label("Staged Files")
			.halign(gtk4::Align::Start)
			.margin_start(12)
			.margin_top(6)
			.margin_bottom(6)
			.build();
		
		staged_box.append(&staged_label);
		
		// Create staged files list
		let staged_list = ListBox::builder()
			.selection_mode(SelectionMode::Multiple)
			.build();
		
		// Add scrollable container for staged files
		let staged_scroll = ScrolledWindow::builder()
			.hscrollbar_policy(gtk4::PolicyType::Never)
			.vscrollbar_policy(gtk4::PolicyType::Automatic)
			.min_content_height(150)
			.child(&staged_list)
			.build();
			
		staged_box.append(&staged_scroll);
		container.append(&staged_box);
		
		// Add separator
		let separator = Separator::new(Orientation::Horizontal);
		separator.set_margin_top(12);
		separator.set_margin_bottom(12);
		container.append(&separator);
		
		// Create unstaged files section
		let unstaged_box = Box::new(Orientation::Vertical, 0);
		
		// Create unstaged files label
		let unstaged_label = Label::builder()
			.label("Unstaged Files")
			.halign(gtk4::Align::Start)
			.margin_start(12)
			.margin_top(6)
			.margin_bottom(6)
			.build();
			
		unstaged_box.append(&unstaged_label);
		
		// Create unstaged files list
		let unstaged_list = ListBox::builder()
			.selection_mode(SelectionMode::Multiple)
			.build();
		
		// Add scrollable container for unstaged files
		let unstaged_scroll = ScrolledWindow::builder()
			.hscrollbar_policy(gtk4::PolicyType::Never)
			.vscrollbar_policy(gtk4::PolicyType::Automatic)
			.min_content_height(150)
			.child(&unstaged_list)
			.build();
			
		unstaged_box.append(&unstaged_scroll);
		container.append(&unstaged_box);
		
		// Add commit box at the bottom
		let commit_box = Box::new(Orientation::Horizontal, 8);
		commit_box.set_margin_top(12);
		commit_box.set_margin_start(12);
		commit_box.set_margin_end(12);
		commit_box.set_margin_bottom(12);
		
		// Add commit button
		let commit_button = gtk4::Button::builder()
			.label("Commit")
			.sensitive(false) // Disabled until files are staged
			.build();
			
		commit_box.append(&commit_button);
		
		container.append(&commit_box);
		
		Self { container }
	}
	
	pub fn widget(&self) -> Box {
		self.container.clone()
	}
} 