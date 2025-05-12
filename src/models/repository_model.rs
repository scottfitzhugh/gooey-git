use std::path::{Path, PathBuf};
use std::cell::RefCell;

use crate::git::repository::Repository;

// Callback type for repository change notifications
pub type RepoChangeCallback = Box<dyn Fn() + 'static>;

pub struct RepositoryModel {
	repo: Option<Repository>,
	path: Option<PathBuf>,
	name: String,
	change_callbacks: RefCell<Vec<RepoChangeCallback>>,
}

impl RepositoryModel {
	pub fn new() -> Self {
		Self {
			repo: None,
			path: None,
			name: String::from("No Repository"),
			change_callbacks: RefCell::new(Vec::new()),
		}
	}
	
	// Register a callback to be notified when repository changes
	pub fn connect_changed<F>(&self, callback: F)
	where
		F: Fn() + 'static,
	{
		self.change_callbacks.borrow_mut().push(Box::new(callback));
	}
	
	// Notify all registered callbacks that the repository has changed
	fn notify_changed(&self) {
		// We can't clone the callbacks, so we need to call them while borrowing
		let callbacks = self.change_callbacks.borrow();
		for callback in callbacks.iter() {
			callback();
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
		
		// Notify listeners that the repository has changed
		self.notify_changed();
		
		Ok(())
	}
	
	pub fn close(&mut self) {
		self.repo = None;
		self.path = None;
		self.name = String::from("No Repository");
		
		// Notify listeners that the repository has changed
		self.notify_changed();
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