use git2::{Repository as Git2Repository, Error};
use std::path::Path;

pub struct Repository {
	repo: Git2Repository,
}

impl Repository {
	/// Open a Git repository at the specified path
	pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
		let repo = Git2Repository::open(path)?;
		Ok(Self { repo })
	}
	
	/// Discover a Git repository starting from the given path and walking up the directory tree
	pub fn discover<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
		let repo = Git2Repository::discover(path)?;
		Ok(Self { repo })
	}
	
	/// Create a Repository from an existing Git2Repository
	pub fn from_git2_repo(repo: Git2Repository) -> Self {
		Self { repo }
	}
	
	/// Get the path to the repository's working directory
	pub fn workdir(&self) -> Option<&Path> {
		self.repo.workdir()
	}
	
	/// Get the path to the repository's .git directory
	pub fn path(&self) -> &Path {
		self.repo.path()
	}
	
	/// Check if the repository is bare
	pub fn is_bare(&self) -> bool {
		self.repo.is_bare()
	}
	
	/// Get the inner Git2Repository object
	pub fn inner(&self) -> &Git2Repository {
		&self.repo
	}
} 