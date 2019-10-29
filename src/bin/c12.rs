use cryptopals::oracle::*;
use cryptopals::aes::*;
use cryptopals::utils::*;
use cryptopals::base64::*;
use std::collections::HashMap;
use rand::*;
use std::f64::MAX as f64MAX;

const APPEND: &str =  "Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkgaGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBqdXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUgYnkK";
const MAX_KEY_LENGTH: u8 = 254;

pub fn main() {
    let key_len = 16; // I think we can use 24 and 32 as well but getting a panic when i try to
    let key = generate_random_key(key_len);
    
    println!("Key Length: {}", key_len);

    // Detect Key Length
    let mut prev_len: Option<u8> = None;
    let mut secret_key_len: Option<u8> = None;
    let mut unknown_text_len: Option<u8> = None;
    for length in (1..MAX_KEY_LENGTH+1) { // change this to infinite
        let plain_text = vec![b'A'; length as usize];
        let output = ebc_prefix_encrypter(&plain_text, &key);
        let output_len = output.len() as u8;
        match prev_len {
            None => {
                prev_len = Some(output_len);
            },
            Some(value) => {
                if output_len > value {
                    unknown_text_len = Some(value);
                    secret_key_len = Some(output_len - value);
                    break;
                }
            }
        }
    }
    let secret_key_len = secret_key_len.unwrap();
    let output = ebc_prefix_encrypter(&vec![b'A'; (secret_key_len*3) as usize], &key);
    println!("Is CBC? : (should be false) {}", is_cbc(&output, secret_key_len));
    
    let mut decrypted: Vec<u8> = Vec::new(); 
    // Originally started with just decrypting for secret key length. 
    // Realized after solving the block that we could just iterate over the whole input length.
    // If the unknown string is not a multiple of key length, the padding at the end would be unpredictable, so i am just breaking out of the loop in code
    let unknown_string_length = unknown_text_len.unwrap();
    for x in 0..unknown_string_length {
        let unknown_block_size = (unknown_string_length - x - 1) as usize;
        let mut trimmed_input = vec![b'A'; (unknown_block_size) as usize];

        println!("Trimmed Input is {:?}", trimmed_input);
        let mut map: HashMap<String, Vec<Vec<u8>>> = HashMap::new();
        // Map of Output => Input
        // Key type is string because i don't want to bother with vector equality right now
        let shifted_output = ebc_prefix_encrypter(&trimmed_input, &key);
        let shifted_block = shifted_output[0..unknown_string_length as usize].to_vec();
        trimmed_input.append(&mut decrypted.clone());
        eprint!(".");
        for ch in 0..=255  {
            let mut attempt = trimmed_input.clone();
            attempt.append(&mut vec![ch;1]);
            
            let attempt_output = ebc_prefix_encrypter(&attempt, &key);
            let attempt_block = attempt_output[0..unknown_string_length as usize].to_vec();

            let map_value = map.entry(bytes_to_string(&attempt_block)).or_insert(Vec::new());
            map_value.push(attempt.clone());
        }
        match map.get(&bytes_to_string(&shifted_block)) {
            Some(candidates) => {
                let mut min_score = f64MAX;
                let mut scored_candidate:Option<Vec<u8>> = None;
                for candidate in candidates {
                    let candidate_score = score(candidate);
                    if candidate_score < min_score {
                        min_score = candidate_score;
                        scored_candidate = Some(candidate.clone());
                    }
                }
                let mut scored_candidate = scored_candidate.unwrap();
                decrypted.push(scored_candidate.pop().unwrap());
            },
            None => {
                break;
            }
        }
        

    }
    println!("\n");
    println!("{}", bytes_to_string(&decrypted));
    
}

pub fn ebc_prefix_encrypter(plain_text: &[u8], key: &[u8]) -> Vec<u8> {
    let mut input = plain_text.clone().to_vec();
    input.append(&mut base64_to_bytes(APPEND));
    encrypt_ebc(input.as_slice(), &key)
}