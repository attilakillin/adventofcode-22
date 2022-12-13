use aoc_common::input::Input;
use compare::Packet;

mod compare;

fn main() {
    let lines = Input::new().read_lines().lines();
    let mut pairs: Vec<(Packet, Packet)> = vec![];

    let mut iter = lines.iter();
    while let (Some(left), Some(right)) = (iter.next(), iter.next()) {
        pairs.push((Packet::new(left), Packet::new(right)));
        iter.next();
    }

    let mut answer_one = 0;
    for (i, pair) in pairs.iter().enumerate() {
        if pair.0.cmp(&pair.1).is_le() {
            answer_one += i + 1;
        }
    }

    let mut all_packets = pairs.iter()
        .fold(vec![], |mut array, e| { array.push(e.0.clone()); array.push(e.1.clone()); array });

    let div_1 = Packet::new("[[2]]");
    let div_2 = Packet::new("[[6]]");
    all_packets.extend([div_1.clone(), div_2.clone()]);

    all_packets.sort();

    let mut answer_two = 1;
    for (i, elem) in all_packets.iter().enumerate() {
        if elem == &div_1 || elem == &div_2 {
            answer_two *= i + 1;
        }
    }

    println!("Answer for #1: {}", answer_one);
    println!("Answer for #1: {}", answer_two);
}
