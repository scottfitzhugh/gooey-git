use std::path::Path;
use crate::git::branch::{self, Branch};
use crate::git::repository::Repository;
use git2::BranchType;

pub struct BranchModel {
	branches: Vec<String>,
	current_branch: Option<String>,
}

impl BranchModel {
	pub fn new() -> Self {
		Self {
			branches: Vec::new(),
			current_branch: None,
		}
	}
	
	pub fn update(&mut self, repo: &Repository) -> Result<(), git2::Error> {
		self.branches.clear();
		
		// Get all branches
		let branches = branch::get_branches(repo, Some(BranchType::Local))?;
		
		for branch in branches {
			if let Ok(Some(name)) = branch.name() {
				if branch.is_head() {
					self.current_branch = Some(name.to_string());
				}
				self.branches.push(name.to_string());
			}
		}
		
		// Sort branches alphabetically
		self.branches.sort();
		
		Ok(())
	}
	
	pub fn branches(&self) -> &[String] {
		&self.branches
	}
	
	pub fn current_branch(&self) -> Option<&String> {
		self.current_branch.as_ref()
	}
	
	pub fn checkout_branch(&self, repo: &Repository, branch_name: &str) -> Result<(), git2::Error> {
		branch::checkout_branch(repo, branch_name)
	}
} 