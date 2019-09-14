use cryptopals::utils::*;
use cryptopals::base64::*;
use openssl::symm::{decrypt, Cipher};

const CODE:&[u8] = b"YELLOW SUBMARINE";

pub fn main() {
    let file_contents = get_file_contents("data/set1_c7.txt");
    let encrypted_vector = base64_to_bytes(&file_contents);
    let encrypted_bytes = encrypted_vector.as_slice();
    let cipher = Cipher::aes_128_ecb();
    let key = CODE;
    let iv: Option<&[u8]> = None;

    let output = decrypt(cipher, key, None, &encrypted_bytes);
    println!("{}", bytes_to_string(output.unwrap().as_slice()));
}