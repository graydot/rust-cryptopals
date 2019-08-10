
fn main() {
    set1::challenge1::run();
}
mod set1 {
    pub mod challenge1 {
        use std::num::ParseIntError;
        const HEX_INPUT: &str = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        const B64_OUTPUT: &str = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"; 
        const BASE64_MAP: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        
        
        pub fn run(){
            // decode hex-encoded string into bytes
            let bytes_from_hex = hex_decode(HEX_INPUT);
            // encode bytes into base64
            let base64_string = base64_encode(&bytes_from_hex.unwrap());
            assert_eq!(&base64_string, B64_OUTPUT);
        }

        fn hex_decode(input: &str) -> Result<Vec<u8>, ParseIntError> {
            input.as_bytes().chunks(2).map(|chunk| {
                u8::from_str_radix(&format!("{}{}",char::from(chunk[0]), char::from(chunk[1]) ), 16)
            }).collect::<Result<Vec<u8>, ParseIntError>>()
        }

        fn base64_encode(input: &[u8]) -> String {
            input.chunks(3).fold(String::new(), |mut acc, chunk| {
                let first = chunk[0] << 2 | chunk[1] >> 2;
                let second = (chunk[1] & 0b0000_0011) << 4  | chunk[2];
                acc.push_str(&format!("{}", BASE64_MAP[first as usize]));
                acc.push_str(&format!("{}", BASE64_MAP[second as usize]));
                acc
            })
        }
    }
}

