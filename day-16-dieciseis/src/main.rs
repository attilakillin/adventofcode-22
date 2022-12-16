use aoc_common::input::Input;
use graph::Graph;

mod graph;

fn main() {
    let lines = Input::new().read_lines().lines();
    let graph = Graph::new(&lines);

    println!("Answer for #1: {}", graph.solve_task(30));
}
