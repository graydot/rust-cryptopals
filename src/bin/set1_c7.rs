use cryptopals::utils::*;
use cryptopals::base64::*;
use cryptopals::aes::*;


const CODE:&[u8] = b"YELLOW SUBMARINE";

pub fn main() {
    let file_contents = get_file_contents("data/set1_c7.txt");
    let encrypted_vector = base64_to_bytes(&file_contents);
    let encrypted_bytes = encrypted_vector.as_slice();

    let output = decrypt_ebc(encrypted_bytes, CODE);
    println!("{}", bytes_to_string(output.as_slice()));
}