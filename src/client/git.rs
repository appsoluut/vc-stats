use git2::{Repository, Time};
use time::OffsetDateTime;

use crate::{Author, Branch, BranchType, Commit};

use super::Client;

pub struct GitClient;

impl From<git2::Signature<'_>> for Author {
    fn from(author: git2::Signature) -> Self {
        Author {
            name: author.name().map(|s| s.to_string()),
            email: author.email().map(|s| s.to_string()),
        }
    }
}

impl GitClient {
    fn convert_time(&self, time: &Time) -> String {
        let offset = time.offset_minutes();
        let (hours, minutes) = (offset / 60, offset % 60);
        let dt = OffsetDateTime::from_unix_timestamp(time.seconds()).unwrap();
        let dto = dt.to_offset(time::UtcOffset::from_hms(hours as i8, minutes as i8, 0).unwrap());
        let format = time::format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]T[offset_hour sign:mandatory][offset_minute]")
                .unwrap();
        let time_str = dto.format(&format).unwrap();
        time_str
    }
}

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

    fn list_commits(&self) -> Vec<Commit> {
        let repo = Repository::open("./").unwrap();
        let mut revwalk = repo.revwalk().unwrap();
        revwalk.push_head().unwrap();
        let commits: Vec<Commit> = revwalk
            .map(|oid| {
                let oid = oid.unwrap();
                let commit = repo.find_commit(oid).unwrap();

                Commit {
                    id: oid.to_string().as_str()[..7].to_owned(),
                    message: commit.message().unwrap().to_string(),
                    body: commit.body().map(|s| s.to_string()),
                    author: Author::from(commit.author()),
                    time: self.convert_time(&commit.time()),
                }
            })
            .collect();
        commits
    }
}
