use aoc_common::input::Input;
use rope::Rope;

mod rope;

fn main() {
    let instructions = Input::new().read_lines().lines_as_words();
    let mut rope = Rope::new(10);

    for instr in instructions {
        rope.move_head(&instr[0], instr[1].parse::<usize>().unwrap());
    }

    println!("Answer for #1: {}", rope.tail_history.len());
}
