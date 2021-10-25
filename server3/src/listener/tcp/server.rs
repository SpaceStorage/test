use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
//use tokio::io::copy;
use std::process;
use thread_id;
use std::{thread, time};

pub async fn run(addr: String) {
    let listener = TcpListener::bind(&addr).await.unwrap();

    loop {
        let (mut socket, _) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            // In a loop, read data from the socket and write the data back.
            loop {
                let n = match socket.read(&mut buf).await {
                    // socket closed
                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };
                println!("buf is {}", String::from_utf8(buf.to_vec()).unwrap());
                println!("My pid is {}", process::id());
                println!("spawned thread has id {}", thread_id::get());
                let ten_millis = time::Duration::from_millis(10000);
                thread::sleep(ten_millis);

                // Write the data back
                if let Err(e) = socket.write_all(&buf[0..n]).await {
                    eprintln!("failed to write to socket; err = {:?}", e);
                    return;
                }
            }
        });
    }
}
