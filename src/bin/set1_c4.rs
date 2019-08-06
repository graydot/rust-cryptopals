mod set1_c3;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;


fn main() -> std::io::Result<()> {
    let mut f = File::open("data/set1_c4.txt")?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let lines = get_lines(&contents);
    let mut max_score = 0;
    let mut max_elem = String::new();

    for line in lines {
        let (scored_string, _code) = return_best_scored(&line);
        let scored_score = set1_c3::score_etaoin_shrdlu(&scored_string);
        if scored_score > max_score {
            max_elem = scored_string;
            max_score = scored_score;
        }   
    }
    println!("{}", max_elem);
    Ok(())
}

/// Takes a line with hex characters and returns highest stored ascii xored
/// string. Return unchanged if that score is higher
fn return_best_scored(line: &str) -> (String, char) {
    let string = set1_c3::hex_to_ascii(&line);
    // println!("{}", string);
    let original_score: i32 = set1_c3::score_etaoin_shrdlu(&string);
    let (decrypted, code, new_score) = set1_c3::decrypt(&line);
    if new_score > original_score {
        return (decrypted, code)
    }
    (string, ' ')
}

fn get_lines(contents: &str) -> Vec<String> {
    let mut vec: Vec<String> = Vec::new();
    for line in contents.lines() {
        vec.push(line.to_string());
    }
    
    vec
}