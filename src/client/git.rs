use git2::Repository;

use crate::{Branch, BranchType};

use super::Client;

pub struct GitClient;

impl Client for GitClient {
    fn list_branches(&self) -> Vec<Branch> {
        let repo = Repository::open("./").unwrap();
        let branches = repo.branches(None).unwrap();
        let branches: Vec<Branch> = branches
            .map(|branch| {
                let (branch, branch_type) = branch.unwrap();
                let name = branch.name().unwrap().unwrap();
                Branch {
                    name: name.to_string(),
                    branch_type: match branch_type {
                        git2::BranchType::Local => BranchType::Local,
                        git2::BranchType::Remote => BranchType::Remote,
                    },
                }
            })
            .collect();
        branches
    }

    fn list_commits(&self) -> Vec<String> {
        let repo = Repository::open("./").unwrap();
        let mut revwalk = repo.revwalk().unwrap();
        revwalk.push_head().unwrap();
        let commits: Vec<String> = revwalk
            .map(|oid| {
                let oid = oid.unwrap();
                let commit = repo.find_commit(oid).unwrap();
                commit.message().unwrap().to_string()
            })
            .collect();
        commits
    }
}
