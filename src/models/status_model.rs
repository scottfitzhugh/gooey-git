use std::path::Path;
use std::rc::Rc;
use std::cell::RefCell;

use crate::git::status::{self, StatusEntry};
use crate::git::repository::Repository;

#[derive(Clone)]
pub struct StatusItem {
	pub path: String,
	pub status_text: String,
	pub is_staged: bool,
}

#[derive(Clone)]
pub struct StatusModel {
	staged_items: Vec<StatusItem>,
	unstaged_items: Vec<StatusItem>,
}

impl StatusModel {
	pub fn new() -> Self {
		Self {
			staged_items: Vec::new(),
			unstaged_items: Vec::new(),
		}
	}
	
	pub fn update(&mut self, repo: &Repository) -> Result<(), git2::Error> {
		self.staged_items.clear();
		self.unstaged_items.clear();
		
		let status_entries = status::get_status(repo)?;
		
		for entry in status_entries {
			let status_text = status_to_text(&entry);
			
			let item = StatusItem {
				path: entry.path.clone(),
				status_text,
				is_staged: entry.is_staged(),
			};
			
			if entry.is_staged() {
				self.staged_items.push(item);
			} else {
				self.unstaged_items.push(item);
			}
		}
		
		// Sort items by path
		self.staged_items.sort_by(|a, b| a.path.cmp(&b.path));
		self.unstaged_items.sort_by(|a, b| a.path.cmp(&b.path));
		
		Ok(())
	}
	
	pub fn staged_items(&self) -> &[StatusItem] {
		&self.staged_items
	}
	
	pub fn unstaged_items(&self) -> &[StatusItem] {
		&self.unstaged_items
	}
	
	pub fn stage_file(&self, repo: &Repository, path: &Path) -> Result<(), git2::Error> {
		status::stage_file(repo, path)
	}
	
	pub fn unstage_file(&self, repo: &Repository, path: &Path) -> Result<(), git2::Error> {
		status::unstage_file(repo, path)
	}
}

fn status_to_text(entry: &StatusEntry) -> String {
	let mut text = String::new();
	
	let status = entry.status;
	
	// Index (staged) status
	if status.is_index_new() {
		text.push_str("New");
	} else if status.is_index_modified() {
		text.push_str("Modified");
	} else if status.is_index_deleted() {
		text.push_str("Deleted");
	} else if status.is_index_renamed() {
		text.push_str("Renamed");
	} else if status.is_index_typechange() {
		text.push_str("Type Changed");
	}
	
	// Working directory (unstaged) status
	if status.is_wt_new() {
		text.push_str("New");
	} else if status.is_wt_modified() {
		text.push_str("Modified");
	} else if status.is_wt_deleted() {
		text.push_str("Deleted");
	} else if status.is_wt_renamed() {
		text.push_str("Renamed");
	} else if status.is_wt_typechange() {
		text.push_str("Type Changed");
	}
	
	// Special statuses
	if status.is_conflicted() {
		text.push_str("Conflicted");
	} else if status.is_ignored() {
		text.push_str("Ignored");
	}
	
	if text.is_empty() {
		text.push_str("Unknown");
	}
	
	text
} 