use std::net::SocketAddr;
use socket2::SockAddr;
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

                if let Ok(slb) = GLOBAL.lock() {
                    slb.metrics_tree.access.with_label_values(&["global", "global", "udp"]).inc();
                    slb.metrics_tree.access_received_bytes.with_label_values(&["global", "global", "udp"]).inc_by(size as f64);
                }

                //println!("UDP Echoed {} bytes to {}: {:?}", size, peer, std::str::from_utf8(&buf[..size]));

                interface::run(&buf[..size]).await;
            }

            to_send = Some(socket.recv_from(&mut buf).await.unwrap());
        }
    }
}

pub async fn server_run(addr: String, size: usize) -> () {
    //let sock = socket2::Socket::new(
    //    socket2::Domain::ipv4(),
    //    socket2::Type::stream(),
    //    Some(socket2::Protocol::tcp()),
    //).unwrap();
    //
    ////sock.set_reuse_port(true).unwrap();
    //let addr_sock: SocketAddr = addr
    //    .parse()
    //    .expect("Unable to parse socket address");
    //let addr_sock2 = SockAddr::from(addr_sock);
    //
    ////sock.set_nonblocking(true).unwrap();
    //sock.bind(&addr_sock2).unwrap();
    //sock.listen(1024).unwrap();
    //sock.set_recv_buffer_size(size).unwrap();
    //
    //let socket = UdpSocket::from_std(sock.into_udp_socket()).unwrap();

    let socket = UdpSocket::bind(&addr).await.unwrap();

    let server = Server {
        socket,
        buf: vec![0; size],
        to_send: None,
    };

    // This starts the server task.
    server.run().await.unwrap();
}
