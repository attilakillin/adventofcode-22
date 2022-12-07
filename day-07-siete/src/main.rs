use aoc_common::input::Input;
use reader::Reader;

use crate::types::{FilesystemElement, Folder};

mod reader;
mod types;

fn main() {
    let lines = Input::new().read_lines().lines();
    let root = Reader::new(lines.iter()).parse();

    let mut sum_one = 0;
    root.visit_folders(&mut (|f: &Folder| {
        if f.size() <= 100000 { sum_one += f.size(); };
    }));

    let mut result_two = 70000000;
    let needed_space = 30000000 - (70000000 - root.size());
    root.visit_folders(&mut (|f: &Folder| {
        let size = f.size();
        if size > needed_space && size < result_two { result_two = size };
    }));

    println!("Answer for #1: {}", sum_one);
    println!("Answer for #2: {}", result_two);
}
