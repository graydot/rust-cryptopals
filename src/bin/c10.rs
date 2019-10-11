use cryptopals::aes::*;
use cryptopals::utils::*;
use cryptopals::base64::*;

pub fn main() {
    let key = "YELLOW SUBMARINE".as_bytes();
    let iv = vec![0u8; key.len()];
    let text = "abcabcabcabababababbabab".as_bytes();
    let encrypted_text = encrypt_cbc(
        text, 
        key, 
        &iv);
    let decrypted = decrypt_cbc(
        &encrypted_text, 
        key, 
        &iv);
    assert_eq!(text, decrypted.as_slice());
    let file_contents = get_file_contents("data/c10.txt");
    let base64_contents = base64_to_bytes(&file_contents);
    let decrypted_bytes = decrypt_cbc(&base64_contents, key, &iv);
    println!("{}", bytes_to_string(&decrypted_bytes));
}