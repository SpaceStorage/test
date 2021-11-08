use crate::fs;
use std::pin::Pin;
use futures::future::join_all;

pub async fn write(name: &str, bytes: &[u8]) {
    let mut filepos = fs::read::read_last_idx(name.to_string()).await;
    filepos += 1;
    let write_op = fs::write::operation_write(&bytes, &filepos, name.to_string());

    let mut fut: Vec<Pin<Box<dyn warp::Future<Output = ()>>>> = Vec::new();
    fut.push(Box::pin(write_op));
    join_all(fut).await;
}
