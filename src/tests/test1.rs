#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_des() { ;

        println!("asdfasfasdfasdfadsfadsdsasfaasdasd");

        let section_bytes = [3, 97, 100, 114, 140, 3, 97, 97, 97];

        let a = parse_section(0, &section_bytes);

        assert_eq!(a, 123);
        // assert_eq!(byte_array, vec![0x01, 0x02, 0x03, 0x04, 0x01, 0x02, 0x03, 0x04]);
    }
}