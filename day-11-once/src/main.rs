use aoc_common::input::Input;
use monkey::Monkey;

mod monkey;

fn main() {
    // Input parsing
    let lines = Input::new().read_lines().lines();
    let input: Vec<&[String]> = lines.split(|s| s.is_empty()).collect();

    // Initialization
    let mut monkeys: Vec<Monkey> = vec![];
    for part in input {
        monkeys.push(Monkey::new(&part, 2 * 3 * 5 * 7 * 11 * 13 * 17 * 19));
    }

    // Simulation - change the round count and the run round parameter for V1 or V2.
    // (and also the division_constant value hardcoded above)
    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            let thrown_items = monkeys[i].run_round(false);

            for (target, item) in thrown_items {
                monkeys[target].accept_item(item);
            }
        }
    }

    let mut answer: Vec<usize> = monkeys.iter().map(|m| m.inspections).collect();
    answer.sort();

    println!("Answer for #1: {}", answer[answer.len() - 1] * answer[answer.len() - 2]);
}
