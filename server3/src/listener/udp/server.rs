use std::net::SocketAddr;
use std::{io};
use tokio::net::UdpSocket;
#[path = "../../handler/call/interface.rs"] mod handler;

struct Server {
    socket: UdpSocket,
    buf: Vec<u8>,
    to_send: Option<(usize, SocketAddr)>,
}

impl Server {
    async fn run(self) -> Result<(), io::Error> {
        let Server {
            socket,
            mut buf,
            mut to_send,
        } = self;

        loop {
            if let Some((size, peer)) = to_send {
                let amt = socket.send_to(&buf[..size], &peer).await.unwrap();

                println!("UDP Echoed {}/{} bytes to {}", amt, size, peer);

                handler::run(&buf).await;
            }

            to_send = Some(socket.recv_from(&mut buf).await.unwrap());
        }
    }
}

pub async fn server_run(addr: String, size: usize) -> () {
    let socket = UdpSocket::bind(&addr).await.unwrap();

    let server = Server {
        socket,
        buf: vec![0; size],
        to_send: None,
    };

    // This starts the server task.
    server.run().await.unwrap();
}
