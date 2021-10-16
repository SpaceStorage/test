use warp::{self, Filter};

use console::Style;
use futures::future::join_all;

mod listener;

#[tokio::main]
async fn main() {
    let target: String = "0.0.0.0:8000".parse().unwrap();
    let green = Style::new().green();
    let cyan = Style::new().cyan();

    let health = listener::router::health::router()
        .and_then(listener::router::health::handler).with(warp::log("health"));
    let dummy = listener::router::dummy::router()
        .and_then(listener::router::dummy::handler).with(warp::log("dummy"));

    println!("HTTP server at {}", green.apply_to(&target));
    println!("Rust inside, warp HTTP server");

    let mut fut = Vec::new();
    let router = health
        .or(dummy);

    fut.push(warp::serve(router.clone())
        .run(([0, 0, 0, 0], 8000))
        );
    fut.push(warp::serve(router.clone())
        .tls()
        .cert_path("tls/cert.pem")
        .key_path("tls/key2.rsa")
        .run(([0, 0, 0, 0], 8001))
        );
    join_all(fut).await;
}
