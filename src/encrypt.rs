// 2025 Steven Chiacchira
use crate::automata::Automaton;
use crate::matrix::{MatrixIndex, ToroidalBinaryMatrix, ToroidalBoolMatrix};
use crate::parse::{concat_bool_to_u8, explode_u8_to_bool};
use std::string::{self, FromUtf8Error};

/// Reads 4 bit values at `idx0`, `idx`, `idx2`, `idx3`, in `matrix`, then concatenates them into a
/// `u8`.
pub fn read_4_bits<T>(
    matrix: &T,
    idx0: MatrixIndex,
    idx1: MatrixIndex,
    idx2: MatrixIndex,
    idx3: MatrixIndex,
) -> u8
where
    T: ToroidalBinaryMatrix,
{
    let mut result: u8 = 0;
    for (i, idx) in [idx0, idx1, idx2, idx3].iter().enumerate() {
        result += if matrix.at(*idx) {
            2_u8.pow(i as u32)
        } else {
            0
        };
    }

    result
}

/// Applies the matrix scrambling algorithm $V$ explained in RFC-0.
fn scramble_matrix_256<T>(message_matrix: &mut T, key: &T)
where
    T: ToroidalBinaryMatrix,
{
    for row_block in 0..4 {
        // iterate over each row in the 'row block' and swap
        let block_offset: isize = 4 * row_block;
        for (row_offset, col_offset) in [0, 2, 1, 3].iter().enumerate() {
            let (r_offset, c_offset) = (row_offset as isize, *col_offset as isize);
            let row_swap_idx = read_4_bits(
                key,
                (block_offset + r_offset, c_offset),
                (block_offset + r_offset, 4 + c_offset),
                (block_offset + r_offset, 8 + c_offset),
                (block_offset + r_offset, 12 + c_offset),
            ) as isize;
            message_matrix.swap_rows(block_offset, row_swap_idx);
        }
    }
    for col_block in 0..4 {
        // iterate over each col in the 'col block' and swap
        let block_offset: isize = 4 * col_block;
        for (col_offset, row_offset) in [3, 0, 2, 1].iter().enumerate() {
            let (r_offset, c_offset) = (*row_offset as isize, col_offset as isize);
            let row_swap_idx = read_4_bits(
                key,
                (r_offset, block_offset + c_offset),
                (4 + r_offset, block_offset + c_offset),
                (8 + r_offset, block_offset + c_offset),
                (12 + r_offset, block_offset + c_offset),
            ) as isize;
            message_matrix.swap_rows(block_offset, row_swap_idx);
        }
    }
}

/// Applies the inverse matrix scrambling algorithm $V^(-1)$ explained in RFC-0.
fn unscramble_matrix_256<T>(message_matrix: &mut T, key: &T)
where
    T: ToroidalBinaryMatrix,
{
    for col_block in (0..4).rev() {
        // iterate over each col in the 'col block' and swap
        let block_offset: isize = 4 * col_block;
        for (col_offset, row_offset) in [3, 0, 2, 1].iter().enumerate().rev() {
            let (r_offset, c_offset) = (*row_offset as isize, col_offset as isize);
            let row_swap_idx = read_4_bits(
                key,
                (r_offset, block_offset + c_offset),
                (4 + r_offset, block_offset + c_offset),
                (8 + r_offset, block_offset + c_offset),
                (12 + r_offset, block_offset + c_offset),
            ) as isize;
            message_matrix.swap_rows(block_offset, row_swap_idx);
        }
    }

    for row_block in (0..4).rev() {
        // iterate over each row in the 'row block' and swap
        let block_offset: isize = 4 * row_block;
        for (row_offset, col_offset) in [0, 2, 1, 3].iter().enumerate().rev() {
            let (r_offset, c_offset) = (row_offset as isize, *col_offset as isize);
            let col_swap_idx = read_4_bits(
                key,
                (block_offset + r_offset, c_offset),
                (block_offset + r_offset, 4 + c_offset),
                (block_offset + r_offset, 8 + c_offset),
                (block_offset + r_offset, 12 + c_offset),
            ) as isize;
            message_matrix.swap_rows(block_offset, col_swap_idx);
        }
    }
}

/// Splits `message` into 256 bit blocks, represented as flat vectors.
/// The final block of `message` is not padded to 256 bits.
fn block_split_256_message(message: &str) -> Vec<Vec<bool>> {
    message
        .as_bytes()
        .to_vec()
        .chunks(256 / 8) // read each byte into a chunk of 256 bits (32 bytes)
        .map(|a| a.iter().map(|b| explode_u8_to_bool(*b)).flatten().collect())
        .collect()
}

/// Reconstructs a UTF-8 string from the bitstring `bits`, represented as a `Vec<bool>`.
pub fn reconstruct_message(bits: Vec<bool>) -> Result<String, string::FromUtf8Error> {
    let bytes: Vec<u8> = bits
        .chunks(u8::BITS as usize)
        .map(|b| concat_bool_to_u8(b.to_vec()))
        .collect();
    String::from_utf8(bytes)
}

/// Encrypts a 256 bit message block with the Talos algorithm.
fn encrypt_block_256(
    message_block: Vec<bool>,
    shift_automata: &mut Automaton,
    transpose_automata: &mut Automaton,
) -> Vec<bool> {
    let mut message_matrix = ToroidalBoolMatrix::from_storage(16, 16, message_block).unwrap();
    shift_automata.iter_rule(11);
    transpose_automata.iter_rule(11);

    scramble_matrix_256(&mut message_matrix, transpose_automata.get_state());
    let _ = message_matrix.bitwise_xor(transpose_automata.get_state());

    message_matrix.get_storage().to_vec()
}

/// Decrypts a 256 bit message block with the Talos algorithm.
fn decrypt_block_256(
    encrypted_block: Vec<bool>,
    shift_automata: &mut Automaton,
    transpose_automata: &mut Automaton,
) -> Vec<bool> {
    let mut message_matrix = ToroidalBoolMatrix::from_storage(16, 16, encrypted_block).unwrap();
    shift_automata.iter_rule(11);
    transpose_automata.iter_rule(11);

    let _ = message_matrix.bitwise_xor(transpose_automata.get_state());
    unscramble_matrix_256(&mut message_matrix, transpose_automata.get_state());

    message_matrix.get_storage().to_vec()
}

/// Encrypts a message with a 256 bit block using the Talos algorithm.
pub fn encrypt_message_256(
    message: &str,
    shift_automata: &mut Automaton,
    transpose_automata: &mut Automaton,
) -> Vec<bool> {
    let mut blocks = block_split_256_message(message);
    if let Some(last) = blocks.last_mut() {
        last.resize(16 * 16, false);
    }

    blocks
        .iter()
        .map(|b| encrypt_block_256(b.to_vec(), shift_automata, transpose_automata))
        .flatten()
        .collect()
}

/// Decrypts a message with a 256 bit block using the Talos algorithm.
pub fn decrypt_message_256(
    ciphertext: Vec<bool>,
    shift_automata: &mut Automaton,
    transpose_automata: &mut Automaton,
) -> Result<String, FromUtf8Error> {
    let message_bits = ciphertext
        .chunks(16 * 16)
        .map(|b| decrypt_block_256(b.to_vec(), shift_automata, transpose_automata))
        .flatten()
        .collect();
    reconstruct_message(message_bits)
}
