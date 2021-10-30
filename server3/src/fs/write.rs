extern crate tokio;
use tokio::fs;

pub async fn write_bytes(bytes: &[u8]) {
    fs::write("/var/log/test.log", &bytes).await.unwrap();
}
