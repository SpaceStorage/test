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
//use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, StatusCode, Request, Response};
use hyper::server::Server;
use std::convert::Infallible;
use hyper::service::{service_fn, make_service_fn};
//use hyper::header::{CONTENT_LENGTH, CONTENT_TYPE};

// for fs
use tokio::fs;
use tokio::fs::File;

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
    futures.push(Box::pin(http_server_start(&rt, "0.0.0.0:1112")));
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

pub async fn write_bytes(bytes: &[u8], name: String) {
    let mut file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(name + ".log")
        .await
        .unwrap();
     file.write_all(&bytes).await.unwrap();
}

async fn read_bytes(name: String) -> Result<String, Box<dyn std::error::Error>> {
    let mut file = File::open(name + ".log").await?;

    let mut contents = vec![];
    file.read_to_end(&mut contents).await?;
    let json_string = String::from_utf8(contents)?;
    return Ok(json_string);
}


async fn router_service(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let header_host = &req.headers()["host"];
    println!("headers: {:?}", header_host);

    //match (req.method(), req.uri().path()) {
    if (req.method() == &Method::GET) && (req.uri().path().starts_with("/test")) {
        get_thread_info();
        Ok(Response::new("Hello, World".into()))
    }
    else if (req.method() == &Method::POST) && (req.uri().path().starts_with("/")) {
        get_thread_info();
        let whole_body = hyper::body::to_bytes(req.into_body()).await.unwrap();
        let reversed_body = whole_body.iter().rev().cloned().collect::<Vec<u8>>();
        write_bytes(&reversed_body, "test.txt".to_string()).await;
        Ok(Response::new(Body::from("{\"status\": \"ok\"}")))
    }
    else if (req.method() == &Method::GET) && (req.uri().path().starts_with("/")) {
        get_thread_info();
        let reversed_body = read_bytes("test.txt".to_string()).await.unwrap();
        Ok(Response::new(Body::from(reversed_body)))
    } else {
        Ok(not_found())
    }
}

async fn http_server_start(rt: &Runtime, addr: &str) {
    // We'll bind to 127.0.0.1:3000
    //let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let socket: SocketAddr = addr
        .parse()
        .expect("Unable to parse socket address");

    let make_svc = make_service_fn(|_conn| async {
        // service_fn converts our function into a `Service`
        Ok::<_, Infallible>(service_fn(router_service))
    });

    let server = Server::bind(&socket)
        .tcp_keepalive(Some(Duration::from_secs(60)))
        .tcp_nodelay(true)
        .serve(make_svc);

    rt.spawn(server);
}

