use crate::matrix::{MatrixIndex, ToroidalBitMatrix};
use std::mem;

/// The character used to represent an [`Automaton`]'s `true` state in files and String
/// representations.
const TRUE_CHAR: char = '#';
/// The character used to represent an [`Automaton`]'s `false` state in files and String
/// representations.
const FALSE_CHAR: char = '.';

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
/// Object defining a 2D, binary cellular automaton
/// This CA implementation assumes that the geometry of the cell-space is spherical.
pub struct Automaton {
    rule: AutomatonRule,
    state: ToroidalBitMatrix,
}

impl Automaton {
    pub fn new(state: ToroidalBitMatrix, rule: AutomatonRule) -> Self {
        Automaton { state, rule }
    }
    pub fn iter_rule(&mut self, iterations: u32) {
        let (rows, cols) = (self.state.rows, self.state.cols);

        let mut copy = self.state.clone();
        for _ in 0..iterations {
            for row in 0..rows {
                for col in 0..cols {
                    let idx = (row as isize, col as isize);
                    let n_alive_neighbors = self.alive_neighbors(idx);

                    if self.state.get(idx) {
                        copy.set(idx, !self.rule.dies[n_alive_neighbors as usize]);
                    } else {
                        copy.set(idx, self.rule.born[n_alive_neighbors as usize]);
                    }
                }
            }

            mem::swap(&mut copy, &mut self.state);
        }
    }

    pub fn popcount(&self) -> u32 {
        self.state.popcount()
    }

    pub fn get_storage(&self) -> Vec<u32> {
        self.state.get_storage()
    }

    pub fn alive_neighbors(&self, idx: MatrixIndex) -> u32 {
        let (row, col) = (idx.0, idx.1);
        let mut sum_neighbors = 0;

        for r in (row - 1)..=(row + 1) {
            for c in (col - 1)..=(col + 1) {
                sum_neighbors += self.state.get((r, c)) as u32
            }
        }

        sum_neighbors -= self.state.get((row, col)) as u32;

        return sum_neighbors;
    }
}

/// Represents the state of the [`Automaton`] as a rectangular array of characters.
impl ToString for Automaton {
    fn to_string(&self) -> String {
        let (rows, cols) = (self.state.rows, self.state.cols);
        let mut result: String = String::with_capacity((self.state.rows + 1) * self.state.cols);

        for row in 0..rows {
            let row_str = (0..cols)
                .map(|c| match self.state.get((row as isize, c as isize)) {
                    true => TRUE_CHAR,
                    false => FALSE_CHAR,
                })
                .collect::<String>();
            result.push_str(&row_str);
            result.push_str("\n");
        }

        result
    }
}
