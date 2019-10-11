use crate::utils::*;
use crate::pk7::*;
use aes::block_cipher_trait::generic_array::GenericArray;
use aes::block_cipher_trait::BlockCipher;
use aes::Aes128;


pub fn encrypt_cbc(plaintext: &[u8], key: &[u8], iv: &[u8]) -> Vec<u8> {
    assert_eq!(key.len(), iv.len());

    let mut local_iv = iv.clone().to_vec();
    let cipher_key = GenericArray::from_slice(key);
    let cipher = Aes128::new(&cipher_key);

    // FixMe: Replace with PK7
    let target_length = match plaintext.len() % key.len() {
        0 => plaintext.len() + key.len(),
        _ => {
            ((plaintext.len() / key.len()) + 1) * key.len()
        }
    };
    let mut input = plaintext.to_vec();
    // padding byte: if padding length is 0, then use block size 
    //  else use padding length
    let padding_byte = match target_length - plaintext.len() {
        0 => key.len() as u8,
        ch => ch as u8,
    };
    input.resize(target_length, padding_byte);
    let encrypted_blocks: Vec<Vec<u8>> = input.chunks(key.len()).map(|chunk| {
        let xored_block: Vec<u8> = chunk.iter().zip(local_iv.iter()).map(|(t_byte, i_byte)|{
            *t_byte ^ *i_byte
        }).collect();
        let xored_block = &xored_block;
        let mut xored_block = GenericArray::clone_from_slice(&xored_block);
        cipher.encrypt_block(&mut xored_block);
        let xored_block = xored_block.to_vec();

        local_iv = xored_block.clone();
        xored_block
    }).collect();
    let encrypted_bytes: Vec<u8> = encrypted_blocks.into_iter().flatten().collect::<Vec<u8>>();
    encrypted_bytes

}

pub fn decrypt_cbc(cipher_text: &[u8], key: &[u8], iv: &[u8]) -> Vec<u8> {
    assert_eq!(key.len(), iv.len());
    let mut local_iv: Vec<u8> = iv.clone().to_vec();
    let cipher_key = GenericArray::from_slice(key);
    let cipher = Aes128::new(&cipher_key);
    let decrypted_blocks: Vec<Vec<u8>> = cipher_text.chunks(key.len()).map(|chunk| {
        let mut block = GenericArray::clone_from_slice(&chunk);
        cipher.decrypt_block(&mut block);
        let aes_decrypted_text = block.to_vec();
        
        let xored_block: Vec<u8> = aes_decrypted_text.iter().zip(local_iv.iter()).map(|(t_byte, i_byte)|{
            *t_byte ^ *i_byte
        }).collect();
        
        local_iv = chunk.clone().to_vec();
        xored_block
    }).collect::<Vec<Vec<u8>>>();
    let mut decrypted_bytes: Vec<u8> = decrypted_blocks.into_iter().flatten().
        collect::<Vec<u8>>();
    let bytes_len = decrypted_bytes.len();
    let padding_len: usize = decrypted_bytes.remove(bytes_len-1) as usize;
    let values_len = bytes_len - padding_len; 
    let (decrypted_bytes, padding) = decrypted_bytes.split_at(values_len);
    
    decrypted_bytes.to_vec()
}
