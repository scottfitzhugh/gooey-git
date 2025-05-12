use git2::{Branch as Git2Branch, BranchType, Error};
use super::repository::Repository;

pub struct Branch<'repo> {
	branch: Git2Branch<'repo>,
}

impl<'repo> Branch<'repo> {
	pub fn from_git2_branch(branch: Git2Branch<'repo>) -> Self {
		Self { branch }
	}
	
	pub fn name(&self) -> Result<Option<&str>, Error> {
		self.branch.name()
	}
	
	pub fn is_head(&self) -> bool {
		self.branch.is_head()
	}
	
	pub fn is_current(&self) -> bool {
		self.branch.is_head()
	}
	
	pub fn is_checked_out(&self) -> bool {
		self.branch.is_head()
	}
	
	pub fn upstream(&self) -> Result<Branch<'repo>, Error> {
		let upstream = self.branch.upstream()?;
		Ok(Branch::from_git2_branch(upstream))
	}
	
	pub fn delete(&mut self) -> Result<(), Error> {
		self.branch.delete()
	}
}

pub fn get_branches(repo: &Repository, branch_type: Option<BranchType>) -> Result<Vec<Branch>, Error> {
	let branches = repo.inner().branches(branch_type)?;
	let mut result = Vec::new();
	
	for branch_result in branches {
		if let Ok((branch, _)) = branch_result {
			result.push(Branch::from_git2_branch(branch));
		}
	}
	
	Ok(result)
}

pub fn get_current_branch(repo: &Repository) -> Result<Branch, Error> {
	let head = repo.inner().head()?;
	if !head.is_branch() {
		return Err(Error::from_str("HEAD is not a branch"));
	}
	
	let branch = Git2Branch::wrap(head);
	Ok(Branch::from_git2_branch(branch))
}

pub fn checkout_branch(repo: &Repository, branch_name: &str) -> Result<(), Error> {
	let obj = repo.inner().revparse_single(&format!("refs/heads/{}", branch_name))?;
	
	repo.inner().checkout_tree(&obj, None)?;
	
	repo.inner().set_head(&format!("refs/heads/{}", branch_name))?;
	
	Ok(())
}

pub fn create_branch<'a>(repo: &'a Repository, branch_name: &str, target_commit: &git2::Commit<'a>) -> Result<Branch<'a>, Error> {
	let branch = repo.inner().branch(branch_name, target_commit, false)?;
	Ok(Branch::from_git2_branch(branch))
} 