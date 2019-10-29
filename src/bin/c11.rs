use cryptopals::oracle::*;
use rand::*;


pub fn main() {
    let input:Vec<u8> = (0..48).map(|_| {
        b'A'
    }).collect();
    for _ in 1..100 {
        let use_cbc = rand::thread_rng().gen_range(0,2) == 1;
        let output = encryption_oracle(&input, 16, use_cbc);
        let detected = is_cbc(&output, 16);
        assert_eq!(use_cbc, detected);
    }
    
}


