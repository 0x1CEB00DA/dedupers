use ring::digest;


pub fn hash(in_bytes: &[u8]) ->  Vec<u8> {
    let dig = digest::digest(&digest::SHA256, in_bytes);
    let dig_repr: &[u8] = dig.as_ref();
    
    dig_repr.to_owned()
}



#[cfg(test)]
mod test_some_hashes {
    use super::*;
    use ring::test;

    #[test]
    fn test_hash() {
        let expected_hex =
            "09ca7e4eaa6e8ae9c7d261167129184883644d07dfba7cbfbc4c8a2e08360d5b";
        let expected: Vec<u8> = test::from_hex(expected_hex).unwrap();

        assert_eq!(expected, hash(b"hello, world"));
    }
}

