use cryptopals::utils::*;

fn main() {
    assert_eq!(
        repeating_key_xor(
            "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal", 
            "ICE"
        ),
        "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f"
    );
}

pub fn repeating_key_xor(text: &str, code: &str) -> String {
    let mut code_bytes = code.as_bytes().iter().cycle();
    let mut code_i = 0;
    let mut encrypted_text = String::new();
    for t in text.as_bytes().iter() {
        let code_byte = code_bytes.next().unwrap();
        let xored_byte = xor_byte(t, code_byte);
        let hex = ascii_to_hex(xored_byte as char);
        encrypted_text.push_str(&hex);
    }
    encrypted_text
}

