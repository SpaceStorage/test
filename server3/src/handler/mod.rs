pub mod call;
pub mod redis;

fn f(data: &[u8]) -> &str {
    println!("hello! {}", String::from_utf8_lossy(data));
    "check_chpok"
}

fn w(data: &[u8]) -> &str {
    println!("world! {}", String::from_utf8_lossy(data));
    "check_ok"
}


pub async fn get_handlers() {
    let funpointer: Vec<fn(&[u8]) -> &str> = vec![f, w];
    let redis_call: fn(&[u8]) -> &str = redis::interface::run;
    ////let functions: Vec<fn(&[u8]) -> impl warp::Future<Output>> = vec![redis::interface::run];
    //let functions: Vec<fn(&[u8]) -> impl warp::Future<Output = &str>> = vec![redis_call];

    funpointer[1]("check1".as_bytes());
    funpointer[0]("check2".as_bytes());

    //return functions
}
