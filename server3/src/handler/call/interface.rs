use std::pin::Pin;
use futures::future::join_all;
//use bstr;

#[path = "../../fs/write.rs"] mod write;

pub async fn run(data: &[u8]) -> &[u8] {

    let newline : &[u8] = &[0x0a];
    let res:Vec<u8> = [data, newline].concat();
    let write_op = write::write_bytes(&res);
    //let res = bstr::concat([data, &[0x0a]]);
    //let res_bytes = &res;
    //let write_op = write::write_bytes(res_bytes);

    let mut fut: Vec<Pin<Box<dyn warp::Future<Output = ()>>>> = Vec::new();
    fut.push(Box::pin(write_op));

    join_all(fut).await;

    return "ok".as_bytes();
}
