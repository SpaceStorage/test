use warp::{
    //filters::BoxedFilter,
    Filter,
    http,
};
use warp::hyper::body::Bytes;
use serde_json;
use std::str::{from_utf8};
use crate::handler::call::interface;
use crate::util::global::{GLOBAL};
//use futures::Future;
use std::io;
use serde_json::json;
use tokio::fs;
use tokio::io::AsyncWriteExt;

//fn path_prefix() -> BoxedFilter<()> {
//    warp::path("/")
//        .boxed()
//}
//
//pub fn router() -> BoxedFilter<()> {
//    warp::get()
//        .and(path_prefix())
//        //.and(warp::header("host")
//        .boxed()
//}

pub fn post_body() -> impl Filter<Extract = (Bytes,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::bytes()) //.and(warp::body::json())
}

pub async fn get(a: String) -> Result<impl warp::Reply, warp::Rejection> {
    if let Ok(slb) = GLOBAL.lock() {
        slb.metrics_tree.access.with_label_values(&["global", "global", "es_api"]).inc();
    }

    println!("path is {}", a);
    let reply = format!("{{\"_index\": \"_index\"}}\n");
    Ok(warp::reply::html(reply))
}


pub async fn put(
    es_index: String,
    es_type: String,
    es_id: u64,
    body: Bytes,
    ) -> Result<impl warp::Reply, warp::Rejection> {

        if let Ok(slb) = GLOBAL.lock() {
            slb.metrics_tree.access.with_label_values(&["global", "global", "es_api"]).inc();
        }

        if let Ok(str_body) = from_utf8(&body) {
            let json_body: serde_json::Value = serde_json::from_str(str_body).unwrap();
            println!("path is {} : {} : {} : {:?}\n{:?}\n", es_index, es_type, es_id, body, json_body);
        }

        let _reply = format!("{{\"_index\": \"_index\"}}\n");
        //Ok(warp::reply::html(reply))
        Ok(warp::reply::with_status(
            "test",
            http::StatusCode::CREATED,
        ))
}

pub async fn post_bulk(
    body: Bytes,
    ) -> Result<impl warp::Reply, warp::Rejection> {

        if let Ok(slb) = GLOBAL.lock() {
            slb.metrics_tree.access.with_label_values(&["global", "global", "es_api"]).inc();
        }

        if let Ok(str_body) = from_utf8(&body) {
            let splited_json = str_body.split("\n");

            let mut index_name: String = String::from("");
            let mut doc_type: String = String::from("");
            let mut i: u64 = 0;
            for s in splited_json {
                let json_body: serde_json::Value = serde_json::from_str(s).unwrap();
                println!("done s!!: {:?}", json_body);

                i += 1;
                if (i % 2) == 1 {
                    let index_data: serde_json::Value = json_body["index"].clone();
                    index_name = index_data["_index"].to_string();
                    doc_type = index_data["_type"].to_string();
                    println!("{}: {} // {}", index_name, doc_type, json_body);
                } else {
                    //let mut file = fs::OpenOptions::new()
                    //   .append(true)
                    //   .create(true)
                    //   .open("./test.log")
                    //   .await
                    //   .unwrap();
                    //file.write_all(&"tst".as_bytes()).await.unwrap();

                    interface::run_elastic_doc(json_body, &index_name, &doc_type).await;
                    ////interface::run_elastic_doc(json_body, &index_name, &doc_type).await?;
                    ////let fut: Vec<Pin<Box<dyn warp::Future<Output = &[u8]>>>> = Vec::new();
                    ////fut.push(Box::pin(iface_op));
                    ////join_all(fut).await;
                }
            }

        //    //let iface_op = interface::run2(str_body.as_bytes());
        //    //let mut fut: Vec<Pin<Box<dyn warp::Future<Output = &[u8]>>>> = Vec::new();
        //    //let mut fut: Vec<Pin<Box<dyn warp::Future<Output = &[u8]>>>> = Vec::new();
        //    //fut.push(Box::pin(iface_op));
        //    //join_all(fut).await;
        }

        let _reply = format!("{{\"_index\": \"_index\"}}\n");
        Ok(warp::reply::with_status(
            "test",
            http::StatusCode::CREATED,
        ))
}

pub async fn post_bulk_index(
    _es_index: String,
    body: Bytes,
    ) -> Result<impl warp::Reply, warp::Rejection> {

        if let Ok(slb) = GLOBAL.lock() {
            slb.metrics_tree.access.with_label_values(&["global", "global", "es_api"]).inc();
        }

        if let Ok(str_body) = from_utf8(&body) {
            let _json_body: serde_json::Value = serde_json::from_str(str_body).unwrap();
        }

        let _reply = format!("{{\"_index\": \"_index\"}}\n");
        Ok(warp::reply::with_status(
            "test",
            http::StatusCode::CREATED,
        ))
}

pub async fn get_index() -> Result<impl warp::Reply, warp::Rejection> {
    let hostname = hostname::get().unwrap();
    let data = json!({
        "name": hostname.to_string_lossy(),
        "cluster_name": "",
        "cluster_uuid": "",
        "tagline": "",
        "version": {
            "number": "0.0.1",
            "build_flavor": "default",
            "build_type" : "cargo",
            "build_snapshot" : "false",
            "lucene_version" : "",
            "minimum_wire_compatibility_version" : "6.8.0",
            "minimum_index_compatibility_version" : "6.0.0-beta1"
        }
    });

    Ok(warp::reply::with_status(
        //data.to_string(),
        serde_json::to_string_pretty(&data).unwrap(),
        http::StatusCode::OK,
    ))
}
