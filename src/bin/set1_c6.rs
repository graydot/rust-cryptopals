use cryptopals::utils::*;

use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

/// Minimum size of the key
const KEY_LOWER: u32 = 1;
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
    Result::Ok(contents)
}

/// Returns 3 minimum hamming distances
fn get_key_lengths(encrypted_text: &str) -> &[u32] {
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
            hamming_distance(&first_charblob, &second_charblob) as f32/i_float + 
            hamming_distance(&second_charblob, &third_charblob) as f32/i_float +
            hamming_distance(&third_charblob, &fourth_charblob) as f32/i_float)/3f32;
        hamming_distances.push((
            i,
            avg_distance
        ));

        
    }
    hamming_distances.sort_by(|a,b| a.1.partial_cmp(&b.1).unwrap());
    let key_lengths = &hamming_distances[0..3].iter().map(|tuple| tuple.0 as u32).collect();
    key_lengths
}

/// Breaks encrypted string into blocks of key_size, 
/// and returns transposed blocks as hex representations of characters
fn get_transposed_blocks(encrypted_text: &str, key_length: u32) -> Vec<String>{
    let mut map: HashMap<usize, Vec<u8>> = HashMap::new();

    encrypted_text.as_bytes().chunks(key_length as usize).for_each(|chunk| {
        chunk.iter().enumerate().for_each(|(block_index, character_byte)|{
            let block = map.entry(block_index).or_insert(Vec::new());
            block.push(character_byte.clone());
        });
    });
    let blocks: Vec<String> = Vec::new();
    for i in 0..key_length {
        let block = map.entry(i as usize);
        block.iter().map(|byte| {byte.as_hex}).flatten_map(|something|).fold(string)
    };
    
}


fn break_repeat_xor(file_name: &str) -> std::io::Result<()> {
    
    let contents_str: &str = &get_file_contents(file_name).unwrap();
    let key_lengths = get_key_lengths(contents_str);
    for i in key_lengths {
        let blocks = get_blocks(contents_str, i);

    }

    
    
    


    Ok(())
}
