use warp::{self, Filter};

use console::Style;

mod listener;

#[tokio::main]
async fn main() {
    let target: String = "0.0.0.0:8000".parse().unwrap();
    let green = Style::new()
        .green();

    let health = listener::router::health::router()
        .and_then(listener::router::health::handler).with(warp::log("health"));
    let dummy = listener::router::dummy::router()
        .and_then(listener::router::dummy::handler).with(warp::log("dummy"));

    println!("\nHTTP server at {}", green.apply_to(&target));
    println!("Rust inside, warp HTTP server");

    warp::serve(health.or(dummy)).run(([0, 0, 0, 0], 8000)).await;
}
