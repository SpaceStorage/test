use warp::{self, Filter};

use console::Style;
use futures::future::join_all;
//use std::marker::Unpin;
use std::pin::Pin;
use std::net::SocketAddr;

mod listener;

mod settings;
use settings::settings::Settings;

#[tokio::main]
async fn main() {
    let conf = Settings::new()
        .expect("Oh no, got an Err!");
    println!("settings is {:#?}", conf);
    println!("http server is {:#?}", conf.http_server[0]);
    println!("log level is {}", conf.log_level);

    let target: String = "0.0.0.0:8000".parse().unwrap();
    let green = Style::new().green();
    let cyan = Style::new().cyan();

    let health = listener::router::health::router()
        .and_then(listener::router::health::handler).with(warp::log("health"));
    let dummy = listener::router::dummy::router()
        .and_then(listener::router::dummy::handler).with(warp::log("dummy"));

    println!("HTTP server at {}", green.apply_to(&target));
    println!("Rust inside, warp HTTP server");

    let mut fut: Vec<Pin<Box<dyn warp::Future<Output = ()>>>> = Vec::new();
    let router = health
        .or(dummy);

    let srv_1 = warp::serve(router.clone())
        .run(([0, 0, 0, 0], 8000));

    let srv_2 = warp::serve(router.clone())
        .tls()
        .cert_path("tls/cert.pem")
        .key_path("tls/key2.rsa")
        .run(([0, 0, 0, 0], 8001));

    for srv_obj in conf.http_server.iter() {
        let socket: SocketAddr = srv_obj.addr
            .parse()
            .expect("Unable to parse socket address");

        let srv_init = warp::serve(router.clone())
            .run((socket));
        fut.push(Box::pin(srv_init));
    }

    fut.push(Box::pin(srv_1));
    fut.push(Box::pin(srv_2));
    join_all(fut).await;
}
