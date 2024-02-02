use futures::future::BoxFuture;
use futures::FutureExt;
use std::sync::Arc;
pub mod call;
pub mod redis;

async fn stub(_data: &[u8]) -> String {
    "".to_string()
}

async fn aw(data: &[u8]) -> String {
    println!("world! {}", String::from_utf8_lossy(data));
    "check_ok".to_string()
}

type AsyncFn = Box<dyn Fn(&[u8]) -> BoxFuture<String> + Send + Sync>;

pub fn get_handlers() -> Vec::<Arc<AsyncFn>> {
    //return functions
    let mut funpointer = Vec::<Arc<AsyncFn>>::new();
    funpointer.push(Arc::new(Box::new(|data| stub(data).boxed())));
    funpointer.push(Arc::new(Box::new(|data| redis::interface::run(data).boxed())));

    return funpointer
}

pub fn get_id_handler(handler: String) -> usize {
    match handler {
        _ if handler == "redis" => return 1,
        _ => return 0,
    }
}
