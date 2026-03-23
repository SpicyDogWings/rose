use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Author {
    pub name: String,
    pub email: String,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Commit {
    pub commit_id: String,
    pub parents: Vec<String>,
    pub change_id: String,
    pub description: String,
    pub author: Author,
    pub committer: Author,
    #[serde(skip)]
    pub is_current: bool,
}
