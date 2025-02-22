// 2025 Steven Chiacchira

/// Transforms a `u8` into a `Vec<bool>` containing its binary representation.
/// See also [`concat_bool_to_u8`].
pub fn explode_u8_to_bool(byte: u8) -> Vec<bool> {
    let mut result = Vec::with_capacity(u8::BITS as usize);
    for i in 0..(u8::BITS as usize) {
        result.push((byte >> i) & 1 == 1);
    }

    result
}

/// Transforms a series of bytes into a series of bools containing the binary representation of
/// the bytes.
/// ex.
/// ```txt
/// [1, 2] -> [0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0]
/// ```
/// 
/// See also [`concat_bool_to_u8_vec`].
pub fn explode_u8_to_bool_vec(bytes: Vec<u8>) -> Vec<bool> {
    bytes
        .iter()
        .map(|b| explode_u8_to_bool(*b))
        .flatten()
        .collect()
}

/// Concatenates a bitstring represented as a `Vec<bool>` into a `u8`.
/// See also [`explode_u8_to_bool`].
pub fn concat_bool_to_u8(bits: Vec<bool>) -> u8 {
    debug_assert!(bits.len() <= 8);
    let mut result = 0;
    for (i, bit) in bits.into_iter().enumerate() {
        result += 2_u8.pow(i as u32) * bit as u8
    }

    result
}

/// Concatenates a bitstring represented as a `Vec<bool>` into a series of `u8`s.
/// See also [`explode_u8_to_bool_vec`].
pub fn concat_bool_to_u8_vec(bits: Vec<bool>) -> Vec<u8> {
    bits.chunks(u8::BITS as usize)
        .map(|b| concat_bool_to_u8(b.to_vec()))
        .collect()
}

#[allow(dead_code)]
fn concat_u8_to_u32(bytes: Vec<u8>) -> u32 {
    let mut result = 0;
    for (i, byte) in bytes.into_iter().enumerate() {
        result += 16_u32.pow(i as u32) * byte as u32;
    }

    result
}
