#[macro_use]
extern crate lazy_static;
use warp::{self, Filter};

use console::Style;
use futures::future::join_all;
use std::pin::Pin;
use std::net::SocketAddr;
use std::fs::create_dir_all;
use std::path::Path;
use std::env;
use std::io;

mod listener;
mod fs;
mod metafs;
mod metrics;
mod parser;

mod settings;
use settings::settings::Settings;

mod util;
mod handler;

#[tokio::main]
async fn main() {
    let conf = Settings::new()
        .expect("Config parse error");
    println!("settings is {:#?}", conf);

    set_path(conf.root_path).unwrap();

    // initialize metrics
    //let mut metrics_tree = metrics::prometheus::Prometheus::new();

    //metrics_tree.access.with_label_values(&["myproj", "select"]).inc();
    //metrics_tree.response_time.with_label_values(&["myproj", "select", "0.5"]).set(0.3);
    //let mut metrics_str = metrics_tree.get_metrics();
    //println!("{}", String::from_utf8(metrics_str).unwrap());

    let bold = Style::new().bold();
    let green = Style::new().green();
    let cyan = Style::new().cyan();
    let red = Style::new().red();
    let magenta = Style::new().magenta();

    let health = listener::router::health::router()
        .and_then(listener::router::health::handler).with(warp::log("health"));
    let dummy = listener::router::dummy::router()
        .and_then(listener::router::dummy::handler).with(warp::log("dummy"));
    let openmetrics = listener::router::openmetrics::router()
        .and_then(listener::router::openmetrics::handler).with(warp::log("openmetrics"));
    //let es_api = listener::router::es_api::router()
    //    .and_then(|scheme_header: Option<String>, host: String, path: FullPath| listener::router::es_api::handler).with(warp::log("es_api"));
    //let es_api = listener::router::es_api::commutation();
    let es_get = warp::get()
        .and(warp::path!("es" / String))
        .and(warp::path::end())
        .and_then(listener::router::es_api::get).with(warp::log("es_get"));
    let es_put = warp::put()
        .and(warp::path!("es" / String / String / u64))
        .and(warp::path::end())
        .and(listener::router::es_api::put_body())
        .and_then(listener::router::es_api::put).with(warp::log("es_put"));

    let es_bulk_put = warp::put()
        .and(warp::path!("_bulk"))
        .and(warp::path::end())
        .and(listener::router::es_api::put_body())
        .and_then(listener::router::es_api::put_bulk).with(warp::log("es_put_bulk"));

    let es_bulk_index_put = warp::put()
        .and(warp::path!(String / "_bulk"))
        .and(warp::path::end())
        .and(listener::router::es_api::put_body())
        .and_then(listener::router::es_api::put_bulk_index).with(warp::log("es_put_bulk_index"));

    let es_index = warp::path::end().and_then(listener::router::es_api::get_index).with(warp::log("es_get"));

    //let es_enrich_put = warp::put()
    //    .and(warp::path!("_enrich"))
    //    .and(warp::path::end())
    //    .and(listener::router::es_api::put_body())
    //    .and_then(listener::router::es_api::put).with(warp::log("es_put"));

    //let es_sql_put = warp::put()
    //    .and(warp::path!("_sql"))
    //    .and(warp::path::end())
    //    .and(listener::router::es_api::put_body())
    //    .and_then(listener::router::es_api::put).with(warp::log("es_put"));

    let mut fut: Vec<Pin<Box<dyn warp::Future<Output = ()>>>> = Vec::new();
    let router = health
        .or(openmetrics)
        .or(es_get)
        .or(es_put)
        .or(es_bulk_put)
        .or(es_bulk_index_put)
        .or(dummy)
        .or(es_index);

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
                let srv_init = listener::udp::server::server_run(srv_obj.addr.clone(), srv_obj.buffer_size);
                fut.push(Box::pin(srv_init));
                println!("Rust inside, Tokio UDP server at {}", red.apply_to(&srv_obj.addr));
            }
        }
    }

    join_all(fut).await;
}

fn set_path(path: String) -> Result<(), io::Error> {
    println!("Set root directory to: {}", path);

    create_dir_all(&path)?;

    let root = Path::new(&path);
    let _result = env::set_current_dir(&root)
        .map(|_| {
            println!("Successfully changed working directory to {}!", root.display());
        })
        .map_err(|err| {
            eprintln!("Error IO change working directory to {}: {:?}!", root.display(), err);
        });
    Ok(())
}
