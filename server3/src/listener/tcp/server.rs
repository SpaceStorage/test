use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
//use std::process;
//use thread_id;
//use std::sync::{Arc, Mutex};
use crate::util::global::{GLOBAL};
use crate::handler::call::interface;
use crate::handler;
use tokio::sync::Mutex;
use std::sync::Arc;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

pub async fn run(addr: String, handler: String) {
    let listener = TcpListener::bind(&addr).await.unwrap();
    let func_id = handler::get_id_handler(handler);
    let function = &handler::get_handlers()[func_id];
    print_type_of(function);

    loop {
        let (mut socket, _) = listener.accept().await.unwrap();

        let function = Arc::new(Box::new(move || Box::pin(function)));
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

                let function = function.clone();
                let function = *function;
                function("klol".as_bytes()).await;

                //let ret = ret.as_bytes();
                let ret = "dscdc".as_bytes();

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
