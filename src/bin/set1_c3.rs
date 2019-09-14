use cryptopals::utils::*;
use cryptopals::base64::*;
fn main() {
    let (bytes, code, _score) = decrypt_by_chars(&hex_to_bytes("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736").unwrap());
    println!("Result: {}, Code: {}", bytes_to_string(bytes.as_slice()), code);
}


