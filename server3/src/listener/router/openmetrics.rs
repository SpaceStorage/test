use warp::{
    filters::BoxedFilter,
    Filter,
};

use crate::util::global::{GLOBAL};

fn path_prefix() -> BoxedFilter<()> {
    warp::path("metrics")
        .boxed()
}

pub fn router() -> BoxedFilter<()> {
    warp::get()
        .and(path_prefix())
        .boxed()
}

pub async fn handler() -> Result<impl warp::Reply, warp::Rejection> {
    let mut ret_string = "".to_string();
    //if let Ok(mut slb) = GLOBAL.lock() {
    match GLOBAL.lock() {
        Ok(mut slb) => {
            slb.metrics_tree.access.with_label_values(&["global", "global", "openmetrics"]).inc();
            //slb.metrics_tree.response_time.with_label_values(&["myproj", "select", "0.5"]).set(0.3);
            let metrics_str = slb.metrics_tree.get_metrics();
            //let metrics_converted = String::from_utf8(metrics_str).unwrap();
            ret_string = String::from_utf8(metrics_str).unwrap();
            //Ok(warp::reply::html(metrics_converted))
        }
        Err(e) => {
            println!("error is {}", e);
        }
    }

    Ok(warp::reply::html(ret_string))
}
