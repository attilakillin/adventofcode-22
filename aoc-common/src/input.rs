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

    /// Returns the read lines with each line converted into a list of whitespace-separated words.
    pub fn lines_as_words(&self) -> Vec<Vec<String>> {
        return self.lines.iter().map(|s| s.split_whitespace().map(str::to_string).collect()).collect();
    }

    /// Returns the read lines with each line converted into a list of chars.
    pub fn lines_as_chars(&self) -> Vec<Vec<char>> {
        return self.lines.iter().map(|s| s.chars().collect()).collect();
    }

    // Returns the read lines with each line split by the given character sequence.
    pub fn lines_split_by(&self, split: &str) -> Vec<Vec<String>> {
        return self.lines.iter().map(|s| s.split(split).map(str::to_string).collect()).collect();
    }

    // Returns the lines as is.
    pub fn lines(&self) -> Vec<String> {
        return self.lines.clone();
    }
}
