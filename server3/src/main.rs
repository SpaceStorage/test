use warp::{self, Filter};

use console::Style;
use futures::future::join_all;
//use std::marker::Unpin;
use std::pin::Pin;
use std::net::SocketAddr;

mod listener;
mod fs;

mod settings;
use settings::settings::Settings;

#[tokio::main]
async fn main() {
    let conf = Settings::new()
        .expect("Config parse error");
    println!("settings is {:#?}", conf);

    let bold = Style::new().bold();
    let green = Style::new().green();
    let cyan = Style::new().cyan();
    let red = Style::new().red();
    let magenta = Style::new().magenta();

    let health = listener::router::health::router()
        .and_then(listener::router::health::handler).with(warp::log("health"));
    let dummy = listener::router::dummy::router()
        .and_then(listener::router::dummy::handler).with(warp::log("dummy"));

    let mut fut: Vec<Pin<Box<dyn warp::Future<Output = ()>>>> = Vec::new();
    let router = health
        .or(dummy);

    for srv_obj in conf.server.iter() {

        if srv_obj.proto == "http" {
            let socket: SocketAddr = srv_obj.addr
                .parse()
                .expect("Unable to parse socket address");

            if srv_obj.tls.key != "" && srv_obj.tls.certificate != "" {
                let srv_init = warp::serve(router.clone())
                    .tls()
                    .cert_path(&srv_obj.tls.certificate)
                    .key_path(&srv_obj.tls.key)
                    .run(socket);
                fut.push(Box::pin(srv_init));
                println!("Rust inside, warp HTTPs server at {}", bold.apply_to(green.apply_to(&srv_obj.addr)));
            } else {
                let srv_init = warp::serve(router.clone())
                    .run(socket);
                fut.push(Box::pin(srv_init));
                println!("Rust inside, warp HTTP server at {}", bold.apply_to(cyan.apply_to(&srv_obj.addr)));
            }
        } else if srv_obj.proto == "tcp" {
            //listener::tcp::server::run(srv_obj.addr.clone());
            if srv_obj.tls.key != "" && srv_obj.tls.certificate != "" {
                let srv_init = listener::tcp::tls::run(srv_obj.addr.clone(), srv_obj.tls.certificate.clone(), srv_obj.tls.key.clone());
                fut.push(Box::pin(srv_init));
                println!("Rust inside, Tokio TLS server at {}", green.apply_to(&srv_obj.addr));
            } else {
                let srv_init = listener::tcp::server::run(srv_obj.addr.clone());
                fut.push(Box::pin(srv_init));
                println!("Rust inside, Tokio TCP server at {}", cyan.apply_to(&srv_obj.addr));
            }
        } else if srv_obj.proto == "udp" {
            if srv_obj.tls.key != "" && srv_obj.tls.certificate != "" {
                println!("Rust inside, Tokio DTLS server at {}", magenta.apply_to(&srv_obj.addr));
                println!("Erorr: spacestorage now not support DTLS, port not listening");
            } else {
                let srv_init = listener::udp::server::server_run(srv_obj.addr.clone());
                fut.push(Box::pin(srv_init));
                println!("Rust inside, Tokio UDP server at {}", red.apply_to(&srv_obj.addr));
            }
        }
    }

    join_all(fut).await;
}
