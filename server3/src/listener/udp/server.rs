use std::net::SocketAddr;
use std::{io};
use tokio::net::UdpSocket;
use crate::util::global::{GLOBAL};
use crate::handler::call::interface;

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
                //let amt = socket.send_to(&buf[..size], &peer).await.unwrap();

                if let Ok(mut slb) = GLOBAL.lock() {
                    slb.metrics_tree.access.with_label_values(&["global", "global", "udp"]).inc();
                    slb.metrics_tree.access_received_bytes.with_label_values(&["global", "global", "udp"]).inc_by(size as f64);

                    println!("lock ok");
                    let metrics_str = slb.metrics_tree.get_metrics();
                    let ret_string = String::from_utf8(metrics_str).unwrap();
                    println!("metrics is {}", ret_string);
                }

                println!("UDP Echoed {} bytes to {}", size, peer);

                interface::run(&buf[..size]).await;
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
