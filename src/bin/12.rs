use array2d::{Array2D, Error};

advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<u32> {
    let mut plots = match parse_farm(input) {
        Ok(v) => v,
        Err(e) => {
            println!("couldn't parse farm: {e}");
            return None;
        }
    };

    let mut count = 0;
    for y in 0..plots.row_len() {
        for x in 0..plots.column_len() {
            let plot = plots.get(y, x)?;
            if plot.processed {
                continue;
            }
            let plant = plot.plant;
            let (area, perimiter) = find_friends(&mut plots, plant, x, y);
            count += area * perimiter;
        }
    }
    Some(count)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[derive(Clone)]
struct Plot {
    plant: char,
    processed: bool,
}

#[derive(Clone, Debug)]
enum Direction {
    East,
    South,
    West,
    North,
}

impl Direction {
    fn step(&self, row: usize, column: usize) -> Option<(usize, usize)> {
        let (row, column) = match self {
            Direction::East => (Some(row), column.checked_add(1)),
            Direction::South => (row.checked_add(1), Some(column)),
            Direction::West => (Some(row), column.checked_sub(1)),
            Direction::North => (row.checked_sub(1), Some(column)),
        };
        Some((row?, column?))
    }

    fn all() -> Vec<Direction> {
        vec![
            Direction::East,
            Direction::South,
            Direction::West,
            Direction::North,
        ]
    }
}

fn parse_farm(input: &str) -> Result<Array2D<Plot>, Error> {
    let mut rows = vec![];
    for line in input.lines() {
        let mut row = vec![];
        for c in line.chars() {
            row.push(Plot {
                plant: c,
                processed: false,
            });
        }
        rows.push(row);
    }
    Array2D::from_rows(&rows)
}

fn find_friends(plots: &mut Array2D<Plot>, friend: char, x: usize, y: usize) -> (u32, u32) {
    let plot = match plots.get_mut(y, x) {
        Some(v) => v,
        None => return (0, 1),
    };
    if plot.plant != friend {
        return (0, 1);
    }
    // stop if we've already taken this plot into account
    if plot.processed {
        return (0, 0);
    }

    plot.processed = true;
    let mut area = 1;
    let mut perimiter = 0;
    for d in Direction::all() {
        let (next_y, next_x) = match d.step(y, x) {
            Some(v) => v,
            None => {
                perimiter += 1;
                continue;
            }
        };
        let (a, p) = find_friends(plots, friend, next_x, next_y);
        area += a;
        perimiter += p;
    }
    (area, perimiter)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
