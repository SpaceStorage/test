extern crate tokio;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use std::pin::Pin;
use futures::future::join_all;

pub async fn write_bytes(bytes: &[u8], name: String) {
    let mut file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(name + ".log")
        .await
        .unwrap();
     file.write_all(&bytes).await.unwrap();
}

pub async fn write_idx(bytes: &u8, name: String) {
    let mut file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(name + ".idx")
        .await
        .unwrap();
     let byte_write: Vec<u8> = vec![*bytes];
     file.write(&byte_write).await.unwrap();
}

pub async fn operation_write(bytes: &[u8], idx: &u8, name: String) {
    println!("file is {}", name);
    let wb = write_bytes(bytes, name.clone());
    let wi = write_idx(idx, name.clone());

    let mut fut: Vec<Pin<Box<dyn warp::Future<Output = ()>>>> = Vec::new();
    fut.push(Box::pin(wb));
    fut.push(Box::pin(wi));

    join_all(fut).await;
}
