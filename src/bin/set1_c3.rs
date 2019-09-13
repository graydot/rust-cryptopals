use cryptopals::utils::*;
use cryptopals::base64::*;
fn main() {
    let (plain_text, code, _score) = decrypt_by_chars(&hex_to_ascii("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736"));
    println!("Result: {}, Code: {}", plain_text, code);
}


