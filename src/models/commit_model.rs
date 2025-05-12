use crate::git::commit::{self, Commit};
use crate::git::repository::Repository;

pub struct CommitItem {
	pub id: String,
	pub summary: String,
	pub author: String,
	pub date: String,
}

pub struct CommitModel {
	commits: Vec<CommitItem>,
}

impl CommitModel {
	pub fn new() -> Self {
		Self {
			commits: Vec::new(),
		}
	}
	
	pub fn update(&mut self, repo: &Repository, count: usize) -> Result<(), git2::Error> {
		self.commits.clear();
		
		let git_commits = commit::get_commit_history(repo, count)?;
		
		for commit in git_commits {
			let id = commit.short_id()?;
			let summary = commit.summary().unwrap_or("").to_string();
			let author = commit.author().name().unwrap_or("Unknown").to_string();
			
			// Format commit date using updated chrono methods
			let time = commit.time();
			let datetime = chrono::DateTime::from_timestamp(time.seconds(), 0)
				.unwrap_or_else(|| chrono::DateTime::from_timestamp(0, 0).unwrap());
			let date = datetime.format("%Y-%m-%d %H:%M").to_string();
			
			let commit_item = CommitItem {
				id,
				summary,
				author,
				date,
			};
			
			self.commits.push(commit_item);
		}
		
		Ok(())
	}
	
	pub fn commits(&self) -> &[CommitItem] {
		&self.commits
	}
} 