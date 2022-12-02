use aoc_common::input::Input;

fn main() {
    let points_one = [
        [4, 8, 3], // Row for A, cols: X, Y, Z
        [1, 5, 9], // Row for B
        [7, 2, 6]  // Row for C
    ];
    let points_two = [
        [3, 4, 8], // Row for A, cols: X, Y, Z
        [1, 5, 9], // Row for B
        [2, 6, 7]  // Row for C
    ];

    let strategy = Input::new().read_lines().lines_as_words();

    let mut total_one = 0;
    let mut total_two = 0;
    for entry in strategy {
        let row = (entry[0].as_bytes()[0] - 'A' as u8) as usize;
        let col = (entry[1].as_bytes()[0] - 'X' as u8) as usize;

        total_one += points_one[row][col];
        total_two += points_two[row][col];
    }

    println!("Answer for #1: {}", total_one);
    println!("Answer for #2: {}", total_two);
}
