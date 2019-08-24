use cryptopals::utils::*;
fn main() {
    let (plain_text, code, _score) = decrypt("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
    println!("Result: {}, Code: {}", plain_text, code);
}


