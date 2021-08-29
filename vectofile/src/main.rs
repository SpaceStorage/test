// vim: ts=4 sw=4 et autoindent backspace=indent,eol,start ruler showcmd
use futures::executor::block_on;

async fn file_write() {
    //let mut f = File::create("a.txt").await?;
    //let n = f.write(b"hello world").await?;
    println!("hello, world!");
}

fn main() {
    let mut arr = vec![];
    arr.push(12);
    arr.push(120);
    arr.push(45);
    arr.push(50);
    arr.push(6345i128);

    let mut u8array: Vec<u8> = vec![];

    for val in arr { u8array.extend(&val.to_le_bytes()) };
    dbg!(&u8array);

    //let mut i = 0;
    //for val in &arr {
    //    println!("val is {}", val);
    //    for le_bytes in val.to_le_bytes() {
    //        u8array[i] = le_bytes;
    //        i += 1;
    //    }
    //}

    let future = file_write();
    //block_on(future);

    //as_u8_slice(&arr)
}
