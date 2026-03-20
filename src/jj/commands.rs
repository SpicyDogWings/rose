use tokio::process::Command;

use crate::jj::types::Commit;
use crate::jj::parser::parse_jj_log;

pub async fn run_jj_log() -> Vec<Commit> {
    let output = Command::new("jj")
        .arg("log")
        .arg("-T")
        .arg("json(self)")
        .output()
        .await
        .unwrap();
    let log_output = String::from_utf8_lossy(&output.stdout).to_string();
    parse_jj_log(&log_output)
}
