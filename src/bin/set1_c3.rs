
fn main() {
    let (plain_text, code) = decrypt("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
    println!("Result: {}, Code: {}", plain_text, code);
}

fn decrypt(text: &str) -> (String, char) {
    // decrypt with all characters
    // score each variant based on on number of e characters
    let mut max_score = 0;
    let mut result: String = "".to_string();
    let mut code = ' ';
    
    let char_codes = decode(text);
    for ch in b'A'..b'Z' {
        let plain_text = xor(&char_codes, ch as char);
        let score_val = score(&plain_text);
        if score_val > max_score {
            result = plain_text;
            max_score = score_val;
            code = ch as char;
        }
    }

    (result, code)
}

fn xor(encrypted: &str, code: char) -> String{
    
    let result = encrypted.chars().map(|a|
    {
        (a as u8 ^ code as u8) as char
    }).collect();
    result
}

fn decode(hex_str: &str) -> String {
    let mut chars = hex_str.chars();
    // may fail if len is not even.
    let mut return_string = String::new();
    while chars.size_hint().0 > 0 {
        let hex1: u8 = match chars.next() {
            Some(ch) => match ch.to_digit(16) {
                Some(val) => val as u8,
                None => 0,
            },
            None => 0
        };
        let hex2: u8 = match chars.next() {
            Some(ch) => match ch.to_digit(16) {
                Some(val) => val as u8,
                None => 0,
            },
            None => 0
        };
        let ascii_val = hex1 * u8::pow(2, 4) + hex2;
        return_string.push(ascii_val as char);
    }
        
    
    return_string
}

fn score(plain_text: &String) -> u32 {
    plain_text.chars().filter(|ch| { 
        ch.is_ascii_alphabetic()
        }).count() as u32
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_xor(){
        assert_eq!(xor("", ""), "");
        assert_eq!(xor("1c0111001f010100061a024b53535009181c", "686974207468652062756c6c277320657965"), "746865206b696420646f6e277420706c6179");

    }
}