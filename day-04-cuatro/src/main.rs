use aoc_common::input::Input;

#[derive(Debug)]
struct Interval {
    start: usize,
    end: usize
}

impl Interval {
    fn new(code: &str) -> Self {
        let parts: Vec<&str> = code.split("-").collect();

        return Interval { start: parts[0].parse::<usize>().unwrap(), end: parts[1].parse::<usize>().unwrap() };
    }
}

fn lines_to_interval_pairs(input: &Vec<Vec<String>>) -> Vec<(Interval, Interval)> {
    return input.iter().map(|l| (Interval::new(&l[0]), Interval::new(&l[1]))).collect();
}

fn main() {
    let intervals = lines_to_interval_pairs(&Input::new().read_lines().lines_split_by(","));

    let count_one = intervals.iter()
        .filter(|(a, b)| (a.start <= b.start && a.end >= b.end) || (b.start <= a.start && b.end >= a.end))
        .count();
    let count_two = intervals.iter()
        .filter(|(a, b)| (a.start <= b.end && a.end >= b.end) || (b.start <= a.end && b.end >= a.end))
        .count();

    println!("Answer for #1: {}", count_one);
    println!("Answer for #2: {}", count_two);
}
