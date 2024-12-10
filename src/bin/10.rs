use std::collections::HashSet;

use array2d::Array2D;
use colored::{self, Colorize};

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<usize> {
    let map = parse_map(input)?;
    let mut count = 0;
    for y in 0..map.row_len() {
        for x in 0..map.column_len() {
            if map[(y, x)] == 0 {
                // println!("\t{}", "start".blue());
                let (trailends, rating) = run_trail(&map, y, x, None);
                // println!("\tfound {} trailheads", trailends.len());
                count += trailends.len();
            }
        }
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = parse_map(input)?;
    let mut count = 0;
    for y in 0..map.row_len() {
        for x in 0..map.column_len() {
            if map[(y, x)] == 0 {
                // println!("\t{}", "start".blue());
                let (trailends, rating) = run_trail(&map, y, x, None);
                // println!("\tfound {} trailheads", trailends.len());
                count += rating;
            }
        }
    }
    Some(count)
}

fn parse_map(input: &str) -> Option<Array2D<u32>> {
    let mut rows = vec![];
    for line in input.lines() {
        let mut row = vec![];
        for point in line.chars() {
            row.push(point.to_digit(10)?);
        }
        rows.push(row);
    }
    Some(match Array2D::from_rows(&rows) {
        Ok(v) => v,
        Err(_) => return None,
    })
}

#[derive(Clone, Debug)]
enum Direction {
    East,
    South,
    West,
    North,
}

impl Direction {
    fn rotate_right(&self) -> Direction {
        match self {
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
            Direction::North => Direction::East,
        }
    }

    fn step(&self, row: usize, column: usize) -> Option<(usize, usize)> {
        let (row, column) = match self {
            Direction::East => (Some(row), column.checked_add(1)),
            Direction::South => (row.checked_add(1), Some(column)),
            Direction::West => (Some(row), column.checked_sub(1)),
            Direction::North => (row.checked_sub(1), Some(column)),
        };
        Some((row?, column?))
    }

    fn all_directions_except(direction: Option<Direction>) -> Vec<Direction> {
        match direction {
            Some(mut d) => {
                let mut v = vec![];
                for _ in 0..3 {
                    d = d.rotate_right();
                    v.push(d.clone());
                }
                v
            }
            None => vec![
                Direction::East,
                Direction::South,
                Direction::West,
                Direction::North,
            ],
        }
    }
}

fn run_trail(
    map: &Array2D<u32>,
    y: usize,
    x: usize,
    direction: Option<Direction>,
) -> (HashSet<(usize, usize)>, u32) {
    let mut trailheads = HashSet::new();
    let mut count = 0;

    // print!("({x}, {y}) from {:?}", direction);
    let point = match map.get(y, x) {
        Some(v) => {
            if *v == 9 {
                // println!("\t{}", 9.to_string().green());
                trailheads.insert((y, x));
                return (trailheads, 1);
            }
            v
        }
        None => return (HashSet::new(), 0),
    };
    // println!(" : {}", point);

    for d in Direction::all_directions_except(direction) {
        let next_position = match d.step(y, x) {
            Some(v) => v,
            None => continue,
        };
        let next_point = match map.get(next_position.0, next_position.1) {
            Some(v) => v,
            None => continue,
        };
        if *next_point == *point + 1 {
            let (further_trailheads, further_count) = run_trail(
                map,
                next_position.0,
                next_position.1,
                Some(d.rotate_right().rotate_right()),
            );
            for th in further_trailheads {
                trailheads.insert(th);
            }
            count += further_count;
        }
    }
    (trailheads, count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
