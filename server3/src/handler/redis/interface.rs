use crate::util::global::{GLOBAL};
use redis_protocol::resp2::prelude::*;
use bytes::{Bytes, BytesMut};

pub async fn run(data: &[u8]) -> String {
    //let mut buf = Bytes::from(data);
    //if let Ok(slb) = GLOBAL.lock() {
    //    slb.metrics_tree.handler_call.with_label_values(&["global", "redis"]).inc();
    //}

    //let (frame, consumed) = match decode(&buf) {
    //    Ok(Some((f, c))) => (f, c),
    //    Ok(None) => panic!("Incomplete frame."),
    //    Err(e) => panic!("Error parsing bytes: {:?}", e)
    //};
    //println!("Parsed frame {:?} and consumed {} bytes", frame, consumed);


    let frame = Frame::BulkString("+OK".into());
    let mut buf = BytesMut::new();

    let len = match encode_bytes(&mut buf, &frame) {
        Ok(l) => l,
        Err(e) => panic!("Error encoding frame: {:?}", e)
    };
    println!("Encoded {} bytes into buffer with contents {:?}", len, buf);

    return String::from_utf8_lossy(&buf).to_string();
}
