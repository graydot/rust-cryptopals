use cryptopals::utils::*;
use std::fs::File;
use std::io::Read;
use std::f64;



fn main() -> std::io::Result<()> {
    let mut f = File::open("data/set1_c4.txt")?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let lines = get_lines(&contents);
    let mut min_score = f64::MAX;
    let mut min_elem = "".to_string();

    for line in lines {
        let (scored_string, _, scored_score) = decrypt_by_chars(&hex_to_ascii(&line));
        // println!("{}", scored_score);
        if scored_score < min_score {
            min_elem = scored_string;
            min_score = scored_score;
        }   
    }
    println!("{}", min_elem);
    Ok(())
}


fn get_lines(contents: &str) -> Vec<String> {
    let mut vec: Vec<String> = Vec::new();
    for line in contents.lines() {
        vec.push(line.to_string());
    }
    
    vec
}