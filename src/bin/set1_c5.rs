mod set1_c3;

fn main() {
    println!("{}",repeating_key_xor("Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal
", "ICE"));
}

pub fn repeating_key_xor(text: &str, code: &str) -> String {
    let mut code_chars = code.chars();
    let mut code_i = 0;
    let mut encrypted_text = String::new();
    for t in text.chars() {
        if t == '\n' {
            encrypted_text.push_str("\n");
            continue;
        }
        let code_char = match code_chars.next() {
            Some(ch) => ch,
            None => {
                code_chars = code.chars();
                match code_chars.next() {
                    Some(ch) => ch,
                    None => ' ',
                }
            }
        };      
        let xored_char = set1_c3::xor_char(t, code_char);
        let hex = ascii_2_hex(xored_char);
        encrypted_text.push_str(&hex);
    }
    encrypted_text
}

pub fn ascii_2_hex(text: char) -> String {
    let digit1: u8 = (text as u8) / 16;
    let digit2: u8 = (text as u8) % 16;
    format!("{:x}{:x}", digit1, digit2)
}