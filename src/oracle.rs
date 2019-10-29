use crate::aes::*;
use rand::*;
use rand::distributions::Alphanumeric;

pub fn generate_random_key(len:u8) -> Vec<u8> {
    // (0..len).map(|_| {
        // rand::thread_rng().gen::<u8>()
    // }).collect();
        rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len as usize)
        .map(|ch| {
            ch as u8
        })
        .collect()
}

pub fn generate_random_length_key(min_len: u8, max_len: u8) -> Vec<u8> {
    let phrase_len = rand::thread_rng().gen_range(min_len, max_len + 1);
    generate_random_key(phrase_len)
}

pub fn encryption_oracle(input: &[u8], key_length: u8, use_cbc: bool) -> Vec<u8> {
    let key = generate_random_key(key_length);
    let mut clear_text = generate_random_length_key(5, 10);
    clear_text.append(&mut input.to_vec());
    clear_text.append(&mut generate_random_length_key(5, 10));
    if use_cbc {
        encrypt_cbc(clear_text.as_slice(), &key, &generate_random_key(key_length))
    } else {
        encrypt_ebc(clear_text.as_slice(), &key)

    }
    
}

pub fn is_cbc(cipher_text: &[u8], block_length: u8) -> bool {
    let cipher_text = cipher_text.to_vec();
    let start = 0;
    let end = cipher_text.len() as u8 - block_length * 2;
    for i in (start..end) {
        let index1 = i as usize;
        let index2 = (block_length + i) as usize;
        let mut ebc = true;
        for j in (0 as usize..block_length as usize){
            if cipher_text[index1 + j] != cipher_text[index2 + j] {
                ebc = false; // inner counter, break and move to next starting position if false
                break;
            }
        };
        if ebc {
            return false // each loop check if it is ebc, if it is, return indicating it is not cbc
        }
    }
    true // nothing matched. this is cbc
}
