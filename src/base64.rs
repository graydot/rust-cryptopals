use std::num::ParseIntError;
const BASE64_MAP: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, ParseIntError> {
    hex.as_bytes().chunks(2).map(|chunk| {
        u8::from_str_radix(&format!("{}{}", char::from(chunk[0]), char::from(chunk[1])), 16)
    }).collect()
}

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
