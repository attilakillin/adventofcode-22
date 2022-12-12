use aoc_common::input::Input;
use graph::BFTreeBuilder;

mod graph;
mod node;

fn main() {
    // Input parsing
    let values = Input::new().read_lines().lines_as_chars();
    
    let mut graph = BFTreeBuilder::new(&values);
    graph.build();

    let answer_one = graph.root().find_by_value('S').unwrap().depth;
    let answer_two = usize::min(answer_one, graph.root().find_depth_of_closest('a').unwrap());

    println!("Answer for #1: {}", answer_one);
    println!("Answer for #2: {}", answer_two);
}
