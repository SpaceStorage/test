extern crate tokio;
use tokio::fs;
use tokio::io::Error;
use crate::tokio::io::AsyncReadExt;

async fn reader() -> Result<(), Error> {
    let mut file = fs::File::open("foo.txt").await?;
    let mut contents = vec![];
    file.read_to_end(&mut contents).await?;
    println!("{:?}", contents);
    Ok(())
}

#[tokio::main]
async fn main() {
    reader().await.unwrap();
}
