use crate::Branch;

pub mod git;
#[cfg(test)]
pub mod mock;

pub trait Client {
    fn list_branches(&self) -> Vec<Branch>;
}
