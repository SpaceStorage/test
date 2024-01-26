use futures::future::BoxFuture;
use futures::FutureExt;
//use futures::Future;
pub mod call;
pub mod redis;

async fn stub(_data: &[u8]) -> String {
    "".to_string()
}

async fn aw(data: &[u8]) -> String {
    println!("world! {}", String::from_utf8_lossy(data));
    "check_ok".to_string()
}

type AsyncFn = Box<dyn Fn(&[u8]) -> BoxFuture<String>>;

pub fn get_handlers() -> Vec::<AsyncFn> {
    //let funpointer: Vec<Box<dyn Fn(&[u8]) -> String>> = vec![Box::new(redis::interface::run), Box::new(w)];
    //let funpointer2: Vec<Fn(&[u8]) -> String> = vec![redis::interface::run, Box::new(w)];
    //let mut fut: Vec<Box<dyn std::future::Future<Output = ()>>> = Vec::new();
    //fut.push(Box::new(redis::interface::run));

    //funpointer[1]("check1".as_bytes());
    //funpointer[0]("check2".as_bytes());

    //return functions
    let mut funpointer = Vec::<AsyncFn>::new();
    funpointer.push(Box::new(|data| stub(data).boxed()));
    funpointer.push(Box::new(|data| redis::interface::run(data).boxed()));

    return funpointer
}

pub fn get_id_handler(handler: String) -> usize {
    match handler {
        _ if handler == "redis" => return 1,
        _ => return 0,
    }
}
