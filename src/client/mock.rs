use crate::{Branch, BranchType, Commit};

use super::Client;

struct MockClient;

impl Client for MockClient {
    fn list_branches(&self) -> Vec<Branch> {
        vec![
            Branch {
                name: "main".to_string(),
                branch_type: BranchType::Local,
            },
            Branch {
                name: "feature".to_string(),
                branch_type: BranchType::Remote,
            },
        ]
    }

    fn list_commits(&self) -> Vec<Commit> {
        vec![Commit {
            message: "bla".to_string(),
            id: "1234567".to_string(),
            time: "2023-10-01 12:00:00+00:00".to_string(),
        }]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_branches() {
        let client = MockClient;
        let branches = client.list_branches();
        assert_eq!(branches.len(), 2);
        assert_eq!(branches[0].name, "main");
        assert_eq!(branches[1].name, "feature");
    }

    #[test]
    fn test_list_commits() {
        let client = MockClient;
        let commits = client.list_commits();
        assert_eq!(commits.len(), 1);
        assert_eq!(commits[0].id, "1234567");
        assert_eq!(commits[0].message, "bla");
        assert_eq!(commits[0].time, "2023-10-01 12:00:00+00:00");
    }
}
