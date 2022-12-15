use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coordinate {
    pub x: isize,
    pub y: isize
}

impl Coordinate {
    pub fn new(x: isize, y: isize) -> Self {
        return Self { x, y };
    }

    pub fn parse(string: &str) -> Self {
        let parts: Vec<String> = string.replace("x=", "").split(", y=").map(|s| s.to_string()).collect();

        return Self::new(parts[0].parse().unwrap(), parts[1].parse().unwrap());
    }

    pub fn distance_to(&self, other: &Self) -> isize {
        return isize::abs(self.x - other.x) + isize::abs(self.y - other.y);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Interval {
    pub start: isize,
    pub end: isize
}

impl Interval {
    pub fn new(start: isize, end: isize) -> Self {
        return Self { start, end };
    }

    pub fn size(&self) -> isize {
        return self.end - self.start + 1;
    }

    pub fn union(&self, other: &Self) -> Self {
        if !self.intersect(other) {
            panic!("Cannot create union, as the two intervals do not intersect each other!");
        }
        return Self::new(isize::min(self.start, other.start), isize::max(self.end, other.end));
    }

    pub fn intersect(&self, other: &Self) -> bool {
        return (self.start >= other.start && self.start <= other.end) ||
               (other.start >= self.start && other.start <= self.end);
    }

    pub fn contains(&self, value: isize) -> bool {
        return self.start <= value && value <= self.end;
    }
}

#[derive(Debug, Clone)]
pub struct Intervals {
    pub intervals: HashSet<Interval>
}

impl Intervals {
    pub fn new_empty() -> Self {
        return Self { intervals: HashSet::new() };
    }

    pub fn size(&self) -> isize {
        return self.intervals.iter().map(|i| i.size()).sum();
    }

    pub fn contains(&self, value: isize) -> bool {
        return self.intervals.iter().any(|i| i.contains(value));
    }

    pub fn merge(&self, other: &Interval) -> Self {
        let mut result = self.clone();
        result.intervals.insert(other.clone());
        result.minimalize();

        return result;
    }

    fn minimalize(&mut self) {
        let mut rerun = true;

        while rerun {
            rerun = false;

            let mut minimal_intervals: HashSet<Interval> = HashSet::new();
            for interval in &self.intervals {
                if let Some(i) = minimal_intervals.iter().find(|i| i.intersect(interval)).map(|i| i.clone()) {
                    minimal_intervals.remove(&i);
                    minimal_intervals.insert(i.union(interval));
                    rerun = true;
                } else {
                    minimal_intervals.insert(interval.clone());
                }
            }
            self.intervals = minimal_intervals;
        }
    }
}

pub struct Sensor {
    pub position: Coordinate,
    pub beacon: Coordinate
}

impl Sensor {
    pub fn new(position: Coordinate, beacon: Coordinate) -> Self {
        return Self { position, beacon };
    }

    pub fn parse(string: &str) -> Self {
        let parts: Vec<String> = string.replace("Sensor at ", "").split(": closest beacon is at ").map(|s| s.to_string()).collect();

        return Self::new(Coordinate::parse(&parts[0]), Coordinate::parse(&parts[1]));
    }

    pub fn radius(&self) -> isize {
        return self.position.distance_to(&self.beacon);
    }

    pub fn covered_at_y(&self, y: isize) -> Option<Interval> {
        let y_distance = isize::abs(self.position.y - y);
        let max_deltax = self.radius() - y_distance;

        return if max_deltax < 0 {
            None
        } else {
            Some(Interval::new(self.position.x - max_deltax, self.position.x + max_deltax))
        };
    }
}
