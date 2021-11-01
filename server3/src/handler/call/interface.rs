use std::pin::Pin;
use futures::future::join_all;
#[path = "../../fs/write.rs"] mod write;
#[path = "../../util/global.rs"] mod global;
use global::GLOBAL;

pub async fn run(data: &[u8]) -> &[u8] {
    let newline : &[u8] = &[0x0a];
    let res:Vec<u8> = [data, newline].concat();

    let mut hm = GLOBAL.lock().unwrap();
    let buf = hm.buffer.get("1");
    if buf == Option::None {
        hm.insert("1".to_string(), res.clone());
        return "ok".as_bytes();
    }

    let data = buf.unwrap();

    println!("data len is {}", data.len());
    if data.len() >= hm.buffer_size {
        println!("data len is {}, write!", data.len());
        let write_op = write::write_bytes(&data);
        let mut fut: Vec<Pin<Box<dyn warp::Future<Output = ()>>>> = Vec::new();
        fut.push(Box::pin(write_op));

        join_all(fut).await;
        hm.buffer.clear();
    } else {
        hm.insert("1".to_string(), res.clone());
    }


    return "ok".as_bytes();
}
