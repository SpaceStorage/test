extern crate tokio;
use tokio::fs;
use tokio::io::AsyncWriteExt;

pub async fn write_bytes(bytes: &[u8]) {
    //fs::append("/var/log/test.log", &bytes).await.unwrap();
    let mut file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open("/var/log/test.log")
        .await
        .unwrap();
     file.write_all(&bytes).await.unwrap();
}
