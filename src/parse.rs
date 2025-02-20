use std::collections::hash_map::HashMap;
use std::iter::zip;

/// Error occurring during the reading of a string defining a table of `bool` values.
#[derive(Debug)]
pub enum TableReadError {
    /// Error occurring from using an invalid character in the file read
    InvalidCharacter(char),
    /// Error occurring from a non-uniform table
    RaggedTable(),
}

const DEFAULT_KEYS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";

pub fn gen_char_map(seed: u32) -> HashMap<char, bool> {
    zip(
        DEFAULT_KEYS.chars(),
        (0..DEFAULT_KEYS.len()).map(|n| (seed >> n) & 1 != 0),
    )
    .collect::<HashMap<char, bool>>()
}

/// Reads a string as a bool table state with characters.
/// Ex:
/// ```txt
/// .....
/// ..#..
/// ...#.
/// .###.
/// ```
///
/// specifies the table
/// ```txt
/// FFFFF
/// FFTFF
/// FFFTF
/// FTTTF
/// ```
/// with char_map { '#': true, '.': false }.
/// Returns a [`Result`] containing either the new bool table on a success, or a
/// [`TableReadError`] on a failure.
pub fn parse_bool_table(
    string: &str,
    char_map: HashMap<char, bool>,
) -> Result<Vec<Vec<bool>>, TableReadError> {
    let mut table: Vec<Vec<bool>> = Vec::new();
    for line in string.lines() {
        let val_row: Vec<bool> = line
            .chars()
            .map(|c| match char_map.get(&c) {
                Some(v) => Ok(v.to_owned()),
                None => Err(TableReadError::InvalidCharacter(c)),
            })
            .collect::<Result<Vec<bool>, TableReadError>>()?;

        table.push(val_row);
    }

    Ok(table)
}
