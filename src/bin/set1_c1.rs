use itertools::Itertools;

const BASE64_MAP: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
fn main() {
    let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d".to_string();
    let output = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t".to_string();
    assert_eq!(hex_2_base64(&input), output);
}
/// # Examples
/// ```
/// let hex = String::new("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
/// let base64 = String::new("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
/// assert_eq!(hex_2_base64(&hex), base64);
/// ```
fn hex_2_base64(hex: &String) -> String {
    // println!("Chars: {:?}", hex.chars());
    let bytes = hex.chars().map(|ch| 
        match ch.to_digit(16) {
            Some(num) => num,
            None => 0,
    } as u16
    );
    let mut base64_values: Vec<u16> = Vec::new(); 
    for mut combination in &bytes.chunks(3){
        let elem0 = match combination.next() {
            Some(num) => num,
            None => 0,
        };
        let elem1 = match combination.next() {
            Some(num) => num,
            None => 0,
        }; 
        let elem2 = match combination.next() {
            Some(num) => num,
            None => 0,
        };
        let two_six_bits = (elem0 * u16::pow(2, 8) + elem1 * u16::pow(2, 4) + elem2)as u16;
        let two_pow_6 = u8::pow(2,6) as u16;
        base64_values.insert(0, two_six_bits/two_pow_6);
        base64_values.insert(0, two_six_bits % two_pow_6);
    };

    let base64_strings: Vec<&str>= base64_values.iter().map(|value|
    {
        let start = *value as usize;
        let end = start + 1;
        match BASE64_MAP.get(start..end) {
            Some(chr) => chr,
            None => "",
        }
    }).rev().collect();
    let return_value = base64_strings.iter().fold(String::new(), |acc, &str_val| {
        acc + str_val
    });
    return_value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_2_base64(){
        assert_eq!(hex_2_base64(&"".to_string()), "".to_string());
        let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d".to_string();
        let output = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t".to_string();
        assert_eq!(hex_2_base64(&input), output);

    }
}