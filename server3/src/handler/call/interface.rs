use std::pin::Pin;
use futures::future::join_all;
#[path = "../../fs/write.rs"] mod write;

pub async fn run(data: &Vec<u8>) -> &[u8] {

    let mut fut: Vec<Pin<Box<dyn warp::Future<Output = ()>>>> = Vec::new();
    let write_op = write::write_bytes(data);
    fut.push(Box::pin(write_op));

    join_all(fut).await;

    return "ok".as_bytes();
}
