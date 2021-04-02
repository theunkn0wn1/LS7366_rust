/// Converts a vector of length 4 into a u32
pub(crate) fn vec_to_i64(data: &[u8]) -> i64 {
    if data.len() > 4 {
        panic!("payload too big!, got {:?}", data)
    }
    let mut result: i64 = 0x00;
    let mut i = 0;
    for byte in data.iter().rev() {
        let converted_byte: i64 = *byte as i64;

        result += converted_byte << i * 8;
        i += 1;
    }
    result
}
#[test]
fn test_vec_to_u32() {
    assert_eq!(vec_to_i64(&vec![0xDE, 0xAD, 0xBE, 0xEF]), 0xDEADBEEF);
    assert_eq!(vec_to_i64(&vec![0x00, 0x00, 0x00, 0x0]), 0x0000000)
}
