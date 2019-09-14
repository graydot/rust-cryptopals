use cryptopals::base64::*;
use cryptopals::utils::*;

fn main() {
    let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d".to_string();
    let output = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t".to_string();
    let bytes = hex_to_bytes(&input.to_string()).unwrap();
    let base64 = bytes_to_base64(&bytes);
    assert_eq!(base64, output);
    // assert_eq!(hex_2_base64(&input), output);
}

