use warp::{
    filters::BoxedFilter,
    Filter,
};
use std::process;
use thread_id;
use crate::util::global::{GLOBAL};

fn path_prefix() -> BoxedFilter<()> {
    warp::path("health")
        .boxed()
}

pub fn router() -> BoxedFilter<()> {
    warp::get()
        .and(path_prefix())
        .boxed()
}

pub async fn handler() -> Result<impl warp::Reply, warp::Rejection> {
    if let Ok(slb) = GLOBAL.lock() {
        slb.metrics_tree.access.with_label_values(&["global", "global", "health"]).inc();
    }


    let reply = format!("{{\"status\": \"ok\"}}\n");
    print!("{}", &reply);
    println!("My pid is {}", process::id());
    println!("spawned thread has id {}", thread_id::get());
    Ok(warp::reply::html(reply))
}
