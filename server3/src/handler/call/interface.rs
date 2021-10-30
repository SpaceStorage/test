use std::pin::Pin;
use futures::future::join_all;
#[path = "../../fs/write.rs"] mod write;
#[path = "../../util/global.rs"] mod global;
use global::GLOBAL;

pub async fn run(data: &[u8]) -> &[u8] {
    let newline : &[u8] = &[0x0a];
    let res:Vec<u8> = [data, newline].concat();

    GLOBAL.buffer.get("1");
    GLOBAL.insert("1".to_string(), res.clone());
    let write_op = write::write_bytes(&res);

    let mut fut: Vec<Pin<Box<dyn warp::Future<Output = ()>>>> = Vec::new();
    fut.push(Box::pin(write_op));

    join_all(fut).await;

    return "ok".as_bytes();
}
