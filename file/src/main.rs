extern crate tokio;

use tokio::prelude::{AsyncWrite, Future};

fn main() {
    //println!("Hello, world!");
    let task = tokio::fs::File::create("foo.txt")
        .and_then(|mut file| file.poll_write(b"hello, world!"))
        .map(|res| {
            println!("{:?}", res);
        }).map_err(|err| eprintln!("IO error: {:?}", err));

    tokio::run(task);
}
