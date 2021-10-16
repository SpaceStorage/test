use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub async fn run(listener: TcpListener) -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080");

    loop {
        let (mut socket, _) = listener.accept();

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

                // Write the data back
                if let Err(e) = socket.write_all(&buf[0..n]).await {
                    eprintln!("failed to write to socket; err = {:?}", e);
                    return;
                }
            }
        });
    }
}

//pub async fn run(listener: TcpListener, shutdown: impl Future) {
//    let (notify_shutdown, _) = broadcast::channel(1);
//    let (shutdown_complete_tx, shutdown_complete_rx) = mpsc::channel(1);
//
//    let mut server = Listener {
//        listener,
//        db_holder: DbDropGuard::new(),
//        limit_connections: Arc::new(Semaphore::new(MAX_CONNECTIONS)),
//        notify_shutdown,
//        shutdown_complete_tx,
//        shutdown_complete_rx,
//    };
//
//    tokio::select! {
//        res = server.run() => {
//            // If an error is received here, accepting connections from the TCP
//            // listener failed multiple times and the server is giving up and
//            // shutting down.
//            //
//            // Errors encountered when handling individual connections do not
//            // bubble up to this point.
//            if let Err(err) = res {
//                error!(cause = %err, "failed to accept");
//            }
//        }
//        _ = shutdown => {
//            // The shutdown signal has been received.
//            info!("shutting down");
//        }
//    }
//
//
//    let Listener {
//        mut shutdown_complete_rx,
//        shutdown_complete_tx,
//        notify_shutdown,
//        ..
//    } = server;
//
//    drop(notify_shutdown);
//
//    drop(shutdown_complete_tx);
//
//    let _ = shutdown_complete_rx.recv().await;
//}
