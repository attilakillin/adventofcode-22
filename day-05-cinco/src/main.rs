use aoc_common::input::Input;
use model::System;

mod model;

fn main() {
    let input = Input::new().read_lines().lines();

    let parts: Vec<Vec<String>> = input.split(|l| l.len() == 0).map(|l| l.to_owned()).collect();

    let mut system_v1 = System::initialize(&parts[0]);
    let mut system_v2 = System::initialize(&parts[0]);

    for line in &parts[1] {
        let words: Vec<&str> = line.split_ascii_whitespace().collect();
        let count = words[1].parse::<usize>().unwrap();
        let source = words[3].parse::<usize>().unwrap();
        let target = words[5].parse::<usize>().unwrap();

        system_v1.move_crates_v1(count, source, target);
        system_v2.move_crates_v2(count, source, target);
    }

    println!("Answer to #1: {}", system_v1.get_message());
    println!("Answer to #2: {}", system_v2.get_message());
}
