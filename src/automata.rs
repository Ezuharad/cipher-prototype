// 2025 Steven Chiacchira
use crate::matrix::{MatrixIndex, ToroidalBinaryMatrix, ToroidalBoolMatrix};
use std::mem;

/// The character used to represent an [`Automaton`]'s `true` state in files and String
/// representations.
const TRUE_CHAR: char = '#';
/// The character used to represent an [`Automaton`]'s `false` state in files and String
/// representations.
const FALSE_CHAR: char = '.';

#[derive(Clone, Debug)]
/// Simple struct defining how an [`Automaton`] will change from one state to the next.
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
    state: ToroidalBoolMatrix,
}

impl Automaton {
    /// Creates a new [`Automaton`] instance from a `state` represented as a [`ToroidalBoolMatrix`]
    /// and an [`AutomatonRule`] `rule`.
    pub fn new(state: ToroidalBoolMatrix, rule: &AutomatonRule) -> Self {
        Automaton {
            state,
            rule: rule.clone(),
        }
    }
    /// Iterates the [`Automaton`]'s rule `iterations` times.
    pub fn iter_rule(&mut self, iterations: u32) {
        let (rows, cols) = (self.state.rows, self.state.cols);

        let mut copy = self.state.clone();
        for _ in 0..iterations {
            for row in 0..rows {
                for col in 0..cols {
                    let idx = (row as isize, col as isize);
                    let n_alive_neighbors = self.alive_neighbors(idx);

                    if self.state.at(idx) {
                        copy.set(&idx, !self.rule.dies[n_alive_neighbors as usize]);
                    } else {
                        copy.set(&idx, self.rule.born[n_alive_neighbors as usize]);
                    }
                }
            }

            mem::swap(&mut copy, &mut self.state);
        }
    }

    /// Returns a reference to the Automaton state, represented as a [`ToroidalBoolMatrix`].
    pub fn get_state(&self) -> &ToroidalBoolMatrix {
        &self.state
    }

    /// Sets the state of the cell at `idx` to `value`, returning the original value at `idx`.
    pub fn set_state(&mut self, idx: &MatrixIndex, value: bool) -> bool {
        self.state.set(&idx, value)
    }

    /// Counts the number of alive [Moore
    /// neighbors](https://en.wikipedia.org/wiki/Moore_neighborhood) at `idx`.
    pub fn alive_neighbors(&self, idx: MatrixIndex) -> u32 {
        let (row, col) = (idx.0, idx.1);
        let mut sum_neighbors = 0;

        for r in (row - 1)..=(row + 1) {
            for c in (col - 1)..=(col + 1) {
                sum_neighbors += self.state.at((r, c)) as u32
            }
        }

        sum_neighbors -= self.state.at((row, col)) as u32;

        return sum_neighbors;
    }
}

/// Represents the state of the [`Automaton`] as a rectangular array of characters.
/// ex. 
/// an Automaton with the state
/// ```txt
/// TFFT
/// TFTT
/// TTTT
/// ```
/// Will be represented as 
/// ```txt
/// #..#
/// TFTT
/// TTTT
/// ```
impl ToString for Automaton {
    fn to_string(&self) -> String {
        let (rows, cols) = (self.state.rows, self.state.cols);
        let mut result: String = String::with_capacity((self.state.rows + 1) * self.state.cols);

        for row in 0..rows {
            let row_str = (0..cols)
                .map(|c| match self.state.at((row as isize, c as isize)) {
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
