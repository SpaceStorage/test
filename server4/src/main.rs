use tokio::runtime::{Builder, Runtime};
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::time::Duration;
use std::{thread, time};
use thread_id;

#[tokio::main]
async fn main() {
    // build runtime
    let rt = Builder::new_multi_thread()
        .worker_threads(4)
        .enable_all()
        .enable_io()
        .on_thread_start(|| {
            println!("thread started");
        })
        .on_thread_stop(|| {
            println!("thread stopping");
        })
        .build()
        .unwrap();

    TcpServerStart(rt, "0.0.0.0:1111").await;
}

async fn TcpServerStart(rt: Runtime, Addr: &str) {
    let listener = TcpListener::bind("0.0.0.0:1111").await.unwrap();

        let mut buf = [0; 1024];
        loop {
            let (mut socket, _) = listener.accept().await.unwrap();
            let n = match socket.read(&mut buf).await {
                // socket closed
                Ok(n) if n == 0 => return,
                Ok(n) => n,
                Err(e) => {
                    eprintln!("failed to read from socket; err = {:?}", e);
                    return;
                }
            };
            rt.spawn(async move {
                let ten_millis = Duration::from_millis(1000);
                thread::sleep(ten_millis);
                println!("thread spawned: {}", String::from_utf8(buf.to_vec()).unwrap());
                println!("spawned thread has id {}", thread_id::get());
            });
            if let Err(e) = socket.write_all(&buf[0..n]).await {
                eprintln!("failed to write to socket; err = {:?}", e);
                return;
            }
        }
}
