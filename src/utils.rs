use std::collections::HashMap;

/// Returns the number of differing bits between two strings
pub fn hamming_distance(text1: &str, text2: &str) -> u32 {
    assert_eq!(text1.len(), text2.len());
    let distance = text1.chars().zip(text2.chars()).fold(0, {|sum, (c1, c2)|
    {
        let char1_val = c1 as u8;
        let char2_val = c2 as u8;
        sum + (char1_val ^ char2_val as u8).count_ones()
    }
    }); 
    distance
}

pub fn score(text: &String) -> i32 {
    let plain_histogram_raw = vec!(
        ("a",8.167,),("b",1.492,),("c",2.782,),("d",4.253,),("e",12.702,),("f",2.228,),("g",2.015,),("h",6.094,),("i",6.966,),("j",0.153,),("k",0.772,),("l",4.025,),("m",2.406,),("n",6.749,),("o",7.507,),("p",1.929,),("q",0.095,),("r",5.987,),("s",6.327,),("t",9.056,),("u",2.758,),("v",0.978,),("w",2.360,),("x",0.150,),("y",1.974,),("z",0.074)
    );
    let mut plain_histogram = HashMap::new();
    for tuple in plain_histogram_raw {
        plain_histogram.insert(tuple.0.to_string(), tuple.1);
    }
    let mut score: i32 = 0;
    let mut chi_score = 0f64;
    let lower_text = text.to_ascii_lowercase();

    let mut text_histogram = HashMap::new();
    // FIXME - may not be needed
    let lower_etaoin_shrldu = "etaoinshrldu";
    for ch in lower_text.chars() {
        if ch.is_alphabetic() {
            *text_histogram.entry(ch.to_string()).or_insert(0) += 1;
        }
        if lower_etaoin_shrldu.contains(ch) {
            score += 1;
        }
    }
    for (key, value) in plain_histogram.iter() {
        let mut expect = *value as f32;
        let mut exist = *text_histogram.entry(key.to_string()).or_insert(0) as f32;
        let chi_squared = f64::powf((expect - exist) as f64, 2 as f64) / expect as f64;
        chi_score += chi_squared;
    }
    // this is bad. 
    f64::abs(chi_score) as i32
}

/// Given a hex representation of a string, return a tuple with the highest ranked
/// String generated by XORing with different characters
pub fn decrypt(hex_string: &str) -> (String, char, i32) {
    let mut min_score: i32 = 99999;
    let mut result: String = "".to_string();
    let mut code = ' ';
    
    let ascii_str = hex_to_ascii(hex_string);
    for ch in (b'A'..b'Z').chain(b'0'..b'9') {
        let plain_text = xor(&ascii_str, ch as char);
        // Use the new score function to solve puzzle 4
        let score_val: i32 = score(&plain_text);
        if score_val < min_score {
            result = plain_text;
            min_score = score_val;
            code = ch as char;
        }
    }

    (result, code, min_score)
}

pub fn xor(encrypted: &str, code: char) -> String{
    
    let result = encrypted.chars().map(|a|
    {
        xor_char(a, code)
    }).collect();
    result
}

pub fn xor_char(text: char, code: char) -> char {
    (text as u8 ^ code as u8) as char
}

/// Returns the ASCII String when provided a hex representation of the string
pub fn hex_to_ascii(hex_str: &str) -> String {
    let mut chars = hex_str.chars();
    // FIXME: may fail if len is not even.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hamming_distance() {
        assert_eq!(hamming_distance("", ""), 0);
        assert_eq!(hamming_distance("this is a test", "wokka wokka!!!"), 37);
    }
}