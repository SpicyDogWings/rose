use tokio::process::Command;

pub async fn run_jj_log() -> String {
  let output = Command::new("jj")
    .arg("log")
    .output()
    .await
    .unwrap();
  String::from_utf8_lossy(&output.stdout).to_string()
}
