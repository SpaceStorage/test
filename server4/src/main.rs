use tokio::runtime::{Builder, Runtime};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::time::Duration;
use std::{thread};
use thread_id;
use futures::future::join_all;
use std::pin::Pin;

// for udp
use tokio::net::UdpSocket;
use std::net::SocketAddr;
use socket2::SockAddr;

// for tcp
use tokio::net::TcpListener;

// for hyper
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, StatusCode, Request, Response, Server};
use std::convert::Infallible;


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

    //let mut futures = Vec::new();
    let mut futures: Vec<Pin<Box<dyn std::future::Future<Output = ()>>>> = Vec::new();
    futures.push(Box::pin(tcp_server_start(&rt, "0.0.0.0:1111")));
    futures.push(Box::pin(udp_server_start(&rt, "0.0.0.0:1111", 10000)));
    futures.push(Box::pin(http_server_start("0.0.0.0:1112")));
    join_all(futures).await;
}

fn get_thread_info() {
    let ten_millis = Duration::from_millis(1000);
    thread::sleep(ten_millis);
    println!("spawned thread has id {}", thread_id::get());
}

async fn tcp_server_start(rt: &Runtime, addr: &str) {
    let listener = TcpListener::bind(addr).await.unwrap();

    let mut buf = [0; 65535];
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
            println!("buf is {}", String::from_utf8(buf.to_vec()).unwrap());
            get_thread_info();
        });
        if let Err(e) = socket.write_all(&buf[0..n]).await {
            eprintln!("failed to write to socket; err = {:?}", e);
            return;
        }
    }
}

async fn udp_server_start(rt: &Runtime, addr: &str, size: usize) {
    let mut to_send: Option<(usize, SocketAddr)>;
    //let mut buf: Vec<u8>;
    let mut buf = [0; 65535];

    let sock = socket2::Socket::new(
        socket2::Domain::ipv4(),
        socket2::Type::dgram(),
        Some(socket2::Protocol::udp()),
    ).unwrap();
    
    let addr_sock: SocketAddr = addr
        .parse()
        .expect("Unable to parse socket address");
    let addr_sock2 = SockAddr::from(addr_sock);
    
    sock.set_nonblocking(true).unwrap();
    sock.bind(&addr_sock2).unwrap();
    println!("set buffer socket size {}", size);
    sock.set_recv_buffer_size(size).unwrap();
    
    let socket = UdpSocket::from_std(sock.into_udp_socket()).unwrap();

    loop {
        to_send = Some(socket.recv_from(&mut buf).await.unwrap());
        if let Some((size, peer)) = to_send {

            println!("UDP Echoed {} bytes to {}: {:?}", size, peer, std::str::from_utf8(&buf[..size]).unwrap());
            rt.spawn(async move {
                let ten_millis = Duration::from_millis(1000);
                thread::sleep(ten_millis);
                println!("thread spawned: {}", String::from_utf8(buf.to_vec()).unwrap());
                println!("spawned thread has id {}", thread_id::get());
            });
            socket.send_to(&buf[..size], &peer).await.unwrap();
        }
    }
}

fn not_found() -> Response<Body> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body("not found".into())
        .unwrap()
}

async fn hello_world(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/test") | (&Method::GET, "/index.html") => {
            get_thread_info();
            Ok(Response::new("Hello, World".into()))
        }
        _ => Ok(not_found()),
    }
}

async fn http_server_start(addr: &str) {
    // We'll bind to 127.0.0.1:3000
    //let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let socket: SocketAddr = addr
        .parse()
        .expect("Unable to parse socket address");

    // A `Service` is needed for every connection, so this
    // creates one from our `hello_world` function.
    let make_svc = make_service_fn(|_conn| async {
        // service_fn converts our function into a `Service`
        Ok::<_, Infallible>(service_fn(hello_world))
    });

    let server = Server::bind(&socket).serve(make_svc);

    // Run this server for... forever!
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

