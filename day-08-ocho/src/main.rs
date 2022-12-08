use aoc_common::input::Input;

fn look_through(matrix: &Vec<Vec<isize>>, visibles: &mut Vec<Vec<bool>>, x_start: isize, y_start: isize, iterator: fn(isize, isize) -> (isize, isize)) {
    let (mut x, mut y) = (x_start, y_start);

    let mut best_height: isize = -1;
    while x >= 0 && y >= 0 && x < matrix[0].len() as isize && y < matrix.len() as isize {
        let (_x, _y) = (x as usize, y as usize);

        if matrix[_y][_x] > best_height {
            visibles[_y][_x] = true;
            best_height = matrix[_y][_x];
        }

        (x, y) = iterator(x, y);
    }
}

fn count_visible(matrix: &Vec<Vec<isize>>, x_start: isize, y_start: isize, iterator: fn(isize, isize) -> (isize, isize)) -> isize {
    let mut count = 0;
    let (mut x, mut y) = (x_start, y_start);

    let target: isize = matrix[y as usize][x as usize];
    (x, y) = iterator(x, y);
    while x >= 0 && y >= 0 && x < matrix[0].len() as isize && y < matrix.len() as isize {
        count += 1;

        if matrix[y as usize][x as usize] >= target {
            break;
        }

        (x, y) = iterator(x, y);
    }

    return count;
}

fn scenic_score_at(matrix: &Vec<Vec<isize>>, x: isize, y: isize) -> isize {
    return count_visible(matrix, x, y, |x, y| (x, y + 1))
        * count_visible(matrix, x, y, |x, y| (x, y - 1))
        * count_visible(matrix, x, y, |x, y| (x + 1, y))
        * count_visible(matrix, x, y, |x, y| (x - 1, y));
}

fn main() {
    let matrix = Input::new().read_lines().lines_as_digit_matrix();
    let (x_size, y_size) = (matrix[0].len() as isize, matrix.len() as isize);

    let mut visibles = vec![vec![false; x_size as usize]; y_size as usize];

    for x in 0..x_size {
        look_through(&matrix, &mut visibles, x, 0, |x, y| (x, y + 1));
        look_through(&matrix, &mut visibles, x, y_size - 1, |x, y| (x, y - 1));
    }

    for y in 0..y_size {
        look_through(&matrix, &mut visibles, 0, y, |x, y| (x + 1, y));
        look_through(&matrix, &mut visibles, x_size - 1, y, |x, y| (x - 1, y));
    }

    let mut result_one = 0;
    for row in visibles {
        for elem in row {
            if elem {
                result_one += 1;
            }
        }
    }

    let mut result_two = 0;
    for y in 0..y_size {
        for x in 0..x_size {
            let current = scenic_score_at(&matrix, x, y);
            if current > result_two {
                result_two = current;
            }
        }
    }

    println!("Answer for #1: {}", result_one);
    println!("Answer for #2: {}", result_two);
}
