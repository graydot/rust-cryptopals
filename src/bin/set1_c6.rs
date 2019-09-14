use cryptopals::utils::*;
use cryptopals::base64::*;
use std::collections::HashMap;
use std::f64;

/// Minimum size of the key
const KEY_LOWER: u32 = 2;
/// Maximum size of the key
const KEY_UPPER: u32 = 40;
fn main() {
    assert_eq!(hamming_distance(b"this is a test", b"wokka wokka!!!"), 37);
    
    break_repeat_xor("data/set1_c6.txt");
}


/// Returns 3 minimum hamming distances
fn get_key_lengths(encrypted_text: &[u8]) -> Vec<u32> {
    let mut hamming_distances = Vec::new();
    //FIXME
    for i in KEY_LOWER..(KEY_UPPER+1) {
        hamming_distances.push((
            i,
            average_hamming_distance(encrypted_text, i)
        ));
    }
    hamming_distances.sort_by(|a,b| a.1.partial_cmp(&b.1).unwrap());
    hamming_distances[0..3].iter().
        map(|tuple| tuple.0 as u32).collect::<Vec<u32>>()
}

/// Breaks encrypted string into blocks of key_size, 
/// and returns transposed blocks as vectors of bytes
fn get_transposed_blocks(encrypted_text: &[u8], key_length: u32) -> Vec<Vec<u8>>{
    let mut map: HashMap<u32, Vec<u8>> = HashMap::new();

    encrypted_text.chunks(key_length as usize).for_each(|chunk| {
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
pub fn transpose_candidates_back(candidates: Vec<(Vec<u8>, char, f64)>) -> (Vec<u8>, String, f64){
    // To remove comments
    // Should return tuple (key, full decrypted text, score of full decrypted text)
    let (mut plain_text_map, key) = candidates.iter().fold((HashMap::new(), String::new()), |mut acc, candidate| {
        candidate.0.as_slice().iter().enumerate().for_each(|(i,ch)| {
            let map_value = acc.0.entry(i).or_insert(Vec::<u8>::new());
            map_value.push(*ch);
        });
        acc.1.push(candidate.1);
        acc
    });

    let mut plain_text = Vec::new();
    for i in 0..plain_text_map.len() {
        let transpose_block = plain_text_map.get_mut(&i).unwrap();
        plain_text.append(transpose_block);
    };
    let plain_text_score = score(plain_text.as_slice());
    (plain_text, key, plain_text_score)
}

fn break_repeat_xor(file_name: &str) -> std::io::Result<()> {
    
    let encrypted_base64: &str = &get_file_contents(file_name);
    let encrypted_v = base64_to_bytes(encrypted_base64);
    let encrypted_bytes = encrypted_v.as_slice();

    let key_lengths = get_key_lengths(&encrypted_bytes);
    let mut min_score: f64 = f64::MAX;
    let mut plain_bytes = Vec::new();
    let mut code_key = "".to_string();
    
    for i in key_lengths {
        let blocks = get_transposed_blocks(encrypted_bytes, i);
        let highscores: Vec<(Vec<u8>, char, f64)> = blocks.iter().map(|block|{
            
            decrypt_by_chars(&block.as_slice())
        }).collect();
        let (decrypted_text, key, full_text_score) = transpose_candidates_back(highscores);
        if full_text_score < min_score {
            min_score = full_text_score;
            plain_bytes = decrypted_text;
            code_key = key;
        }
    };
    let plain_text = plain_bytes.iter().fold(String::new(), |mut acc, byte| {
        acc.push(*byte as char);
        acc
    });
    println!("Plaintext: {}, Code: {}, Score: {}", plain_text, code_key, min_score);
    
    
    


    Ok(())
}
