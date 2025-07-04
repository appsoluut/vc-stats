pub mod client;

#[derive(Debug)]
pub enum BranchType {
    Local,
    Remote,
}

#[derive(Debug)]
pub struct Branch {
    pub name: String,
    pub branch_type: BranchType,
}

#[derive(Debug)]
pub struct Commit {
    pub id: String,
    pub message: String,
    pub body: Option<String>,
    pub author: Author,
    pub time: String,
}

#[derive(Debug)]
pub struct Author {
    pub name: Option<String>,
    pub email: Option<String>,
}
