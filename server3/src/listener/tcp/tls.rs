use rustls_pemfile::{certs, rsa_private_keys};
use std::fs::File;
use std::io::{self, BufReader};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::io::{copy, sink, split, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio_rustls::rustls::{self, Certificate, PrivateKey};
use tokio_rustls::TlsAcceptor;
use crate::util::global::{GLOBAL};

fn load_certs(path: &Path) -> io::Result<Vec<Certificate>> {
    certs(&mut BufReader::new(File::open(path)?))
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid cert"))
        .map(|mut certs| certs.drain(..).map(Certificate).collect())
}

fn load_keys(path: &Path) -> io::Result<Vec<PrivateKey>> {
    rsa_private_keys(&mut BufReader::new(File::open(path)?))
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid key"))
        .map(|mut keys| keys.drain(..).map(PrivateKey).collect())
}

//#[tokio::main]
//async fn run() -> io::Result<()> {
pub async fn run(addr: String, cert: String, key: String) {

    let mut path_cert = PathBuf::new();
    path_cert.push(cert);
    let mut path_key = PathBuf::new();
    path_key.push(key);

    //let addr = addrBind.to_socket_addrs().unwrap();
    //    .next()
    //    .ok_or_else(|| io::Error::from(io::ErrorKind::AddrNotAvailable))?;
    let certs = load_certs(&path_cert).unwrap();
    let mut keys = load_keys(&path_key).unwrap();
    let flag_echo = false;

    let config = rustls::ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(certs, keys.remove(0))
        .map_err(|err| io::Error::new(io::ErrorKind::InvalidInput, err)).unwrap();
    let acceptor = TlsAcceptor::from(Arc::new(config));

    let listener = TcpListener::bind(&addr).await.unwrap();

    loop {
        let (stream, peer_addr) = listener.accept().await.unwrap();
        let acceptor = acceptor.clone();

        if let Ok(slb) = GLOBAL.lock() {
            slb.metrics_tree.access.with_label_values(&["global", "global", "tls"]).inc();
        }

        let fut = async move {
            let mut stream = acceptor.accept(stream).await.unwrap();

            if flag_echo {
                let (mut reader, mut writer) = split(stream);
                let n = copy(&mut reader, &mut writer).await.unwrap();
                writer.flush().await.unwrap();
                println!("Echo: {} - {}", peer_addr, n);
            } else {
                let mut output = sink();
                stream
                    .write_all(
                        &b"HTTP/1.0 200 ok\r\n\
                    Connection: close\r\n\
                    Content-length: 12\r\n\
                    \r\n\
                    Hello world!"[..],
                    )
                    .await.unwrap();
                stream.shutdown().await.unwrap();
                copy(&mut stream, &mut output).await.unwrap();
                println!("Hello: {}", peer_addr);
            }

            Ok(()) as io::Result<()>
        };

        tokio::spawn(async move {
            if let Err(err) = fut.await {
                println!("{:?}", err);
            }
        });
    }
}
