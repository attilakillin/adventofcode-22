use aoc_common::input::Input;

use crate::ai::{read_string_until_four_different_chars, read_string_until_fourteen_different_chars};

mod ai;

fn main() {
    let input = Input::new().read_lines().as_line();

    println!("Answer to #1: {}", read_string_until_four_different_chars(&input));
    println!("Answer to #2: {}", read_string_until_fourteen_different_chars(&input));
}
