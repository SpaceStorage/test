use warp::{
    filters::BoxedFilter,
    Filter,
};

fn path_prefix() -> BoxedFilter<()> {
    warp::path("dummy")
        .boxed()
}

pub fn router() -> BoxedFilter<(String, )> {
    warp::get()
        .and(path_prefix())
        .and(warp::path::param::<String>())
        .boxed()
}

pub async fn handler(name: String) -> Result<impl warp::Reply, warp::Rejection> {
    let reply = format!("Dummy: '{}'!\n", name);
    print!("{}", &reply);
    Ok(warp::reply::html(reply))
}
