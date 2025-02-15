use crate::{Table, TableOption};

use super::Line;

/// A horizontal split line which can be used to set a border.
#[derive(Debug, Clone)]
pub struct VerticalLine {
    pub(crate) index: usize,
    pub(crate) line: Option<Line>,
}

impl VerticalLine {
    /// Creates a new horizontal split line.
    pub const fn new(index: usize, line: Line) -> Self {
        Self {
            index,
            line: Some(line),
        }
    }

    /// Removes an existing split line by index.
    ///
    /// It not present or in case of index bigger than the count of columns it has no affect.
    pub fn empty(index: usize) -> Self {
        Self { index, line: None }
    }

    /// Sets a horizontal character.
    pub const fn main(mut self, c: Option<char>) -> Self {
        let mut line = match self.line {
            Some(line) => line,
            None => Line::empty(),
        };

        line.main = c;
        self.line = Some(line);

        self
    }

    /// Sets a vertical intersection character.
    pub const fn intersection(mut self, c: Option<char>) -> Self {
        let mut line = match self.line {
            Some(line) => line,
            None => Line::empty(),
        };

        line.intersection = c;
        self.line = Some(line);

        self
    }

    /// Sets a top character.
    pub const fn top(mut self, c: Option<char>) -> Self {
        let mut line = match self.line {
            Some(line) => line,
            None => Line::empty(),
        };

        line.connector1 = c;
        self.line = Some(line);

        self
    }

    /// Sets a bottom character.
    pub const fn bottom(mut self, c: Option<char>) -> Self {
        let mut line = match self.line {
            Some(line) => line,
            None => Line::empty(),
        };

        line.connector2 = c;
        self.line = Some(line);

        self
    }

    /// Checks if it's an empty line.
    pub const fn is_empty(&self) -> bool {
        match &self.line {
            Some(l) => l.is_empty(),
            None => true,
        }
    }
}

impl<R> TableOption<R> for VerticalLine {
    fn change(&mut self, table: &mut Table<R>) {
        match &self.line {
            Some(line) => table
                .get_config_mut()
                .set_vertical_line(self.index, papergrid::VerticalLine::from(*line)),
            None => table.get_config_mut().remove_vertical_line(self.index),
        }
    }
}
