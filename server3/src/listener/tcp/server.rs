use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
//use std::process;
//use thread_id;
use std::sync::{Arc, Mutex};
use crate::util::global::{GLOBAL};
use crate::handler::call::interface;

pub async fn run(addr: String, handler: String) {
    let listener = TcpListener::bind(&addr).await.unwrap();

    loop {
        let (mut socket, _) = listener.accept().await.unwrap();

        let handler = Arc::new(Mutex::new(handler.clone()));
        tokio::spawn(async move {
            let mut buf = [0; 1024];

            loop {
                let size = match socket.read(&mut buf).await {
                    // socket closed
                    Ok(size) if size == 0 => return,
                    Ok(size) => size,
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };

                let buffer_cloned = buf.clone();
                //println!("spawned thread has id {}, pid is {}", thread_id::get(), process::id());
                let handler = handler.lock().unwrap().clone();
                let ret = interface::run(&buffer_cloned[..size], &handler).await;
                let ret = ret.as_bytes();

                if let Err(e) = socket.write_all(&ret).await {
                    eprintln!("failed to write to socket; err = {:?}", e);
                    return;
                }

                if let Ok(slb) = GLOBAL.lock() {
                    slb.metrics_tree.access.with_label_values(&["global", "global", "tcp"]).inc();
                }
            }
        });
    }
}
