use aoc_common::input::Input;
use cave::{Cave, Coordinate};

mod cave;

fn main() {
    let paths = Input::new().read_lines().lines();
    let mut cave = Cave::new(&paths, Coordinate::new(500, 0), true); // Change to false for part 1.

    let mut answer = 0;
    while cave.run_one() {
        answer += 1;
    }

    println!("Answer: {}", answer);
}
