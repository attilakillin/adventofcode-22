use std::io::{self, BufRead};

pub struct Input {
    lines: Vec<String>,
}

impl Input {
    pub fn new() -> Self {
        return Input { lines: Vec::new() };
    }

    pub fn read_lines(&mut self) -> &Self {
        let mut lines: Vec<String> = Vec::new();

        let mut stream = io::stdin().lock().lines();
        while let Some(line) = stream.next() {
            lines.push(line.unwrap().to_string());
        }

        self.lines = lines;
        return self;
    }

    pub fn lines_as_numbers(&self) -> Vec<Option<isize>> {
        return self.lines.iter().map(|s| s.parse::<isize>().ok()).collect();
    }
}
