/// Converts a vector of length 4 into a u32
pub(crate) fn vec_to_u32(data: &[u8]) -> u32 {
    if data.len()>4{
        panic!("payload too big!, got {:?}", data)
    }
    let mut result: u32 = 0x00;
    let mut i = 0;
    for byte in data.iter().rev() {
        let converted_byte: u32 = *byte as u32;

        result += converted_byte << i * 8;
        i += 1;
    }
    result
}
#[test]
fn test_vec_to_u32(){
    assert_eq!(vec_to_u32(&vec![0xDE, 0xAD, 0xBE, 0xEF]), 0xDEADBEEF);
    assert_eq!(vec_to_u32(&vec![0x00, 0x00, 0x00, 0x0]), 0x0000000)
}