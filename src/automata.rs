use std::ops::Index;
use std::string::ToString;
use crate::matrix;

/// The character used to represent an [`Automaton`]'s `true` state in files and String
/// representations.
const TRUE_CHAR: char = '#';
/// The character used to represent an [`Automaton`]'s `false` state in files and String
/// representations.
const FALSE_CHAR: char = '.';

/// Index for cells in an [`Automaton`]'s spherical cell-space. Negative index entries will wrap to
/// the corresponding non-negative index.
type AutomatonIndex = (i32, i32);

/// Error occurring during [`Automaton`] initialization
#[derive(Debug)]
pub enum AutomatonConstructError {
    /// Every row of the table used to define an Automaton's initial state must have the same number of columns
    RaggedTable(),
    /// An Automaton cannot have no cells
    EmptyTable(),
}

/// Simple struct defining how an [`Automaton`] will change from one state to the next.
#[derive(Debug)]
pub struct AutomatonRule {
    /// A 9-element array of booleans. If the ith element is `true`, then a dead cell with `i`
    /// alive neighbors will become alive.
    /// ex. the `born` array `[true, true, false, false, false, false, false, false, false]`
    /// specifies that only cells with 0 or 1 neighboring alive cells will become alive.
    pub born: [bool; 9],
    /// A 9-element array of booleans. If the ith element is `true`, then a living cell with `i`
    /// alive neighbors will die.
    /// ex. the `dies` array `[true, true, false, false, false, false, false, false, false]`
    /// specifies that only cells with 0 or 1 neighboring alive cells will die.
    pub dies: [bool; 9],
}

#[derive(Debug)]
/// Object defining a 2D, binary cellular automaton's current cell state.
/// This CA implementation assumes that the geometry of the cell-space is spherical.
pub struct Automaton {
    /// Internal storage specifying the Automaton's state. Cells states are stored as booleans,
    /// with a `true` value indicating a cell is alive and a `false` value indicating a cell is
    /// dead. These values are stored as a flat array in column-major order.
    storage: Vec<bool>,
    /// The number of rows of cells in the Automaton's cell-space.
    rows: usize,
    /// The number of columns of cells in the Automaton's cell-space.
    cols: usize,
}

impl Automaton {
    /// Creates a new Automaton from a rectangular table of bools.
    /// Returns a [`Result`] containing either an [`Automaton`] if `table` contains a positive number of
    /// elements and is rectangular, or an [`AutomatonConstructError`] otherwise.
    pub fn from_table(table: Vec<Vec<bool>>) -> Result<Self, AutomatonConstructError> {
        let rows = table.len();
        let cols = if rows == 0 { 0 } else { table[0].len() };

        if cols == 0 {
            return Err(AutomatonConstructError::EmptyTable());
        }

        // if the table is ragged (every column is not the same size) then we reject the input and return an Err result
        if table
            .iter()
            .map(|row| row.len() != cols)
            .fold(false, |a, b| a | b)
        {
            return Err(AutomatonConstructError::RaggedTable());
        }

        // ravel table for performance
        let mut storage = vec![false; rows * cols];
        for row in 0..rows {
            let l_offset = row * cols;
            let r_offset = l_offset + cols;
            storage[l_offset..r_offset].copy_from_slice(&table[row]);
        }

        Ok(Automaton {
            storage,
            rows,
            cols,
        })
    }

    /// Sets the value of the cell at `idx` to `value` in the [`Automaton`]'s cell-space.
    pub fn set_cell(&mut self, idx: AutomatonIndex, value: bool) -> bool {
        let row = idx.0.rem_euclid(self.rows as i32);
        let col = idx.1.rem_euclid(self.cols as i32);

        let flat_idx: usize = row as usize * self.cols + col as usize;
        let result: bool = self.storage[flat_idx];
        self.storage[flat_idx] = value;
        result
    }

    /// Gives the number of alive cells Moore-neighboring the cell at `idx`.
    pub fn alive_neighbors(&self, idx: AutomatonIndex) -> u32 {
        let mut result = 0;
        for row in (idx.0 - 1)..=(idx.0 + 1) {
            for col in (idx.1 - 1)..=(idx.1 + 1) {
                result += self[(row, col)] as u32;
            }
        }

        // we cannot count this cell as a neighbor to itself
        result -= self[idx] as u32;

        result as u32
    }

    /// Modifies `Automaton` by iterating with [`AutomatonRule`] `rule`.
    pub fn iter_rule(&mut self, rule: &AutomatonRule) {
        let mut copy = self.storage.clone();

        for row in 0..self.rows {
            for col in 0..self.cols {
                let alive_neighbors = self.alive_neighbors((row as i32, col as i32));

                let is_alive: bool = if self[(row as i32, col as i32)] {
                    // if cell is alive, kill it if it should die
                    !rule.dies[alive_neighbors as usize]
                } else {
                    // otherwise set cell to true if it should be born
                    rule.born[alive_neighbors as usize]
                };

                copy[row * self.cols + col] = is_alive;
            }
        }
        self.storage = copy;
    }
}

/// Returns the state of the cell at `idx`.
impl Index<AutomatonIndex> for Automaton {
    type Output = bool;
    fn index(&self, idx: AutomatonIndex) -> &bool {
        // we use the modulo because we are modeling a spherical automata
        let row = idx.0.rem_euclid(self.rows as i32);
        let col = idx.1.rem_euclid(self.cols as i32);
        let flat_idx: usize = row as usize * self.cols + col as usize;
        &self.storage[flat_idx]
    }
}

/// Represents the state of the [`Automaton`] as a rectangular array of characters.
impl ToString for Automaton {
    fn to_string(&self) -> String {
        let mut result: String = String::with_capacity((self.rows + 1) * self.cols);
        for row in 0..self.rows {
            let l_offset = row * self.cols;
            let r_offset = l_offset + self.cols;

            result.push_str(
                &self.storage[l_offset..r_offset]
                    .iter()
                    .map(|bit| match bit {
                        true => TRUE_CHAR,
                        false => FALSE_CHAR,
                    })
                    .collect::<String>(),
            );
            result.push_str("\n");
        }
        result
    }
}
