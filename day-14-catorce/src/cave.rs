use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize
}

impl Coordinate {
    pub fn new(x: usize, y: usize) -> Self {
        return Coordinate { x, y };
    }

    pub fn from(value: &str) -> Self {
        let parts: Vec<&str> = value.split(',').collect();
        return Coordinate { x: parts[0].parse().unwrap(), y: parts[1].parse().unwrap() };
    }
}

#[derive(Debug)]
pub struct Cave {
    sand_source: Coordinate,

    inner_state: Vec<Vec<char>>
}

impl Cave {
    pub fn new(paths: &[String], sand_source: Coordinate, has_floor: bool) -> Self {
        let mut coords: Vec<Vec<Coordinate>> = paths.iter()
            .map(|path| path.split(" -> ").map(|c| Coordinate::from(c)).collect())
            .collect();
        
        coords.push(vec![sand_source]);

        let mut offset_x = coords.iter().map(|p| p.iter().min_by(|c1, c2| c1.x.cmp(&c2.x)).unwrap().x).min().unwrap();
        let offset_y = coords.iter().map(|p| p.iter().min_by(|c1, c2| c1.y.cmp(&c2.y)).unwrap().y).min().unwrap();

        let mut max_x = coords.iter().map(|p| p.iter().max_by(|c1, c2| c1.x.cmp(&c2.x)).unwrap().x).max().unwrap() - offset_x;
        let mut max_y = coords.iter().map(|p| p.iter().max_by(|c1, c2| c1.y.cmp(&c2.y)).unwrap().y).max().unwrap() - offset_y;

        coords.pop();

        if has_floor {
            max_y += 2;

            offset_x -= max_y;
            max_x += 2 * max_y;

            coords.push(vec![Coordinate::new(offset_x, max_y), Coordinate::new(offset_x + max_x, max_y)]);
        }

        let mut inner_state = vec![vec!['.'; max_x + 1]; max_y + 1];

        for row in coords {
            for i in 1..row.len() {
                let (start, end) = (&row[i - 1], &row[i]);

                for y in usize::min(start.y, end.y) ..= usize::max(start.y, end.y) {
                    for x in usize::min(start.x, end.x) ..= usize::max(start.x, end.x) {
                        inner_state[y - offset_y][x - offset_x] = '#';
                    }
                }
            }
        }

        return Self {
            sand_source: Coordinate::new(sand_source.x - offset_x, sand_source.y - offset_y),
            inner_state
        };
    }

    pub fn run_one(&mut self) -> bool {
        let mut sand = self.sand_source.clone();

        if self.inner_state[sand.y][sand.x] != '.' {
            return false;
        }

        while (sand.y + 1) < self.inner_state.len() {
            if self.inner_state[sand.y + 1][sand.x] == '.' {
                sand.y += 1;
            } else if sand.x >= 1 && self.inner_state[sand.y + 1][sand.x - 1] == '.' {
                sand.x -= 1;
                sand.y += 1;
            } else if sand.x + 1 < self.inner_state[0].len() && self.inner_state[sand.y + 1][sand.x + 1] == '.' {
                sand.x += 1;
                sand.y += 1;
            } else if sand.x == 0 || sand.x == self.inner_state[0].len() {
                return false;
            } else {
                self.inner_state[sand.y][sand.x] = 'o';
                return true;
            }
        }

        return false;
    }
}

impl Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.inner_state {
            match write!(f, "{}\n", row.iter().collect::<String>()) {
                Ok(_) => (),
                err => return err
            };
        };
        Ok(())
    }
}
