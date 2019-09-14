pub trait Pad {
    fn pk7pad(self, len: usize) -> Vec<u8>;
}

impl<T> Pad for T where T: AsRef<str> {
    fn pk7pad(self, len: usize) -> Vec<u8>{
        let mut byte_vec = self.as_ref().as_bytes().to_vec();
        let str_len = byte_vec.len();
        if str_len < len {
            byte_vec.resize(len, 4);
        }
        byte_vec
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_pk7pad(){
        assert_eq!("YELLOW SUBMARINE".pk7pad(20 as usize).as_slice(), b"YELLOW SUBMARINE\x04\x04\x04\x04");    
    }
}
