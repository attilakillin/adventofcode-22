use aoc_common::input::Input;

// Map char array to priorities.
fn map_chars_to_priorities(chars: &[char]) -> Vec<usize> {
    return chars.iter()
        .map(|c| if c.is_lowercase() {
            (c.to_owned() as u8 - 'a' as u8 + 1) as usize
        } else {
            (c.to_owned() as u8 - 'A' as u8 + 27) as usize
        })
        .collect();
}

// Find the once common element in every rucksack.
fn solve_one(input: &Vec<Vec<char>>) -> usize {
    let mut commons: Vec<char> = vec![];
    for rucksack in input {
        let separator = rucksack.len() / 2;
        let comp_one = &rucksack[..separator];
        let comp_two = &rucksack[separator..];

        for element in comp_one {
            if comp_two.contains(element) {
                commons.push(element.to_owned());
                break;
            }
        }
    }

    return map_chars_to_priorities(&commons).iter().sum::<usize>();
}

// Find the one common char in each tree consecutive lines.
fn solve_two(input: &Vec<Vec<char>>) -> usize {
    let mut commons: Vec<char> = vec![];

    let mut iter = input.iter();
    while let (Some(a), Some(b), Some(c)) = (iter.next(), iter.next(), iter.next()) {
        commons.push(a.iter().filter(|e| b.contains(e)).find(|e| c.contains(e)).unwrap().to_owned());
    }

    return map_chars_to_priorities(&commons).iter().sum::<usize>();
}

fn main() {
    let input = Input::new().read_lines().lines_as_chars();

    println!("Answer for #1: {}", solve_one(&input));
    println!("Answer for #2: {}", solve_two(&input));
}
