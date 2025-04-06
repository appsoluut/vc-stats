use crate::{Branch, BranchType};

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
}
