use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::cell::RefCell;

use crate::git::repository::Repository;
use crate::git::branch;
use crate::git::commit;
use crate::git::status;

pub struct RepositoryModel {
	repo: Option<Repository>,
	path: Option<PathBuf>,
	name: String,
}

impl RepositoryModel {
	pub fn new() -> Self {
		Self {
			repo: None,
			path: None,
			name: String::from("No Repository"),
		}
	}
	
	pub fn open<P: AsRef<Path>>(&mut self, path: P) -> Result<(), git2::Error> {
		let repo = Repository::discover(&path)?;
		
		let workdir = repo.workdir()
			.unwrap_or_else(|| repo.path())
			.to_path_buf();
			
		// Extract repository name from the path
		let name = workdir
			.file_name()
			.and_then(|name| name.to_str())
			.unwrap_or("Unknown")
			.to_string();
			
		self.repo = Some(repo);
		self.path = Some(workdir);
		self.name = name;
		
		Ok(())
	}
	
	pub fn close(&mut self) {
		self.repo = None;
		self.path = None;
		self.name = String::from("No Repository");
	}
	
	pub fn is_open(&self) -> bool {
		self.repo.is_some()
	}
	
	pub fn name(&self) -> &str {
		&self.name
	}
	
	pub fn path(&self) -> Option<&Path> {
		self.path.as_deref()
	}
	
	pub fn repo(&self) -> Option<&Repository> {
		self.repo.as_ref()
	}
} 