use git2::{Status, StatusOptions, StatusShow};
use std::path::Path;

use super::repository::Repository;

pub struct StatusEntry {
	pub path: String,
	pub status: Status,
}

impl StatusEntry {
	pub fn is_new(&self) -> bool {
		self.status.is_wt_new()
	}
	
	pub fn is_modified(&self) -> bool {
		self.status.is_wt_modified()
	}
	
	pub fn is_deleted(&self) -> bool {
		self.status.is_wt_deleted()
	}
	
	pub fn is_renamed(&self) -> bool {
		self.status.is_wt_renamed()
	}
	
	pub fn is_staged(&self) -> bool {
		self.status.is_index_new() || 
		self.status.is_index_modified() || 
		self.status.is_index_deleted() || 
		self.status.is_index_renamed()
	}
}

pub fn get_status(repo: &Repository) -> Result<Vec<StatusEntry>, git2::Error> {
	let mut opts = StatusOptions::new();
	opts.include_untracked(true)
		.renames_head_to_index(true)
		.recurse_untracked_dirs(true)
		.show(StatusShow::IndexAndWorkdir);
	
	let statuses = repo.inner().statuses(Some(&mut opts))?;
	
	let mut result = Vec::new();
	
	for entry in statuses.iter() {
		let path = entry.path().unwrap_or("").to_string();
		let status = entry.status();
		
		result.push(StatusEntry { path, status });
	}
	
	Ok(result)
}

pub fn stage_file(repo: &Repository, path: &Path) -> Result<(), git2::Error> {
	let mut index = repo.inner().index()?;
	index.add_path(path)?;
	index.write()?;
	Ok(())
}

pub fn unstage_file(repo: &Repository, path: &Path) -> Result<(), git2::Error> {
	let head = repo.inner().head()?;
	let object = head.peel(git2::ObjectType::Commit)?;
	
	repo.inner().reset_default(Some(&object), &[path])?;
	
	Ok(())
} 