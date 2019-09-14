use std::num::ParseIntError;
use std::collections::HashMap;

const BASE64_MAP: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";



pub fn bytes_to_base64(bytes: &[u8]) -> String {
    bytes.chunks(3).fold(String::new(), |mut acc, chunk| {
        let first = (chunk[0] & 0b1111_1100) >> 2;
        let second = (chunk[0] & 0b0000_0011) << 4 | chunk[1] >> 4;
        let third = (chunk[1] & 0b0000_1111) << 2 | chunk[2] >> 6;
        let fourth = chunk[2] & 0b0011_1111;
        acc.push_str(&format!("{}", char::from(BASE64_MAP[first as usize]))); 
        acc.push_str(&format!("{}", char::from(BASE64_MAP[second as usize]))); 
        acc.push_str(&format!("{}", char::from(BASE64_MAP[third as usize]))); 
        acc.push_str(&format!("{}", char::from(BASE64_MAP[fourth as usize]))); 

        acc
    })
}

pub fn base64_to_ascii(text: &str) -> String {
    // FIXME: cache this
    let mut base64_rev_map = BASE64_MAP.iter().enumerate().fold(HashMap::new(), |mut acc, (i,ch)|{
        acc.insert(ch, i as u8);
        acc
    });

    base64_rev_map.insert(&(0 as u8), 0 as u8);
    let result = text.as_bytes().chunks(4).fold(String::new(), |mut acc, chunk| {
        let mut chunk_vec = chunk.to_vec();
        for _ in 0..(4-chunk_vec.len()) {
            chunk_vec.push(0 as u8);
        }
        let first = base64_rev_map.get(&chunk_vec.remove(0)).unwrap();
        let second = base64_rev_map.get(&chunk_vec.remove(0)).unwrap();
        let third = base64_rev_map.get(&chunk_vec.remove(0)).unwrap();
        let fourth = base64_rev_map.get(&chunk_vec.remove(0)).unwrap();
    

        acc.push((first << 2 | second >> 4) as char);
        acc.push((second << 4 | third >> 2) as char);
        acc.push((third << 6 | fourth) as char);

        acc
    });
    result
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_bytes(){
        assert_eq!(hex_to_bytes(&"".to_string()), "".to_string());
        let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d".to_string();
        let output = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t".to_string();
        assert_eq!(hex_to_bytes(&input), output);

    }

    fn test_bytes_to_base64(){

    }
}
