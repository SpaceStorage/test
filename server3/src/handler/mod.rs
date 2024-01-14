pub mod call;
pub mod redis;

fn f() {
    println!("hello!")
}

fn w() {
    println!("world!")
}


//pub async fn get_handlers() -> Vec<for<'a> fn(&[u8]) -> impl for<'b> warp::Future<&str>> {
//pub async fn get_handlers() -> Vec<for<'a> fn(&[u8]) -> impl warp::Future<Output = &str>> {
pub async fn get_handlers() {
    //let redis_call: fn(&[u8]) -> impl warp::Future<Output = &str> = redis::interface::run;
    ////let functions: Vec<fn(&[u8]) -> impl warp::Future<Output>> = vec![redis::interface::run];
    //let functions: Vec<fn(&[u8]) -> impl warp::Future<Output = &str>> = vec![redis_call];

    let funpointer: Vec<fn()> = vec![f, w];
    funpointer[1]();
    funpointer[0]();

    //return functions
}
