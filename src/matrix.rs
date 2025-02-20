type MIndex = (isize, isize);
use std::iter::zip;
use std::ops::{Add, Index};

/// Error occurring during [`BitMatrix`] initialization
#[derive(Debug)]
pub enum BitMatrixConstructError {
    /// Every row of the table used to define a [`BitMatrix`]'s initial state must have the same number of columns
    RaggedTable(),
    /// A [`BitMatrix`] cannot have no cells
    EmptyTable(),
}

struct ToroidalBitMatrix {
    rows: usize,
    cols: usize,
    storage: Vec<u32>,
}

impl ToroidalBitMatrix {
    pub fn new(table: Vec<Vec<bool>>) -> Result<Self, BitMatrixConstructError> {
        let rows = table.len();
        let cols = if rows == 0 { 0 } else { table[0].len() };
        if cols == 0 {
            return Err(BitMatrixConstructError::EmptyTable());
        }

        // if the table is ragged (every column is not the same size) then we reject the input and return an Err result
        if table
            .iter()
            .map(|row| row.len() != cols)
            .fold(false, |a, b| a | b)
        {
            return Err(BitMatrixConstructError::RaggedTable());
        }

        let mut storage: Vec<u32> = Vec::with_capacity(rows * cols * u32::BITS as usize / 8);
        for chunk in table
            .into_iter()
            .flat_map(|r| r.into_iter())
            .collect::<Vec<bool>>()
            .chunks(u32::BITS as usize)
        {
            let mut next_element: u32 = 0;
            for (i, b) in chunk.to_vec().into_iter().enumerate() {
                next_element += if b { 2 ^ i as u32 } else { 0 };
            }
            storage.push(next_element);
        }

        Ok(ToroidalBitMatrix {
            rows,
            cols,
            storage,
        })
    }

    pub fn get(&self, idx: MIndex) -> bool {
        let row = idx.0.rem_euclid(self.rows as isize);
        let col = idx.1.rem_euclid(self.cols as isize);
        let bit_index = row as usize * self.cols + col as usize;

        let vec_idx: usize = bit_index / u32::BITS as usize;
        let element_offset: usize = bit_index % u32::BITS as usize;

        (self.storage[vec_idx] >> element_offset) & 1 != 0
    }

    pub fn set(&mut self, idx: MIndex, value: bool) {
        let row = idx.0.rem_euclid(self.rows as isize);
        let col = idx.1.rem_euclid(self.cols as isize);
        let bit_index = row as usize * self.cols + col as usize;

        let vec_idx: usize = bit_index / u32::BITS as usize;
        let element_offset: usize = bit_index % u32::BITS as usize;
        if value {
            self.storage[vec_idx] |= 1 << element_offset;
        } else {
            self.storage[vec_idx] &= !(1 << element_offset);
        }
    }

    pub fn bitwise_xor(&mut self, other: &mut ToroidalBitMatrix) {
        for (i, element) in (&mut self.storage).into_iter().enumerate() {
            *element ^= other.storage[i as usize];
        }
    }
}
