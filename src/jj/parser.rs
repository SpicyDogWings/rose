use serde_json;

use crate::jj::types::Commit;

pub fn parse_jj_log(log: &str) -> Vec<Commit> {
    let mut commits = Vec::new();
    for line in log.lines() {
        if line.starts_with('@') || line.starts_with('◆') {
            if let Some(json_str) = line.splitn(2, ' ').nth(1) {
                if let Ok(mut commit) = serde_json::from_str::<Commit>(json_str) {
                    commit.is_current = line.starts_with('@');
                    commits.push(commit);
                }
            }
        }
    }
    commits
}
