use warp::{
    filters::BoxedFilter,
    Filter,
};

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
    let reply = format!("{{\"status\": \"ok\"}}\n");
    print!("{}", &reply);
    Ok(warp::reply::html(reply))
}
