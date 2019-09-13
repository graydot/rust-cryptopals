use cryptopals::utils::*;
use cryptopals::base64::*;
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

/// Minimum size of the key
const KEY_LOWER: u32 = 2;
/// Maximum size of the key
const KEY_UPPER: u32 = 40;
fn main() {
    assert_eq!(hamming_distance("this is a test", "wokka wokka!!!"), 37);
    
    break_repeat_xor("data/set1_c6.txt");
}

fn get_file_contents(file_name: &str) -> std::io::Result<(String)> {
    let mut f = File::open(file_name)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents);
    
    Result::Ok(contents.lines().fold(String::new(), |mut acc, line| {

        acc.push_str(line.trim_end_matches('=').trim_start_matches('/'));
        acc
    }))
}

/// Returns 3 minimum hamming distances
fn get_key_lengths(encrypted_text: &str) -> Vec<u32> {
    let mut hamming_distances = Vec::new();
    //FIXME
    for i in KEY_LOWER..(KEY_UPPER+1) {
        let key_size: usize = i as usize;
        // let mut chars = contents.chars();

        let first_charblob = &encrypted_text[0..key_size];
        let second_charblob = &encrypted_text[key_size..key_size*2];
        let third_charblob = &encrypted_text[key_size*2..key_size*3];
        let fourth_charblob = &encrypted_text[key_size*3..key_size*4];
        let i_float = i as f32;
        let avg_distance = (
            hamming_distance(&first_charblob, &second_charblob) as f32 + 
            hamming_distance(&second_charblob, &third_charblob) as f32 +
            hamming_distance(&third_charblob, &fourth_charblob) as f32)/ (i_float * 3f32);
        hamming_distances.push((
            i,
            avg_distance
        ));
    }
    hamming_distances.sort_by(|a,b| a.1.partial_cmp(&b.1).unwrap());
hamming_distances[0..3].iter().
        map(|tuple| tuple.0 as u32).collect::<Vec<u32>>()
}

/// Breaks encrypted string into blocks of key_size, 
/// and returns transposed blocks as vectors of bytes
fn get_transposed_blocks(encrypted_text: &str, key_length: u32) -> Vec<Vec<u8>>{
    let mut map: HashMap<u32, Vec<u8>> = HashMap::new();

    encrypted_text.as_bytes().chunks(key_length as usize).for_each(|chunk| {
        chunk.iter().enumerate().for_each(|(block_index, character_byte)|{
            let block = map.entry(block_index as u32).or_insert(Vec::new());
            block.push(character_byte.clone());
        });
    });
    let mut blocks: Vec<Vec<u8>> = Vec::new();
    for i in 0..key_length {
        let block = map.get(&i).unwrap().clone();
        blocks.insert(i as usize, block);
    };
    blocks
}
pub fn transpose_candidates_back(candidates: Vec<(String, char, f64)>) -> (String, String, f64){
    // To remove comments
    // Should return tuple (key, full decrypted text, score of full decrypted text)
    let (plain_text_map, key) = candidates.iter().fold((HashMap::new(), String::new()), |mut acc, candidate| {
        candidate.0.chars().enumerate().for_each(|(i,ch)| {
            let map_value = acc.0.entry(i).or_insert(String::new());
            map_value.push(ch);
        });
        acc.1.push(candidate.1);
        acc
    });

    let mut plain_text = String::new();
    for i in 0..plain_text_map.len() {
        plain_text.push_str(plain_text_map.get(&i).unwrap());
    };
    let plain_text_score = score(&plain_text);
    (plain_text, key, plain_text_score)
}

fn break_repeat_xor(file_name: &str) -> std::io::Result<()> {
    
    let encrypted_base64: &str = &get_file_contents(file_name).unwrap();
    let encrypted_ascii: &str = &base64_to_ascii(encrypted_base64);
    // println!("{}", encrypted_ascii);

    let key_lengths = get_key_lengths(encrypted_ascii);
    // println!("Possible Key lengths {:?}", key_lengths);
    
    for i in key_lengths {
        println!("================");
        println!("================");
        println!("================");
        println!("================");
        let blocks = get_transposed_blocks(encrypted_ascii, i);
        let highscores: Vec<(String, char, f64)> = blocks.iter().map(|block|{
            let ascii_str = block.iter().fold(String::new(), |mut acc, ch|{
                let ch_char = *ch as char;
                acc.push(ch_char);
                acc
            });
            decrypt_by_chars(&ascii_str)
        }).collect();
        let (decrypted_text, key, full_text_score) = transpose_candidates_back(highscores);
        println!("Decrypted: {}", decrypted_text);
        println!("Key: ###### {} ######", key);
        println!("Decrypted Score: {}", full_text_score);
    };

    
    
    


    Ok(())
}
