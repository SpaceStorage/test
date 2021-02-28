use std::mem;
mod vectorqueue;
//use vectorqueue::vqueue;

fn analyze_slice(slice: &[i128]) {
    println!("first elem of slice: {}", slice[0]);
    println!("nums elems in slice: {}", slice.len());
}

fn main() {
    let xs: [i128; 5] = [1, 2, 3, 4, 5];
    let ys: [i128; 500] = [0; 500];
    println!("first elem: {}", xs[0]);
    println!("array length: {}", xs.len());
    println!("array size is {} byte", mem::size_of_val(&xs));
    analyze_slice(&xs);
    analyze_slice(&ys[1 .. 20]);
    println!("{}", xs[4]);
    vectorqueue::vqueue();
}
