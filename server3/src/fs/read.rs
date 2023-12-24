//extern crate tokio;
//use tokio::fs;
//use tokio::io::AsyncReadExt;

//pub async fn read_bytes(bytes: &[u8], name: String) {
//    let mut file = fs::OpenOptions::new()
//        .open(name + ".log")
//        .await
//        .unwrap();
//     file.read_all(&bytes).await.unwrap();
//}

//pub async fn read_last_idx(name: String) -> u8 {
//    //let mut file = fs::File::open(name + ".idx").await;
//
//    match fs::File::open(name + ".idx").await {
//        Ok(mut file) => {
//            let mut buffer: Vec<u8> = Vec::new();
//            file.read_to_end(&mut buffer).await.unwrap();
//            return buffer[buffer.len() - 1];
//        }
//        Err(_e) => {
//            return 0;
//        }
//    }
//}
