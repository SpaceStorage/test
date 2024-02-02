#![allow(unused)]
use crate::util::global::{GLOBAL};
use futures::future::BoxFuture;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use crate::handler;

pub async fn run(addr: String, handler: String) {
    let listener = TcpListener::bind(&addr).await.unwrap();
    let func_id = handler::get_id_handler(handler);
    let function = Arc::clone(&handler::get_handlers()[func_id]);

    loop {
        let (mut socket, _) = listener.accept().await.unwrap();
        let function = function.clone();

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            loop {
                let size = match socket.read(&mut buf).await {
                    Ok(size) if size == 0 => return,
                    Ok(size) => size,
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };

                let buffer_cloned = buf.clone();
                let ret = function("klol".as_bytes()).await;
                let ret = ret.as_bytes();

                //let ret = "dscdc".as_bytes();
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
