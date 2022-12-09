use std::collections::HashMap;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Position {
    x: isize,
    y: isize
}

impl Position {
    pub fn new() -> Self {
        return Position { x: 0, y: 0 };
    }

    pub fn from(x: isize, y: isize) -> Self {
        return Position { x, y };
    }
}

pub struct Rope {
    knots: Vec<Position>,
    pub tail_history: HashMap<Position, usize>
}

impl Rope {
    pub fn new(knot_count: usize) -> Self {
        return Rope {
            knots: vec![Position::new(); knot_count],
            tail_history: HashMap::from([(Position::new(), 1)])
        }
    }

    pub fn move_head(&mut self, dir: &str, count: usize) {
        for _ in 0..count {
            self.step_head(dir);
            for id in 1..self.knots.len() {
                self.step_tail(id);
            }
        }
    }

    fn step_head(&mut self, dir: &str) {
        let prev = &self.knots[0];
        self.knots[0] = match dir {
            "U" => Position::from(prev.x, prev.y + 1),
            "D" => Position::from(prev.x, prev.y - 1),
            "L" => Position::from(prev.x - 1, prev.y),
            "R" => Position::from(prev.x + 1, prev.y),
            _ => panic!("Invalid direction string!")
        };
    }

    fn step_tail(&mut self, id: usize) {
        let count = self.knots.len();

        let head = self.knots[id - 1];
        let tail = &mut self.knots[id];

        let x_delta = head.x - tail.x;
        let y_delta = head.y - tail.y;

        if isize::abs(x_delta) <= 1 && isize::abs(y_delta) <= 1 {
            return;
        }

        tail.x += isize::signum(x_delta);
        tail.y += isize::signum(y_delta);

        if id == count - 1 {
            *self.tail_history.entry(*tail).or_insert(0) += 1;
        }
    }
}
