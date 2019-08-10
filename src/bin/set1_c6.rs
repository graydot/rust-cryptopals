mod utils;
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

const KEY_LOWER: u32 = 1;
const KEY_UPPER: u32 = 40;
fn main() {
    println!("{}", utils::hamming_distance("this is a test", "wokka wokka!!!"));
    break_repeat_xor("data/set1_c6.txt");
}

fn break_repeat_xor(file_name: &str) -> std::io::Result<()> {
    let mut f = File::open(file_name)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    
    
    let mut hamming_distances = Vec::new();
    //FIXME
    for i in KEY_LOWER..(KEY_UPPER+1) {
        let mut chars = contents.chars();
        let mut first_charblob = "".to_string();
        let mut second_charblob = "".to_string();
        // normalized distance is 2 for a bunch of keysize values so trying more blobs
        let mut third_charblob = "".to_string();
        let mut fourth_charblob = "".to_string();
        for _ in 0..i {
            if let Some(ch) = chars.next() {
                first_charblob.push(ch);
            } else {
                break;
            }
            if let Some(ch) = chars.next() {
                second_charblob.push(ch);
            } else {
                break;
            }
            if let Some(ch) = chars.next() {
                third_charblob.push(ch);
            } else {
                break;
            }
            if let Some(ch) = chars.next() {
                fourth_charblob.push(ch);
            } else {
                break;
            }
        }
        let i_float = i as f32;
        let avg_distance = (
            utils::hamming_distance(&first_charblob, &second_charblob) as f32/i_float + 
            utils::hamming_distance(&second_charblob, &third_charblob) as f32/i_float +
            utils::hamming_distance(&third_charblob, &fourth_charblob) as f32/i_float)/3f32;
        hamming_distances.push((
            i.to_string(),
            avg_distance
        ));

        
    }
    hamming_distances.sort_by(|a,b| a.1.partial_cmp(&b.1).unwrap());
    
    let hamming_candidates = hamming_distances.drain(0..5);
    println!("{:?}", hamming_candidates);
    println!("{}", utils::score(&"abcc".to_string()));
    Ok(())
}
