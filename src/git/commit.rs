use git2::{Commit as Git2Commit, Error, ObjectType, Oid, Time};
use std::fmt;

use super::repository::Repository;

pub struct Commit<'repo> {
	commit: Git2Commit<'repo>,
}

impl<'repo> Commit<'repo> {
	pub fn from_git2_commit(commit: Git2Commit<'repo>) -> Self {
		Self { commit }
	}
	
	pub fn id(&self) -> Oid {
		self.commit.id()
	}
	
	pub fn short_id(&self) -> Result<String, Error> {
		let oid = self.commit.id();
		Ok(oid.to_string()[..7].to_string())
	}
	
	pub fn summary(&self) -> Option<&str> {
		self.commit.summary()
	}
	
	pub fn message(&self) -> Option<&str> {
		self.commit.message()
	}
	
	pub fn time(&self) -> Time {
		self.commit.time()
	}
	
	pub fn author(&self) -> git2::Signature {
		self.commit.author()
	}
	
	pub fn committer(&self) -> git2::Signature {
		self.commit.committer()
	}
	
	pub fn parent_count(&self) -> usize {
		self.commit.parent_count()
	}
	
	pub fn parent_id(&self, i: usize) -> Result<Oid, Error> {
		Ok(self.commit.parent_id(i)?)
	}
	
	pub fn parent(&self, i: usize) -> Result<Commit<'repo>, Error> {
		let parent = self.commit.parent(i)?;
		Ok(Commit::from_git2_commit(parent))
	}
}

impl<'repo> fmt::Display for Commit<'repo> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let id = match self.short_id() {
			Ok(id) => id,
			Err(_) => String::from("unknown"),
		};
		
		let summary = self.summary().unwrap_or("no summary");
		write!(f, "{} - {}", id, summary)
	}
}

pub fn get_commit_history(repo: &Repository, count: usize) -> Result<Vec<Commit>, Error> {
	let mut revwalk = repo.inner().revwalk()?;
	revwalk.push_head()?;
	
	let mut commits = Vec::new();
	
	for (i, oid_result) in revwalk.enumerate() {
		if i >= count {
			break;
		}
		
		if let Ok(oid) = oid_result {
			let commit = repo.inner().find_commit(oid)?;
			commits.push(Commit::from_git2_commit(commit));
		}
	}
	
	Ok(commits)
}

pub fn create_commit(
	repo: &Repository,
	message: &str,
	tree: &git2::Tree,
	parents: &[&git2::Commit],
) -> Result<Oid, Error> {
	let sig = repo.inner().signature()?;
	repo.inner().commit(Some("HEAD"), &sig, &sig, message, tree, parents)
}

pub fn get_commit_by_id(repo: &Repository, id: Oid) -> Result<Commit, Error> {
	let commit = repo.inner().find_commit(id)?;
	Ok(Commit::from_git2_commit(commit))
} 