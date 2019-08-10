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



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hamming_distance() {
        assert_eq!(hamming_distance("", ""), 0);
        assert_eq!(hamming_distance("this is a test", "wokka wokka!!!"), 37);
    }
}