use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use std::num::ParseIntError;


/// Returns the number of differing bits between two strings
pub fn hamming_distance(text1: &[u8], text2: &[u8]) -> u32 {
    assert_eq!(text1.len(), text2.len());
    let distance = text1.iter().zip(text2.iter()).fold(0, {|sum, (c1, c2)|
    {
        sum + (c1 ^ c2).count_ones()
    }
    }); 
    distance
}

pub fn average_hamming_distance(text: &[u8], key_size: u32 ) -> f32 {
    let i = key_size as usize;
    let first_charblob = &text[0..i];
    let second_charblob = &text[i..i*2];
    let third_charblob = &text[i*2..i*3];
    let fourth_charblob = &text[i*3..i*4];
    let i_float = i as f32;
    let avg_distance = (
        hamming_distance(&first_charblob, &second_charblob) as f32 + 
        hamming_distance(&second_charblob, &third_charblob) as f32 +
        hamming_distance(&third_charblob, &fourth_charblob) as f32)/ (i_float * 3f32);

    avg_distance
}

pub fn score(text: &[u8]) -> f64 {
    let plain_histogram_raw = vec!(
        ("a",8.167,),
        ("b",1.492,),
        ("c",2.782,),
        ("d",4.253,),
        ("e",12.702,),
        ("f",2.228,),
        ("g",2.015,),
        ("h",6.094,),
        ("i",6.966,),
        ("j",0.153,),
        ("k",0.772,),
        ("l",4.025,),
        ("m",2.406,),
        ("n",6.749,),
        ("o",7.507,),
        ("p",1.929,),
        ("q",0.095,),
        ("r",5.987,),
        ("s",6.327,),
        ("t",9.056,),
        ("u",2.758,),
        ("v",0.978,),
        ("w",2.360,),
        ("x",0.150,),
        ("y",1.974,),
        ("z",0.074)
    );
    let mut plain_histogram = HashMap::new();
    for tuple in plain_histogram_raw {
        plain_histogram.insert(tuple.0.to_string(), tuple.1);
    }
    let mut chi_score:f64 = 0.0;

    let mut text_histogram = HashMap::new();
    let mut nonalpha_count = 0;
    let mut uppercase_count = 0;

    for byte in text {
        let ch = *byte as char;
        if ch.is_alphabetic() {
            *text_histogram.entry(ch.to_lowercase().to_string()).or_insert(0) += 1;
            if ch.is_uppercase() {
                uppercase_count += 1;
            }
        } else {
            nonalpha_count += 1;   
        }
    }
    let len = text.len();
    for (key, value) in plain_histogram.iter() {
        let expect = *value as f32;
        let exist = (*text_histogram.entry(key.to_string()).or_insert(0) as f32)/(len as f32) * 100.0;
        // println!("Char: {}, Expected: {}%, Exist: {}%", key, expect, exist);
        let chi_squared = f64::abs(f64::powf((exist - expect) as f64, 2 as f64) / (expect + exist) as f64);
        chi_score += chi_squared;
    }
    (chi_score + uppercase_count as f64/(len - nonalpha_count) as f64) * (nonalpha_count as f64/len as f64)
}

/// Given an ascii representation of a string, return a tuple with the highest ranked
/// String generated by XORing with different characters
pub fn decrypt_by_chars(bytes: &[u8]) -> (Vec<u8>, char, f64) {
    let mut min_score: f64 = score(bytes);
    let mut result: Vec<u8> = bytes.to_vec();
    let mut code = ' ';
    for ch in (b'a'..b'z').chain(b'0'..b'9').chain(b'A'..b'Z') {
        let plain_text = xor(bytes, &ch);
        // Use the new score function to solve puzzle 4
        let score_val: f64 = score(&plain_text);
        if score_val < min_score {
            result = plain_text;
            min_score = score_val;
            code = ch as char;
        }
    }

    (result, code, min_score)
}

pub fn xor(encrypted: &[u8], code: &u8) -> Vec<u8>{
    
    let result = encrypted.iter().map(|a|
    {
        xor_byte(a, code)
    }).collect();
    result
}

pub fn xor_byte(text: &u8, code: &u8) -> u8 {
    (text ^ code)
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

pub fn bytes_to_string(bytes: &[u8]) -> String {
    bytes.iter().fold(String::new(), |mut acc, byte|{
        acc.push(*byte as char);
        acc
    })
}
pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, ParseIntError> {
    hex.as_bytes().chunks(2).map(|chunk| {
        u8::from_str_radix(&format!("{}{}", char::from(chunk[0]), char::from(chunk[1])), 16)
    }).collect()
}

pub fn ascii_to_hex(text: char) -> String {
    let digit1: u8 = (text as u8) / 16;
    let digit2: u8 = (text as u8) % 16;
    format!("{:x}{:x}", digit1, digit2)
}


pub fn get_file_contents(file_name: &str) -> String {
    let lines = get_file_lines(file_name).unwrap();
    lines.iter().fold(String::new(), |mut acc, line| {
        acc.push_str(line);
        acc
    })
}

pub fn get_file_lines(file_name: &str) -> std::io::Result<(Vec<String>)> {
    let mut f = File::open(file_name)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents);
    Result::Ok(contents.lines().fold(Vec::new(), |mut acc, line| {
        acc.push(line.trim_end_matches('=').to_string());
        acc
    }))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hamming_distance() {
        assert_eq!(hamming_distance(b"", b""), 0);
        assert_eq!(hamming_distance(b"this is a test", b"wokka wokka!!!"), 37);
    }
    
}
