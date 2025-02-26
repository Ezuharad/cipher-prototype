// 2025 Steven Chiacchira
use crate::matrix::{MatrixConstructError, MatrixIndex, MatrixOpError, ToroidalBinaryMatrix};

#[derive(Debug, Clone)]
pub struct ToroidalBitMatrix {
    pub rows: usize,
    pub cols: usize,
    storage: Vec<u32>,
}

impl ToroidalBinaryMatrix for ToroidalBitMatrix {
    fn get_rows(&self) -> usize {
        self.rows
    }
    fn get_cols(&self) -> usize {
        self.cols
    }
    fn new(table: Vec<Vec<bool>>) -> Result<Self, MatrixConstructError> {
        let rows = table.len();
        let cols = if rows == 0 { 0 } else { table[0].len() };
        if cols == 0 {
            return Err(MatrixConstructError::EmptyTable());
        }

        // if the table is ragged (every column is not the same size) then we reject the input and return an Err result
        if table
            .iter()
            .map(|row| row.len() != cols)
            .fold(false, |a, b| a | b)
        {
            return Err(MatrixConstructError::RaggedTable());
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
                next_element += if b { 2_u32.pow(i as u32) } else { 0 };
            }
            storage.push(next_element);
        }

        Ok(Self {
            rows,
            cols,
            storage,
        })
    }
    fn at(&self, idx: MatrixIndex) -> bool {
        let row = idx.0.rem_euclid(self.rows as isize);
        let col = idx.1.rem_euclid(self.cols as isize);
        let bit_index = row as usize * self.cols + col as usize;

        let vec_idx: usize = bit_index / u32::BITS as usize;
        let element_offset: usize = bit_index % u32::BITS as usize;

        (self.storage[vec_idx] >> element_offset) & 1 != 0
    }
    fn set(&mut self, idx: &MatrixIndex, value: bool) -> bool {
        let row = idx.0.rem_euclid(self.rows as isize);
        let col = idx.1.rem_euclid(self.cols as isize);
        let bit_index = row as usize * self.cols + col as usize;

        let vec_idx: usize = bit_index / u32::BITS as usize;
        let element_offset: usize = bit_index % u32::BITS as usize;
        
        let original_value = self.storage[vec_idx] << (element_offset & 1 ) > 0;
        if value {
            self.storage[vec_idx] |= 1 << element_offset;
        } else {
            self.storage[vec_idx] &= !(1 << element_offset);
        }

        original_value
    }
    fn bitwise_xor(&mut self, other: &ToroidalBitMatrix) -> Result<(), MatrixOpError> {
        if self.rows != other.rows || self.cols != other.cols {
            return Err(MatrixOpError::DifferentShapes());
        }
        for (i, element) in (&mut self.storage).into_iter().enumerate() {
            *element ^= other.storage[i as usize];
        }
        Ok(())
    }
    fn popcount(&self) -> u32 {
        self.storage.iter().map(|e| e.count_ones()).sum()
    }
}

impl ToroidalBitMatrix {
    /// Returns the storage backing the matrix.
    pub fn get_storage(&self) -> &Vec<u32> {
        &self.storage
    }
    /// Constructs a new [`ToroidalBitMatrix`] from storage, as well as the count of rows and
    /// columns. Returns an error if the storage is the wrong size for the specified matrix shape.
    pub fn from_storage(
        rows: usize,
        cols: usize,
        storage: Vec<u32>,
    ) -> Result<Self, MatrixConstructError> {
        if rows == 0 || cols == 0 {
            return Err(MatrixConstructError::EmptyTable());
        }
        let n_elements = rows * cols;
        if storage.len()
            != ((n_elements / u32::BITS as usize) + (n_elements % u32::BITS as usize > 0) as usize)
        {
            return Err(MatrixConstructError::InvalidStorage());
        }
        Ok(Self {
            rows,
            cols,
            storage,
        })
    }
}
