#[cfg(test)]
mod tests {
    use crate::protocol::parse_section;
    use std::io::{Cursor, Write};
    use serde::Deserialize;
    use crate::constants::*;
    
    #[test]
    fn test_parse_section_string() {

        let section_bytes = [4, 97, 98, 99, 100, SERIALIZE_TYPE_STRING, 12, 97, 97, 97];

        let mut new_bytes_cursor = Cursor::new(Vec::new());
        
        parse_section(0, &section_bytes, &mut new_bytes_cursor);

        let s: String = bincode::deserialize(&new_bytes_cursor.clone().into_inner()).unwrap();

        assert_eq!(new_bytes_cursor.into_inner(), [3, 0, 0, 0, 0, 0, 0, 0, 97, 97, 97]);
        assert_eq!(s, "aaa".to_string());
    }


    #[test]
    fn test_parse_section_u64() {

        let section_bytes = [1, 97, SERIALIZE_TYPE_INT64, 0x1, 0x3, 0x5, 0x7, 0x9, 0xb, 0xd, 0xf];
        
        let mut new_bytes_cursor = Cursor::new(Vec::new());
        parse_section(0, &section_bytes, &mut new_bytes_cursor);

        let n: u64 = bincode::deserialize(&new_bytes_cursor.clone().into_inner()).unwrap();

        assert_eq!(new_bytes_cursor.into_inner(), [0x1, 0x3, 0x5, 0x7, 0x9, 0xb, 0xd, 0xf]);
        assert_eq!(n, 1084535218666537729);
    }


    #[test]
    fn test_parse_section_bool() {

        let section_bytes = [1, 97, SERIALIZE_TYPE_BOOL, 0x1];
        
        let mut new_bytes_cursor = Cursor::new(Vec::new());
        parse_section(0, &section_bytes, &mut new_bytes_cursor);

        let b: bool = bincode::deserialize(&new_bytes_cursor.clone().into_inner()).unwrap();

        assert_eq!(new_bytes_cursor.into_inner(), [0x1]);
        assert_eq!(b, true);
    }


    #[test]
    fn test_parse_section_object() {

        #[derive(Deserialize, Debug)]
        struct TestObject {
            number_key: u32,
        }

        let section_bytes = [1, 97, SERIALIZE_TYPE_OBJECT, 0x04, 1, 97, SERIALIZE_TYPE_INT32, 0x1, 0x1, 0x1, 0x1];
        
        let mut new_bytes_cursor = Cursor::new(Vec::new());
        parse_section(0, &section_bytes, &mut new_bytes_cursor);

        let test_object: TestObject = bincode::deserialize(&new_bytes_cursor.clone().into_inner()).unwrap();

        assert_eq!(new_bytes_cursor.into_inner(), [0x1, 0x1, 0x1, 0x1]);
        assert_eq!(test_object.number_key, 16843009);
    }
}