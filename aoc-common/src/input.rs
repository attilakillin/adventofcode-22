use std::io::{self, BufRead};

/// Handles input reading and collecting.
pub struct Input {
    lines: Vec<String>,
}

impl Input {
    /// Basic constructor to construct a new instance.
    pub fn new() -> Self {
        return Input { lines: Vec::new() };
    }

    /// Reads the entire standard input stream line-by-line.
    pub fn read_lines(&mut self) -> &Self {
        let mut lines: Vec<String> = Vec::new();

        let mut stream = io::stdin().lock().lines();
        while let Some(line) = stream.next() {
            lines.push(line.unwrap().to_string());
        }

        self.lines = lines;
        return self;
    }

    /// Returns the read lines with each line converted into a number where possible.
    pub fn lines_as_numbers(&self) -> Vec<Option<isize>> {
        return self.lines.iter().map(|s| s.parse::<isize>().ok()).collect();
    }
}
