use cryptopals::utils::*;
use cryptopals::base64::*;
use std::collections::HashMap;


const CODE:&[u8] = b"YELLOW SUBMARINE";

pub fn main() {
    let lines = get_file_lines("data/set1_c8.txt").unwrap();

    let min = lines.iter().enumerate().map(|(lno,line)|{
        let encrypted_vector = hex_to_bytes(&line).unwrap();
        let encrypted_bytes = encrypted_vector.as_slice();
        let mut map = HashMap::new();
        encrypted_bytes.chunks(16).for_each(|ch| {
            let entry = map.entry(ch).or_insert(0u32);
            *entry += 1;
        });
        let total: u32 = map.values().sum();
        let entries = map.len() as u32;
        let score: u32 = total - entries;
        (score, lno, encrypted_vector)
    }).max_by(|a,b| {
        a.0.partial_cmp(&b.0).unwrap()
    });
    let (score, line_no, vector) = &min.unwrap();
    println!("Line: {}, Value: {:?}, Score: {}", line_no, bytes_to_string(vector), score);

    
    
    
    // let cipher = Cipher::aes_128_ecb();
    // let key = CODE;
    // let iv: Option<&[u8]> = None;

    // let output = decrypt(cipher, key, None, &encrypted_bytes);
    // println!("{}", bytes_to_string(output.unwrap().as_slice()));
}