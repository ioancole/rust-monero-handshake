#[cfg(test)]
mod tests {
    use crate::protocol::parse_varint;
    
    #[test]
    fn test_parse_varint_3() {
        let varint_bytes = [12];
        let varint_result = parse_varint(0, &varint_bytes);
        assert_eq!(varint_result, [3, 1]);
    }

    #[test]
    fn test_parse_varint_250() {
        let varint_bytes = [0xe9, 0x03];
        let varint_result = parse_varint(0, &varint_bytes);
        assert_eq!(varint_result, [250, 2]);
    }

    #[test]
    fn test_parse_varint_100() {
        let varint_bytes = [0x91, 0x01];
        let varint_result = parse_varint(0, &varint_bytes);
        assert_eq!(varint_result, [100, 2]);
    }
}