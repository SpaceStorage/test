use aes::Aes128;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use hex_literal::hex;

// create an alias for convenience
type Aes128Cbc = Cbc<Aes128, Pkcs7>;

fn aes_encrypt(plaintext: &[u8], iv: &[u8], key: &[u8]) -> Vec<u8> {
    let cipher = Aes128Cbc::new_from_slices(&key, &iv).unwrap();
    // buffer must have enough space for message+padding
    let mut buffer = [0u8; 32];
    // copy message to the buffer
    let pos = plaintext.len();
    buffer[..pos].copy_from_slice(plaintext);
    let ciphertext = cipher.encrypt(&mut buffer, pos).unwrap();

    return ciphertext.to_vec();
}

fn aes_decrypt(ciphertext: &[u8], iv: &[u8], key: &[u8]) -> Vec<u8> {
    let cipher = Aes128Cbc::new_from_slices(&key, &iv).unwrap();
    let mut buf = ciphertext.to_vec();
    //let mut buf = ciphertext.clone();
    let decrypted_ciphertext = cipher.decrypt(&mut buf).unwrap();

    return decrypted_ciphertext.to_vec();
}

fn main() {
    let key = hex!("000102030405060708090a0b0c0d0e0f");
    let iv = hex!("f0f1f2f3f4f5f6f7f8f9fafbfcfdfeff");
    let plaintext = b"Hello world!";
    
    let ciphertext = aes_encrypt(plaintext, &iv, &key);
    let decrypted_ciphertext = aes_decrypt(&ciphertext, &iv, &key);
    assert_eq!(ciphertext, hex!("1b7a4c403124ae2fb52bedc534d82fa8"));
    
    // re-create cipher mode instance
    
    assert_eq!(decrypted_ciphertext, plaintext);
}
