use crate::{Branch, Commit};

pub mod git;
#[cfg(test)]
pub mod mock;

pub trait Client {
    fn list_branches(&self) -> Vec<Branch>;
    fn list_commits(&self) -> Vec<Commit>;
}
