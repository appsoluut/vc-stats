use vc_stats::client::{Client, git::GitClient};

fn main() {
    let client = GitClient;
    let branches = client.list_branches();
    println!("Branches in the repository:");
    branches.iter().for_each(|branch| {
        println!("* {} [{:?}]", branch.name, branch.branch_type);
    });
}
