use std::collections::HashSet;

use aoc_common::input::Input;
use sensors::Sensor;

use crate::sensors::{Intervals, Coordinate};

mod sensors;

fn find_coverage_at_y(sensors: &[Sensor], y: isize) -> Intervals {
    let mut cover_at_y = Intervals::new_empty();
    for sensor in sensors {
        match sensor.covered_at_y(y) {
            Some(interval) => {
                cover_at_y = cover_at_y.merge(&interval);
            },
            None => ()
        }
    }

    return cover_at_y;
}

fn main() {
    let mut sensors: Vec<Sensor> = vec![];
    for line in Input::new().read_lines().lines() {
        sensors.push(Sensor::parse(&line));
    }
    let beacons: HashSet<Coordinate> = sensors.iter().map(|s| s.beacon.clone()).collect();

    let y = 2000000; // Change this to change the y to check.
    let y_max = 4000000; // And this for part 2.

    let cover_at_y = find_coverage_at_y(&sensors, y);
    let answer_one = cover_at_y.size() - beacons.iter()
        .filter(|b| b.y == y && cover_at_y.contains(b.x))
        .count() as isize;

    let mut answer_two = 0;
    for y in 0..=y_max {
        if y % (y_max / 100 + 1) == 0 {
            println!("Checking rows... {}%", y * 100 / y_max);
        }

        let current_cover = find_coverage_at_y(&sensors, y);
        if current_cover.intervals.len() > 1 {
            let x = current_cover.intervals.iter().map(|i| i.end).min().unwrap() + 1;
            println!("Found coverage with more than one intervals - ({},{}) is not covered", x, y);
            answer_two = x * 4000000 + y;
            break;
        }
    }

    println!("Answer for #1: {}", answer_one);
    println!("Answer for #2: {}", answer_two);
}
