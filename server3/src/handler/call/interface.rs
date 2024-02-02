//use std::pin::Pin;
use std::str;
//use std::io;
//use futures::future::join_all;
use crate::metafs::write;
//use crate::util::global::{GLOBAL};
use crate::parser;
use crate::handler::redis;
use std::pin::Pin;
use futures::Future;
//use std::sync::Arc;

//fn print_type_of<T>(_: &T) {
//    println!("{}", std::any::type_name::<T>())
//}

//use std::cmp;
//use std::fmt;
//use ::std::{*,
//    convert::{
//        TryFrom,
//    },
//    ops::{
//        Not,
//        Sub,
//    },
//};
//use ::num_bigint::{ // 0.2.2
//    BigInt,
//    BigUint,
//};

//pub async fn run<'a>(data: &'a [u8], handler: &'a str) -> &'a [u8] {
//pub async fn run<'a>(data: &'a [u8], handler: &'a str) -> String {
pub async fn run<'a>(data: &'a [u8], handler: &Box<dyn Fn(&'a [u8]) -> Pin<Box<dyn warp::Future<Output = std::string::String> + std::marker::Send>>>) -> String {
//pub async fn run<'a>(data: &'a [u8], handler: &Future<Output = ()>) -> String {
    let newline : &[u8] = &[0x0a];
    let _res:Vec<u8> = [data, newline].concat();

    let mut rec_data = parser::record::Record::new(data.to_vec());
    rec_data.syslog_parse();

    let mut file_write: Vec<u8> = "test".as_bytes().to_vec();
    let file_write_field = rec_data.field.get("appname");
    if let Some(v) = file_write_field {
        file_write = v.clone();
    }
    let file_write_str = str::from_utf8(&file_write).unwrap_or_else(|_| "test");

    //if let Ok(mut slb) = GLOBAL.lock() {
    //    let buf = slb.buffer.get_mut(file_write_str);
    //    //println!("res len is {}", res.len());
    //    if buf == Option::None {
    //        slb.insert(file_write_str.to_string(), res.clone());
    //        return "ok".as_bytes();
    //    }

    //    let bufdata = buf.unwrap();
    //    //print_type_of(&bufdata);

    //    println!("bufdata len is {}, capacity is {}, buffer/file is {}", bufdata.len(), bufdata.capacity(), file_write_str);
    //    if bufdata.len() >= 10000 {
    //        println!("bufdata len is: {}, write to file: {}!", bufdata.len(), file_write_str);
    //        let write_op = write::write(file_write_str, &bufdata).await;

    //        //let mut fut: Vec<Pin<Box<dyn warp::Future<Output = ()>>>> = Vec::new();
    //        //fut.push(Box::pin(write_op));

    //        //join_all(fut).await;
    //        slb.buffer.clear();
    //    } else {
    //        bufdata.extend(res.clone());
    //    }
    //}
    let _write_op = write::write(file_write_str, &data).await;

    let mut ret = "".to_string();
    handler(data).await;

    //if handler == "redis" {
    //    //ret = "+OK\r\n";
    //    ret = redis::interface::run(data).await;
    //}
    //}

    return ret;
}

#[derive(Debug)]
struct IOerr;
impl warp::reject::Reject for IOerr {}

pub async fn run_elastic_doc(data: serde_json::Value, index: &str, doc_type: &str) ->  Result<&'static [u8], warp::Rejection> {
    let file_write: Vec<u8> = "test".as_bytes().to_vec();
    let file_write_str = str::from_utf8(&file_write).unwrap_or_else(|_| "test");

    println!("Index: {}, Type: {}, Data: {:?}", index, doc_type, data);
            let _write_op = write::write(file_write_str, "test".as_bytes());

    //        let mut fut: Vec<Pin<Box<dyn warp::Future<Output = ()>>>> = Vec::new();
    //        fut.push(Box::pin(write_op));

    //        join_all(fut).await;
    //        //slb.buffer.clear();
    ////return "ok".as_bytes();

    //if 1 == 2 {
    //    return Err(warp::reject::custom(IOerr));
    //}

    Ok("ok".as_bytes())
}
