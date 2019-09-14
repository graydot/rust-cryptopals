use cryptopals::utils::*;
use cryptopals::base64::*;
use openssl::symm::{decrypt, Cipher};

const CODE:&[u8] = b"YELLOW SUBMARINE";

pub fn main() {
    let lines = get_file_lines("data/set1_c8.txt").unwrap();

    let min = lines.iter().map(|line|{
        let encrypted_vector = hex_to_bytes(&line).unwrap();
        let encrypted_bytes = encrypted_vector.as_slice();
        (average_hamming_distance(encrypted_bytes, 16 as u32), encrypted_vector)
    }).min_by(|a,b| {
        a.0.partial_cmp(&b.0).unwrap()
    });
    println!("{:?}", bytes_to_string(&min.unwrap().1));

    
    
    
    // let cipher = Cipher::aes_128_ecb();
    // let key = CODE;
    // let iv: Option<&[u8]> = None;

    // let output = decrypt(cipher, key, None, &encrypted_bytes);
    // println!("{}", bytes_to_string(output.unwrap().as_slice()));
}