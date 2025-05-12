use libadwaita as adw;
use adw::prelude::*;
use gtk4::{Box, Button, Entry, Label, ListBox, ListBoxRow, Orientation, ScrolledWindow, SelectionMode, Separator};
use gtk4::glib::clone;
use std::rc::Rc;
use std::cell::RefCell;
use std::path::Path;

use crate::models::repository_model::RepositoryModel;
use crate::models::status_model::StatusModel;

pub struct StatusView {
	container: Box,
	repo_model: Rc<RefCell<RepositoryModel>>,
	status_model: StatusModel,
	staged_list: ListBox,
	unstaged_list: ListBox,
	commit_message: Entry,
	commit_button: Button,
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
		
		// Add unstage button for staged files
		let unstage_btn = Button::builder()
			.label("Unstage Selected")
			.halign(gtk4::Align::End)
			.margin_top(6)
			.margin_end(12)
			.sensitive(false)
			.build();
			
		container.append(&unstage_btn);
		
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
		
		// Add stage button for unstaged files
		let stage_btn = Button::builder()
			.label("Stage Selected")
			.halign(gtk4::Align::End)
			.margin_top(6)
			.margin_end(12)
			.sensitive(false)
			.build();
			
		container.append(&stage_btn);
		
		// Add commit box at the bottom
		let commit_box = Box::new(Orientation::Vertical, 8);
		commit_box.set_margin_top(12);
		commit_box.set_margin_start(12);
		commit_box.set_margin_end(12);
		commit_box.set_margin_bottom(12);
		
		// Add commit message entry
		let commit_message = Entry::builder()
			.placeholder_text("Commit message")
			.build();
		commit_box.append(&commit_message);
		
		// Add commit button in a horizontal box for alignment
		let button_box = Box::new(Orientation::Horizontal, 0);
		button_box.set_halign(gtk4::Align::End);
		
		let commit_button = Button::builder()
			.label("Commit")
			.sensitive(false) // Disabled until files are staged
			.build();
			
		button_box.append(&commit_button);
		commit_box.append(&button_box);
		
		container.append(&commit_box);
		
		// Create the models - repository model will be replaced in set_repo_model
		let repo_model = Rc::new(RefCell::new(RepositoryModel::new()));
		let status_model = StatusModel::new();
		
		// Create the StatusView instance
		let status_view = Self { 
			container: container.clone(),
			repo_model: repo_model.clone(),
			status_model: status_model.clone(),
			staged_list: staged_list.clone(),
			unstaged_list: unstaged_list.clone(),
			commit_message: commit_message.clone(),
			commit_button: commit_button.clone()
		};
		
		// Clone widgets for closures
		let stage_btn_clone = stage_btn.clone();
		let unstage_btn_clone = unstage_btn.clone();
		let unstaged_list_for_stage = unstaged_list.clone();
		let staged_list_for_unstage = staged_list.clone();
		let staged_list_for_stage = staged_list.clone();
		let commit_button_for_stage = commit_button.clone();
		let commit_button_for_unstage = commit_button.clone();
		let commit_button_for_commit = commit_button.clone();
		let unstaged_list_for_unstage = unstaged_list.clone();
		let commit_message_for_commit = commit_message.clone();
		let staged_list_for_commit = staged_list.clone();
		let unstaged_list_for_commit = unstaged_list.clone();
		
		// Connect the stage button
		stage_btn.connect_clicked(clone!(@weak unstaged_list_for_stage, @weak repo_model, @strong status_model, @weak staged_list_for_stage, @weak commit_button_for_stage => move |_| {
			let selected = unstaged_list_for_stage.selected_rows();
			if selected.is_empty() {
				return;
			}
			
			let repo_model_ref = repo_model.borrow();
			if let Some(repo) = repo_model_ref.repo() {
				for row in selected {
					if let Some(child) = row.child() {
						if let Some(box_layout) = child.downcast_ref::<Box>() {
							if let Some(first_child) = box_layout.first_child() {
																	if let Some(label) = first_child.downcast_ref::<Label>() {
									let path_str = label.label();
									let path = Path::new(&path_str);
									if let Err(err) = status_model.stage_file(repo, path) {
										println!("Error staging file: {}", err);
									}
								}
							}
						}
					}
				}
				
				// Update the view after staging
				let mut status_model_clone = status_model.clone();
				if let Some(_repo) = repo_model_ref.repo() {
					if let Some(repo) = repo_model_ref.repo() {
						let _ = status_model_clone.update(repo);
					}
				}
				
				// Clear the lists
				while let Some(child) = staged_list_for_stage.first_child() {
					staged_list_for_stage.remove(&child);
				}
				
				while let Some(child) = unstaged_list_for_stage.first_child() {
					unstaged_list_for_stage.remove(&child);
				}
				
				// Repopulate the lists
				if let Some(_repo) = repo_model_ref.repo() {
					// Add staged files
					for item in status_model_clone.staged_items() {
						let row = ListBoxRow::new();
						let box_layout = Box::new(Orientation::Horizontal, 6);
						box_layout.set_margin_start(6);
						box_layout.set_margin_end(6);
						box_layout.set_margin_top(3);
						box_layout.set_margin_bottom(3);
						
						let file_label = Label::new(Some(&item.path));
						file_label.set_halign(gtk4::Align::Start);
						file_label.set_hexpand(true);
						file_label.set_xalign(0.0);
						
						let status_label = Label::new(Some(&item.status_text));
						status_label.set_halign(gtk4::Align::End);
						
						box_layout.append(&file_label);
						box_layout.append(&status_label);
						
						row.set_child(Some(&box_layout));
						staged_list_for_stage.append(&row);
					}
					
					// Add unstaged files
					for item in status_model_clone.unstaged_items() {
						let row = ListBoxRow::new();
						let box_layout = Box::new(Orientation::Horizontal, 6);
						box_layout.set_margin_start(6);
						box_layout.set_margin_end(6);
						box_layout.set_margin_top(3);
						box_layout.set_margin_bottom(3);
						
						let file_label = Label::new(Some(&item.path));
						file_label.set_halign(gtk4::Align::Start);
						file_label.set_hexpand(true);
						file_label.set_xalign(0.0);
						
						let status_label = Label::new(Some(&item.status_text));
						status_label.set_halign(gtk4::Align::End);
						
						box_layout.append(&file_label);
						box_layout.append(&status_label);
						
						row.set_child(Some(&box_layout));
						unstaged_list_for_stage.append(&row);
					}
					
					// Enable commit button if there are staged files
					commit_button_for_stage.set_sensitive(!status_model_clone.staged_items().is_empty());
				}
			}
		}));
		
		// Connect the unstage button
		unstage_btn.connect_clicked(clone!(@weak staged_list_for_unstage, @weak repo_model, @strong status_model, @weak unstaged_list_for_unstage, @weak commit_button_for_unstage => move |_| {
			let selected = staged_list_for_unstage.selected_rows();
			if selected.is_empty() {
				return;
			}
			
			let repo_model_ref = repo_model.borrow();
			if let Some(repo) = repo_model_ref.repo() {
				for row in selected {
					if let Some(child) = row.child() {
						if let Some(box_layout) = child.downcast_ref::<Box>() {
							if let Some(first_child) = box_layout.first_child() {
								if let Some(label) = first_child.downcast_ref::<Label>() {
									let path_str = label.label();
									let path = Path::new(&path_str);
									if let Err(err) = status_model.unstage_file(repo, path) {
										println!("Error unstaging file: {}", err);
									}
								}
							}
						}
					}
				}
				
				// Update the view after unstaging
				let mut status_model_clone = status_model.clone();
				if let Some(_repo) = repo_model_ref.repo() {
					if let Some(repo) = repo_model_ref.repo() {
						let _ = status_model_clone.update(repo);
					}
				}
				
				// Clear the lists
				while let Some(child) = staged_list_for_unstage.first_child() {
					staged_list_for_unstage.remove(&child);
				}
				
				while let Some(child) = unstaged_list_for_unstage.first_child() {
					unstaged_list_for_unstage.remove(&child);
				}
				
				// Repopulate the lists
				if let Some(_repo) = repo_model_ref.repo() {
					// Add staged files
					for item in status_model_clone.staged_items() {
						let row = ListBoxRow::new();
						let box_layout = Box::new(Orientation::Horizontal, 6);
						box_layout.set_margin_start(6);
						box_layout.set_margin_end(6);
						box_layout.set_margin_top(3);
						box_layout.set_margin_bottom(3);
						
						let file_label = Label::new(Some(&item.path));
						file_label.set_halign(gtk4::Align::Start);
						file_label.set_hexpand(true);
						file_label.set_xalign(0.0);
						
						let status_label = Label::new(Some(&item.status_text));
						status_label.set_halign(gtk4::Align::End);
						
						box_layout.append(&file_label);
						box_layout.append(&status_label);
						
						row.set_child(Some(&box_layout));
						staged_list_for_unstage.append(&row);
					}
					
					// Add unstaged files
					for item in status_model_clone.unstaged_items() {
						let row = ListBoxRow::new();
						let box_layout = Box::new(Orientation::Horizontal, 6);
						box_layout.set_margin_start(6);
						box_layout.set_margin_end(6);
						box_layout.set_margin_top(3);
						box_layout.set_margin_bottom(3);
						
						let file_label = Label::new(Some(&item.path));
						file_label.set_halign(gtk4::Align::Start);
						file_label.set_hexpand(true);
						file_label.set_xalign(0.0);
						
						let status_label = Label::new(Some(&item.status_text));
						status_label.set_halign(gtk4::Align::End);
						
						box_layout.append(&file_label);
						box_layout.append(&status_label);
						
						row.set_child(Some(&box_layout));
						unstaged_list_for_unstage.append(&row);
					}
					
					// Enable commit button if there are staged files
					commit_button_for_unstage.set_sensitive(!status_model_clone.staged_items().is_empty());
				}
			}
		}));
		
		// Connect the selection signal to enable/disable stage/unstage buttons
		unstaged_list.connect_selected_rows_changed(clone!(@weak stage_btn_clone => move |list_box| {
			stage_btn_clone.set_sensitive(!list_box.selected_rows().is_empty());
		}));
		
		staged_list.connect_selected_rows_changed(clone!(@weak unstage_btn_clone => move |list_box| {
			unstage_btn_clone.set_sensitive(!list_box.selected_rows().is_empty());
		}));
		
		// Connect the commit button
		commit_button.connect_clicked(clone!(@weak commit_message_for_commit, @weak repo_model, @strong status_model, @weak staged_list_for_commit, @weak unstaged_list_for_commit, @weak commit_button_for_commit => move |_| {
			let message = commit_message_for_commit.text().to_string();
			if message.is_empty() {
				// TODO: Show error dialog
				println!("Commit message cannot be empty");
				return;
			}
			
			let repo_model_ref = repo_model.borrow();
			if let Some(_repo) = repo_model_ref.repo() {
				// TODO: Implement commit functionality
				println!("Committing with message: {}", message);
				
				// Clear commit message after commit
				commit_message_for_commit.set_text("");
				
				// Update the view after commit
				let mut status_model_clone = status_model.clone();
				if let Some(_repo) = repo_model_ref.repo() {
					if let Some(repo) = repo_model_ref.repo() {
						let _ = status_model_clone.update(repo);
					}
				}
				
				// Clear the lists
				while let Some(child) = staged_list_for_commit.first_child() {
					staged_list_for_commit.remove(&child);
				}
				
				while let Some(child) = unstaged_list_for_commit.first_child() {
					unstaged_list_for_commit.remove(&child);
				}
				
				// Repopulate the lists
				if let Some(_repo) = repo_model_ref.repo() {
					// Add staged files
					for item in status_model_clone.staged_items() {
						let row = ListBoxRow::new();
						let box_layout = Box::new(Orientation::Horizontal, 6);
						box_layout.set_margin_start(6);
						box_layout.set_margin_end(6);
						box_layout.set_margin_top(3);
						box_layout.set_margin_bottom(3);
						
						let file_label = Label::new(Some(&item.path));
						file_label.set_halign(gtk4::Align::Start);
						file_label.set_hexpand(true);
						file_label.set_xalign(0.0);
						
						let status_label = Label::new(Some(&item.status_text));
						status_label.set_halign(gtk4::Align::End);
						
						box_layout.append(&file_label);
						box_layout.append(&status_label);
						
						row.set_child(Some(&box_layout));
						staged_list_for_commit.append(&row);
					}
					
					// Add unstaged files
					for item in status_model_clone.unstaged_items() {
						let row = ListBoxRow::new();
						let box_layout = Box::new(Orientation::Horizontal, 6);
						box_layout.set_margin_start(6);
						box_layout.set_margin_end(6);
						box_layout.set_margin_top(3);
						box_layout.set_margin_bottom(3);
						
						let file_label = Label::new(Some(&item.path));
						file_label.set_halign(gtk4::Align::Start);
						file_label.set_hexpand(true);
						file_label.set_xalign(0.0);
						
						let status_label = Label::new(Some(&item.status_text));
						status_label.set_halign(gtk4::Align::End);
						
						box_layout.append(&file_label);
						box_layout.append(&status_label);
						
						row.set_child(Some(&box_layout));
						unstaged_list_for_commit.append(&row);
					}
					
					// Enable commit button if there are staged files
					commit_button_for_commit.set_sensitive(!status_model_clone.staged_items().is_empty());
				}
			}
		}));
		
		status_view
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
		while let Some(child) = self.staged_list.first_child() {
			self.staged_list.remove(&child);
		}
		
		while let Some(child) = self.unstaged_list.first_child() {
			self.unstaged_list.remove(&child);
		}
		
		let repo_model = self.repo_model.borrow();
		
		if let Some(repo) = repo_model.repo() {
			// Update status information
			if let Ok(()) = self.status_model.update(repo) {
				// Add staged files
				for item in self.status_model.staged_items() {
					let row = ListBoxRow::new();
					let box_layout = Box::new(Orientation::Horizontal, 6);
					box_layout.set_margin_start(6);
					box_layout.set_margin_end(6);
					box_layout.set_margin_top(3);
					box_layout.set_margin_bottom(3);
					
					let file_label = Label::new(Some(&item.path));
					file_label.set_halign(gtk4::Align::Start);
					file_label.set_hexpand(true);
					file_label.set_xalign(0.0);
					
					let status_label = Label::new(Some(&item.status_text));
					status_label.set_halign(gtk4::Align::End);
					
					box_layout.append(&file_label);
					box_layout.append(&status_label);
					
					row.set_child(Some(&box_layout));
					self.staged_list.append(&row);
				}
				
				// Add unstaged files
				for item in self.status_model.unstaged_items() {
					let row = ListBoxRow::new();
					let box_layout = Box::new(Orientation::Horizontal, 6);
					box_layout.set_margin_start(6);
					box_layout.set_margin_end(6);
					box_layout.set_margin_top(3);
					box_layout.set_margin_bottom(3);
					
					let file_label = Label::new(Some(&item.path));
					file_label.set_halign(gtk4::Align::Start);
					file_label.set_hexpand(true);
					file_label.set_xalign(0.0);
					
					let status_label = Label::new(Some(&item.status_text));
					status_label.set_halign(gtk4::Align::End);
					
					box_layout.append(&file_label);
					box_layout.append(&status_label);
					
					row.set_child(Some(&box_layout));
					self.unstaged_list.append(&row);
				}
				
				// Enable commit button if there are staged files
				self.commit_button.set_sensitive(!self.status_model.staged_items().is_empty());
			}
		} else {
			// Disable commit button when no repository is open
			self.commit_button.set_sensitive(false);
		}
	}
} 