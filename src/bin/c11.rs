use cryptopals::aes::*;
use rand::*;


pub fn main() {
    let input:Vec<u8> = (0..48).map(|_| {
        b'a'
    }).collect();
    let use_cbc = rand::thread_rng().gen_range(0,2) == 1;
    let output = encryption_oracle(&input, 16, use_cbc);
    let detected = is_cbc(&output, 16);
    assert_eq!(use_cbc, detected);
}
pub fn generate_random_key(len:u8) -> Vec<u8> {
    (0..len).map(|_| {
        rand::thread_rng().gen::<u8>()
    }).collect()
}

pub fn generate_random_phrase(min_len: u8, max_len: u8) -> Vec<u8> {
    let phrase_len = rand::thread_rng().gen_range(min_len, max_len + 1);
    generate_random_key(phrase_len)
}

pub fn encryption_oracle(input: &[u8], key_length: u8, use_cbc: bool) -> Vec<u8> {
    let key = generate_random_key(key_length);
    let mut clear_text = generate_random_phrase(5, 10);
    clear_text.append(&mut input.to_vec());
    clear_text.append(&mut generate_random_phrase(5, 10));
    if use_cbc {
        encrypt_cbc(clear_text.as_slice(), &key, &generate_random_key(key_length))
    } else {
        encrypt_ebc(clear_text.as_slice(), &key)

    }
    
}

pub fn is_cbc(cipher_text: &[u8], block_length: u8) -> bool {
    let cipher_text = cipher_text.to_vec();
    let index1 = block_length as usize;
    let index2 = (block_length * 2) as usize;
    let mut matches = true;
    for i in (0 as usize..block_length as usize){
        if cipher_text[index1 + i] != cipher_text[index2 + i] {
            matches = false;
            break;
        }
    };
    !matches
}


