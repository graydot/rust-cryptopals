use itertools::Itertools;

fn main() {
    println!("{}", xor("1c0", "686"));
}

fn xor(buffer1: &str, buffer2: &str) -> String{
    assert_eq!(buffer1.len(), buffer2.len());
    let result = buffer1.chars().zip_eq(buffer2.chars()).map(|(a,b)|
    {
        let val1 = match a.to_digit(16){
            Some(num) => num,
            None => 0,
        } as u8;
        let val2 = match b.to_digit(16){
            Some(num) => num,
            None => 0,
        } as u8;
        let val3 = (val1 ^ val2) as u8;
        format!("{:x}", val3)
    }).collect();
    result
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_xor(){
        assert_eq!(xor("", ""), "");
        assert_eq!(xor("1c0111001f010100061a024b53535009181c", "686974207468652062756c6c277320657965"), "746865206b696420646f6e277420706c6179");

    }
}