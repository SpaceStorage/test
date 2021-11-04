use std::pin::Pin;
use futures::future::join_all;
use crate::fs::write;
use crate::util::global::{GLOBAL};

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

pub async fn run(data: &[u8]) -> &[u8] {
    let newline : &[u8] = &[0x0a];
    let res:Vec<u8> = [data, newline].concat();

    if let Ok(mut slb) = GLOBAL.lock() {
        let buf = slb.buffer.get_mut("1");
        println!("res len is {}", res.len());
        if buf == Option::None {
            slb.insert("1".to_string(), res.clone());
            return "ok".as_bytes();
        }

        let bufdata = buf.unwrap();
        print_type_of(&bufdata);

        println!("bufdata len is {}, capacity is {}", bufdata.len(), bufdata.capacity());
        if bufdata.len() >= 1000000 {
            println!("bufdata len is {}, write!", bufdata.len());
            let write_op = write::write_bytes(&bufdata);
            let mut fut: Vec<Pin<Box<dyn warp::Future<Output = ()>>>> = Vec::new();
            fut.push(Box::pin(write_op));

            join_all(fut).await;
            slb.buffer.clear();
        } else {
            bufdata.extend(res.clone());
        }
    }

    return "ok".as_bytes();
}
