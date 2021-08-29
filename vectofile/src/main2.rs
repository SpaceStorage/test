fn main() {
    let arr: Vec<i128> = vec![12, 120, 45, 50, 6345];
    
    let u8_arr = arr
        .into_iter()
        .map(|value| value.to_le_bytes().to_vec())
        .flatten()
        .collect::<Vec<_>>();
    dbg!(&u8_arr);
}
