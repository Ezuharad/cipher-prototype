// 2025 Steven Chiacchira
/// Type used to specify elements of a [`ToroidalBinaryMatrix`].
pub type MatrixIndex = (isize, isize);

/// Error occurring during Matrix initialization
#[derive(Debug)]
pub enum MatrixConstructError {
    /// Every row of the table used to define a Matrix's initial state must have the same number of columns
    RaggedTable(),
    /// A Matrix cannot have no cells
    EmptyTable(),
    /// A Matrix should have precisely enough elements to store its entries.
    InvalidStorage(),
}

/// Error arising from applying a matrix operation
#[derive(Debug)]
pub enum MatrixOpError {
    /// Some operations require matrices to have the same shape.
    DifferentShapes(),
    /// Some operations require matrices to have compatible shapes.
    IncompatibleShapes(),
}

/// Trait specifying methods for matrices with binary entries on a torus.
pub trait ToroidalBinaryMatrix: Sized {
    /// Creates a new instance of a matrix with entries from a table of `bool` values.
    fn new(table: Vec<Vec<bool>>) -> Result<Self, MatrixConstructError>;
    /// Returns the number of rows the matrix has.
    fn get_rows(&self) -> usize;
    /// Returns the number of columns the matrix has.
    fn get_cols(&self) -> usize;
    /// Returns the value of the matrix element at `idx`. If the row or column coordinate in `idx` is
    /// negative or greater than the number of rows or columns of the matrix respectively, the
    /// modulo of the coordinate will be used. This property is what makes the matrix 'toroidal'.
    fn at(&self, idx: MatrixIndex) -> bool;
    /// Sets the value of the matrix element at `idx` to `value`. If the row or column coordinate
    /// in `idx` is negative or greater than the number of rows or columns of the matrix
    /// respectively, the modulo of the coordinate will be used. This property is what makes the
    /// matrix 'toroidal'.
    fn set(&mut self, idx: MatrixIndex, value: bool);
    /// Performs bitwise xor of this matrix with `other`, returning a [`MatrixOpError`] if the two
    /// matrices have different shapes.
    fn bitwise_xor(&mut self, other: &Self) -> Result<(), MatrixOpError>;
    /// Swaps the two rows indexed by `row1` and `row2` of this Matrix.
    fn swap_rows(&mut self, row1: isize, row2: isize) {
        for col in 0..self.get_cols() {
            let temp = self.at((row1, col as isize));
            self.set((row2, col as isize), self.at((row1, col as isize)));
            self.set((row1, col as isize), temp);
        }
    }
    /// Swaps the two columns indexed by `col1` and `col2` of this Matrix.
    fn swap_cols(&mut self, col1: isize, col2: isize) {
        for row in 0..self.get_rows() {
            let temp = self.at((row as isize, col1));
            self.set((row as isize, col1), self.at((row as isize, col2)));
            self.set((row as isize, col2), temp);
        }
    }
    /// Returns the number of 'alive' (1) elements in the Matrix.
    fn popcount(&self) -> u32;
}
