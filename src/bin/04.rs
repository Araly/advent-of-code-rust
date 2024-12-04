use std::usize;

use array2d::Array2D;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let grid = make_grid(input)?;

    let mut count = 0;
    for row in 0..grid.row_len() {
        for column in 0..grid.column_len() {
            count += count_words(&grid, row, column);
        }
    }

    Some(count)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

const WORD: &str = "XMAS";

enum Direction {
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
    North,
}

impl Direction {
    fn step_towards(&self, row: usize, column: usize, step: usize) -> Option<(usize, usize)> {
        let (row, column) = match self {
            Direction::NorthEast => (row.checked_sub(step), column.checked_add(step)),
            Direction::East => (Some(row), column.checked_add(step)),
            Direction::SouthEast => (row.checked_add(step), column.checked_add(step)),
            Direction::South => (row.checked_add(step), Some(column)),
            Direction::SouthWest => (row.checked_add(step), column.checked_sub(step)),
            Direction::West => (Some(row), column.checked_sub(step)),
            Direction::NorthWest => (row.checked_sub(step), column.checked_sub(step)),
            Direction::North => (row.checked_sub(step), Some(column)),
        };
        Some((row?, column?))
    }

    fn all_directions() -> Vec<Direction> {
        let mut directions = Vec::new();
        directions.push(Direction::NorthEast);
        directions.push(Direction::East);
        directions.push(Direction::SouthEast);
        directions.push(Direction::South);
        directions.push(Direction::SouthWest);
        directions.push(Direction::West);
        directions.push(Direction::NorthWest);
        directions.push(Direction::North);
        directions
    }
}

fn make_grid(input: &str) -> Option<Array2D<char>> {
    let lines = input.lines();
    let mut rows = vec![];
    for line in lines {
        rows.push(line.chars().collect());
    }

    let grid = Array2D::from_rows(&rows);
    let grid = match grid {
        Ok(v) => v,
        Err(e) => {
            println!("Couldn't make grid from rows: {}", e);
            return None;
        }
    };
    Some(grid)
}

fn count_words(grid: &Array2D<char>, row: usize, column: usize) -> u32 {
    if grid.get(row, column) != Some(&WORD.chars().next().unwrap_or('.')) {
        return 0;
    }

    let mut count = 0;
    for direction in Direction::all_directions() {
        let mut word_found = true;
        for step in 1..WORD.len() {
            let coordinates = direction.step_towards(row, column, step);
            let (r, c) = match coordinates {
                Some((v1, v2)) => (v1, v2),
                None => {
                    word_found = false;
                    break;
                }
            };
            if grid.get(r, c) != Some(&WORD.chars().nth(step).unwrap_or('.')) {
                word_found = false;
                break;
            }
        }
        if word_found {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
