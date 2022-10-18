use std::fmt::{self, Display, Formatter};

use crate::board:: COLUMNS;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Position {
    index: i8,
}

impl Position {
    pub fn from_index(index: usize) -> Self {
        Self {
            index: index.try_into().unwrap()
        }
    }

    pub fn new(row: i8, column: i8) -> Self {
        Self {
            index: row * COLUMNS as i8 + column,
        }
    }

    pub fn as_index(self) -> usize {
        self.index as usize
    }

    pub fn row(self) -> i8 {
        self.index / COLUMNS as i8
    }

    pub fn column(self) -> i8 {
        self.index % COLUMNS as i8
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let letter = (self.row() as u8 + b'A') as char;
        let digit = (self.column() as u8) + 1;
        write!(f, "{letter}{digit}")
    }
}

#[cfg(test)]
mod tests {
    use super::Position;

    #[test]
    fn print_position() {
        assert_eq!("A1", Position::new(0, 0).to_string());
    }
}
