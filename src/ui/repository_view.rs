use libadwaita as adw;
use adw::prelude::*;
use gtk4::{Box, Label, ListBox, Orientation, ScrolledWindow, SelectionMode, ListBoxRow};
use std::rc::Rc;
use std::cell::RefCell;

use crate::models::repository_model::RepositoryModel;
use crate::models::branch_model::BranchModel;

pub struct RepositoryView {
	container: Box,
	repo_model: Rc<RefCell<RepositoryModel>>,
	branch_model: BranchModel,
	branches_list: ListBox,
	remotes_list: ListBox,
	placeholder: Label,
}

impl RepositoryView {
	pub fn new() -> Self {
		// Create the branch model
		let branch_model = BranchModel::new();
		
		// Create main container
		let container = Box::new(Orientation::Vertical, 0);
		
		// Create repository title label
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
		
		// Create a repository model but it will be replaced in set_repo_model
		let repo_model = Rc::new(RefCell::new(RepositoryModel::new()));
		
		Self { 
			container,
			repo_model,
			branch_model,
			branches_list,
			remotes_list,
			placeholder
		}
	}
	
	pub fn widget(&self) -> Box {
		self.container.clone()
	}
	
	pub fn set_repo_model(&mut self, repo_model: Rc<RefCell<RepositoryModel>>) {
		self.repo_model = repo_model;
		self.update_view();
	}
	
	pub fn update_view(&mut self) {
		// Clear existing items
		while let Some(child) = self.branches_list.first_child() {
			self.branches_list.remove(&child);
		}
		
		while let Some(child) = self.remotes_list.first_child() {
			self.remotes_list.remove(&child);
		}
		
		let repo_model = self.repo_model.borrow();
		
		if let Some(repo) = repo_model.repo() {
			// Hide the placeholder
			self.placeholder.set_visible(false);
			
			// Update branch information
			if let Ok(()) = self.branch_model.update(repo) {
				for branch_name in self.branch_model.branches() {
					let row = ListBoxRow::new();
					let label = Label::new(Some(branch_name));
					row.set_child(Some(&label));
					self.branches_list.append(&row);
				}
			}
			
			// TODO: Add remotes display
		} else {
			// Show the placeholder
			self.placeholder.set_visible(true);
		}
	}
} 