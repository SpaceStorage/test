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

// for prometheus
#[macro_use]
extern crate lazy_static;
use prometheus::{Encoder, Opts, Registry, TextEncoder, IntCounterVec, CounterVec, GaugeVec, register_int_counter_vec};
use std::sync::Mutex;
use std::collections::HashMap;

pub struct SpaceLocalBuffer {
    pub buffer: HashMap<String, Vec<u8>>,
    pub buffer_size: usize,
    pub metrics_tree: Prometheus,
}

impl SpaceLocalBuffer {
    pub fn new() -> SpaceLocalBuffer {
        return SpaceLocalBuffer {
            buffer: HashMap::new(),
            buffer_size: 1000000,
            metrics_tree: Prometheus::new(),
        };
    }

    pub fn insert(&mut self, name: String, value: Vec<u8>) {
        self.buffer.insert(name, value);
    }
}

pub struct Prometheus {
    r: prometheus::Registry,
    pub access: prometheus::CounterVec,
    pub access_received_bytes: prometheus::CounterVec,
    pub response_time: prometheus::GaugeVec,
}

impl Prometheus {
    pub fn new() -> Self {
        let access_opts = Opts::new("spacestorage_access", "access queries");
        let access_received_bytes_opts = Opts::new("spacestorage_received_bytes", "received bytes");
        let response_time_opts = Opts::new("spacestorage_response_time", "response time");
        let metric_tree = Prometheus {
            r: Registry::new(),
            access: CounterVec::new(access_opts, &["namespace", "project", "operation"]).unwrap(),
            access_received_bytes: CounterVec::new(access_received_bytes_opts, &["namespace", "project", "operation"]).unwrap(),
            response_time: GaugeVec::new(response_time_opts, &["project", "operation", "quantile"]).unwrap(),
        };

        metric_tree.r.register(Box::new(METRIC_ACCESS_COUNTER.clone())).unwrap();
        metric_tree.r.register(Box::new(METRIC_RECEIVED_BYTES_COUNTER.clone())).unwrap();

        return metric_tree;
    }

    pub fn get_metrics(&mut self) -> std::vec::Vec::<u8> {
        let mut buffer = Vec::<u8>::new();
        let encoder = TextEncoder::new();
        let metric_families = self.r.gather();
        encoder.encode(&metric_families, &mut buffer).unwrap();

        return buffer;
    }
}

lazy_static!(
    pub static ref GLOBAL: Mutex<SpaceLocalBuffer> = Mutex::new(SpaceLocalBuffer::new());
);

lazy_static! {
    pub static ref METRIC_ACCESS_COUNTER: IntCounterVec =
        register_int_counter_vec ! (
            "spacestorage_access",
            "access queries",
            & ["namespace", "project", "operation"]
        ).unwrap();
    pub static ref METRIC_RECEIVED_BYTES_COUNTER: IntCounterVec =
        register_int_counter_vec ! (
            "spacestorage_received_bytes",
            "received bytes",
            & ["namespace", "project", "operation"]
        ).unwrap();
}


#[tokio::main]
async fn main() {
    // build runtime
    let rt = Builder::new_multi_thread()
        .worker_threads(47)
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
    futures.push(Box::pin(tcp_server_start(&rt, "0.0.0.0:1112")));
    futures.push(Box::pin(udp_server_start(&rt, "0.0.0.0:514", 10000)));
    futures.push(Box::pin(http_server_start(&rt, "0.0.0.0:8080")));
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

            //println!("UDP Echoed {} bytes to {}: {:?}", size, peer, std::str::from_utf8(&buf[..size]).unwrap());
            rt.spawn(async move {
                //let ten_millis = Duration::from_millis(1000);
                //thread::sleep(ten_millis);
                let _x = pi(12);
                //println!("x is {}", x);
                //println!("spawned thread has id {}", thread_id::get());
                METRIC_ACCESS_COUNTER.with_label_values(&["global", "global", "udp"]).inc();
                METRIC_RECEIVED_BYTES_COUNTER.with_label_values(&["global", "global", "udp"]).inc_by(size as u64);
            });
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

async fn openmetrics() -> Result<String, Box<dyn std::error::Error>> {
    let mut ret_string: String = "".to_string();

    match GLOBAL.lock() {
        Ok(mut slb) => {
            let metrics_str = slb.metrics_tree.get_metrics();
            //let metrics_converted = String::from_utf8(metrics_str).unwrap();
            ret_string = String::from_utf8(metrics_str).unwrap();
        }
        Err(e) => {
            println!("error is {}", e);
        }
    }
    return Ok(ret_string);
}


async fn router_service(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let header_host = &req.headers()["host"];
    println!("headers: {:?}", header_host);

    if (req.method() == &Method::GET) && (req.uri().path().starts_with("/test")) {
        get_thread_info();
        Ok(Response::new("Hello, World".into()))
    }
    else if (req.method() == &Method::GET) && (req.uri().path().starts_with("/metrics")) {
        let response = openmetrics().await.unwrap();
        Ok(Response::new(Body::from(response)))
    }
    else if (req.method() == &Method::POST) && (req.uri().path().starts_with("/")) {
        get_thread_info();
        let whole_body = hyper::body::to_bytes(req.into_body()).await.unwrap();
        //let reversed_body = whole_body.iter().rev().cloned().collect::<Vec<u8>>();
        write_bytes(&whole_body, "test.txt".to_string()).await;
        Ok(Response::new(Body::from("{\"status\": \"ok\"}")))
    }
    else if (req.method() == &Method::PUT) && (req.uri().path().starts_with("/")) {
        get_thread_info();
        let whole_body = hyper::body::to_bytes(req.into_body()).await.unwrap();
        //let reversed_body = whole_body.iter().rev().cloned().collect::<Vec<u8>>();
        write_bytes(&whole_body, "test.txt".to_string()).await;
        Ok(Response::new(Body::from("{\"status\": \"ok\"}")))
    }
    else if (req.method() == &Method::GET) && (req.uri().path().starts_with("/")) {
        get_thread_info();
        let response = read_bytes("test.txt".to_string()).await.unwrap();
        Ok(Response::new(Body::from(response)))
    }
    else {
        Ok(not_found())
    }
}

async fn http_server_start(rt: &Runtime, addr: &str) {
    let socket: SocketAddr = addr
        .parse()
        .expect("Unable to parse socket address");

    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(router_service))
    });

    let server = Server::bind(&socket)
        .tcp_keepalive(Some(Duration::from_secs(60)))
        .tcp_nodelay(true)
        .serve(make_svc);

    rt.spawn(server);
}



use std::cmp;
use std::fmt;
use ::std::{*,
    convert::{
        TryFrom,
    },
    ops::{
        Not,
        Sub,
    },
};
use ::num_bigint::{ // 0.2.2
    BigInt,
    BigUint,
};
use fraction::Fraction;

fn pi (precision: u64) -> String
{
    /// atan(x) = x - x^3/3 + x^5/5 - x^7/7 + x^9/9...
    fn atan (x: Fraction, precision: u64) -> Fraction
    {
        //use ::num_traits::pow::pow;
        let end: BigUint =
            BigUint::from(10_u32)
                .pow(precision as u32)
        ;
        let target = Fraction::new(1.into(), end);

        let mut current_term = x.clone();
        let mut ret = Fraction::from(0);
        let mut sign = BigInt::from(1);
        let mut n = BigUint::from(1_u32);
        let mut x_pow_n = x.clone();
        let two = BigUint::from(2_u32);
        let x_square = &x * &x;

        while current_term.abs() > target {
            ret = ret + current_term;
            // eprintln!(
            //     "atan({}) ~ {}",
            //     x,
            //     ret.decimal(precision as usize),
            // );
            n += &two;
            sign = -sign;
            x_pow_n = x_pow_n * &x_square;
            current_term = &x_pow_n * Fraction::new(
                sign.clone(),
                n.clone(),
            );
        }
        ret
    }

    let precision_usize = usize::
        try_from(precision)
            .expect("Overflow")
    ;
    let pi_approx = Fraction::sub(
        Fraction::from(16) * atan(
            Fraction::new(1.into(), 5_u32.into()),
            precision
                .checked_add(2) // 16 -> 10 ^ 2
                .expect("Overflow"),
        ),
        Fraction::from(4) * atan(
            Fraction::new(1.into(), 239_u32.into()),
            precision + 1, // 4 -> 10 ^ 1
        ),
    );
    pi_approx.decimal(precision_usize)
}

mod fraction {
    #![allow(clippy::suspicious_arithmetic_impl)]
    use super::*;
    use ::num_bigint::*;
    use ::num_traits::*; // 0.2.8
    use ::core::ops::{
        Add,
        Div,
        Mul,
        Neg,
        Sub,
    };

    #[derive(
        Debug,
        Clone,
        PartialEq, Eq,
        Ord,
    )]
    pub
    struct Fraction {
        pub
        numerator: BigInt,

        pub
        denominator: BigUint,
    }

    impl From<i32> for Fraction {
        #[inline]
        fn from (x: i32) -> Self
        {
            Self::new(x.into(), 1_u32.into())
        }
    }

    impl PartialOrd for Fraction {
        fn partial_cmp (
            self: &'_ Self,
            other: &'_ Self,
        ) -> Option<cmp::Ordering>
        {
            (self - other)
                .numerator
                .partial_cmp(&BigInt::zero())
        }
    }

    impl Fraction {
        pub
        fn new (
            numerator: BigInt,
            denominator: BigUint,
        ) -> Self
        {
            assert!(denominator.is_zero().not(), "Division by zero");
            let mut ret = Self {
                numerator,
                denominator,
            };
            ret.simplify();
            ret
        }

        pub
        fn simplify (self: &'_ mut Self)
        {
            let (sign, abs) = self.numerator.split();
            let gcd = gcd(
                &abs,
                &self.denominator,
            );
            self.numerator = BigInt::from_biguint(
                sign,
                abs / &gcd,
            );
            self.denominator /= gcd;
        }

        pub
        fn inverse (self: &'_ Self) -> Self
        {
            if let Some(numerator) = self.numerator.to_biguint() {
                Fraction::new(
                    self.denominator.to_bigint()
                        .unwrap() // why ???
                    ,
                    numerator,
                )
            } else {
                Fraction::new(
                    BigInt::from_biguint(
                        self.numerator.sign(),
                        self.denominator.clone(),
                    ),
                    self.numerator
                        .clone()
                        .neg()
                        .to_biguint()
                        .unwrap(),
                )
            }
        }

        pub
        fn abs (self: &'_ Self) -> Self
        {
            Self {
                numerator: self.numerator.abs(),
                denominator: self.denominator.clone(),
            }
        }

        pub
        fn decimal (
            self: &'_ Self,
            precision: usize,
        ) -> String
        {
            use ::core::fmt::Write;
            use ::num_integer::Integer;
            let mut ret = String::new();
            let Self {
                numerator,
                denominator,
            } = self.clone();
            let (sign, mut numerator) = numerator.split();
            if let Sign::Minus = sign {
                ret.push('-');
            }
            let base = BigUint::from(10_u32);
            let (q, r) = numerator.div_mod_floor(&denominator);
            write!(&mut ret, "{}", q).unwrap();
            if r.is_zero() {
                return ret;
            } else {
                ret.reserve(1 + precision);
                ret.push('.');
            }
            numerator = r * &base;
            for _ in 0 .. precision {
                let (q, r) = numerator.div_mod_floor(&denominator);
                write!(&mut ret, "{}", q).unwrap();
                if r.is_zero() { break; }
                numerator = r * &base;
            }
            ret
        }
    }

    macro_rules! derive_op {(
        impl $Op:ident for Fraction {
            type Output = Fraction;

            fn $op:ident (&$self:tt, &$other:tt) -> Self::Output
            $body:block
        }
    ) => (
        impl<'a> $Op for &'a Fraction {
            type Output = Fraction;

            fn $op ($self: &'a Fraction, $other: &'a Fraction) -> Self::Output
            $body
        }

        impl<'a> $Op<&'a Fraction> for Fraction {
            type Output = Fraction;

            #[inline]
            fn $op ($self: Fraction, $other: &'a Fraction) -> Self::Output
            {
                $Op::$op(&$self, $other)
            }
        }

        impl<'a> $Op<Fraction> for &'a Fraction {
            type Output = Fraction;

            #[inline]
            fn $op ($self: &'a Fraction, $other: Fraction) -> Self::Output
            {
                $Op::$op($self, &$other)
            }
        }

        impl $Op for Fraction {
            type Output = Fraction;

            #[inline]
            fn $op ($self: Fraction, $other: Fraction) -> Self::Output
            {
                $Op::$op(&$self, &$other)
            }
        }
    )}

    derive_op! {
        impl Add for Fraction {
            type Output = Fraction;

            fn add (&self, &other) -> Self::Output
            {
                let lhs = {
                    let (sign, abs) = self.numerator.split();
                    BigInt::from_biguint(
                        sign,
                        abs * &other.denominator,
                    )
                };
                let rhs = {
                    let (sign, abs) = other.numerator.split();
                    BigInt::from_biguint(
                        sign,
                        abs * &self.denominator,
                    )
                };
                Fraction::new(
                    lhs + rhs,
                    &self.denominator * &other.denominator,
                )
            }
        }
    }

    derive_op! {
        impl Sub for Fraction {
            type Output = Fraction;

            fn sub (&self, &other) -> Self::Output
            {
                let lhs = {
                    let (sign, abs) = self.numerator.split();
                    BigInt::from_biguint(
                        sign,
                        abs * &other.denominator,
                    )
                };
                let rhs = {
                    let (sign, abs) = other.numerator.split();
                    BigInt::from_biguint(
                        sign,
                        abs * &self.denominator,
                    )
                };
                Fraction::new(
                    lhs - rhs,
                    &self.denominator * &other.denominator,
                )
            }
        }
    }

    derive_op! {
        impl Mul for Fraction {
            type Output = Fraction;

            fn mul (&self, &other) -> Self::Output
            {
                Fraction::new(
                    &self.numerator * &other.numerator,
                    &self.denominator * &other.denominator,
                )
            }
        }
    }

    derive_op! {
        impl Div for Fraction {
            type Output = Fraction;

            fn div (&self, &other) -> Self::Output
            {
                self * other.inverse()
            }
        }
    }

    impl fmt::Display for Fraction {
        fn fmt (
            self: &'_ Self,
            stream: &'_ mut fmt::Formatter<'_>,
        ) -> fmt::Result
        {
            write!(stream,
                "{} / {}",
                self.numerator,
                self.denominator,
            )
        }
    }

    fn gcd (a: &'_ BigUint, b: &'_ BigUint) -> BigUint
    {
        let mut a = a.clone();
        let mut b = b.clone();
        while b.is_zero().not() {
            let r = a % &b;
            a = b;
            b = r;
        }
        a
    }

    trait SignSplit {
        fn split (self: &'_ Self) -> (Sign, BigUint);
    }
    impl SignSplit for BigInt {
        fn split (self: &'_ BigInt) -> (Sign, BigUint)
        {
            fn to_biguint_lossy (this: &'_ BigInt) -> BigUint
            {
                this.to_biguint()
                    .unwrap_or_else(||
                        this.neg()
                            .to_biguint()
                            .unwrap()
                    )
            }
            (self.sign(), to_biguint_lossy(self))
        }
    }
}
