use std::collections::BinaryHeap;

use aoc_common::{input::Input, vectors::group_option_vector};

fn main() {
    let food_carried = group_option_vector(Input::new().read_lines().lines_as_numbers());
    let mut totals = food_carried.iter().map(|v| v.iter().sum::<isize>()).collect::<BinaryHeap<isize>>();

    let mut top_three = Vec::new();
    for _ in 0..3 {
        if let Some(value) = totals.pop() {
            top_three.push(value);
        }
    }

    println!("Answer for #1: {}", top_three[0]);
    println!("Answer for #2: {}", top_three.iter().sum::<isize>());
}
