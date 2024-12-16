use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    fmt,
};

use array2d::Array2D;

advent_of_code::solution!(16);

pub fn part_one(input: &str) -> Option<usize> {
    let (grid, start, end) = match parse_grid(input) {
        Ok(v) => v,
        Err(e) => {
            println!("couldn't parse grid: {e}");
            return None;
        }
    };
    search(&grid, start, end)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[derive(Clone)]
enum Tile {
    Floor,
    Wall,
    Start,
    End,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Floor => '.',
                Tile::Wall => '#',
                Tile::Start => 'S',
                Tile::End => 'E',
            }
        )
    }
}

impl Tile {
    fn new(tile: char) -> Option<Tile> {
        match tile {
            '#' => Some(Tile::Wall),
            '.' => Some(Tile::Floor),
            'S' => Some(Tile::Start),
            'E' => Some(Tile::End),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
enum Direction {
    East,
    South,
    West,
    North,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Direction::East => '>',
                Direction::South => 'v',
                Direction::West => '<',
                Direction::North => '^',
            }
        )
    }
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

    fn rotate_left(&self) -> Direction {
        match self {
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
            Direction::North => Direction::West,
        }
    }

    fn rotate_right(&self) -> Direction {
        match self {
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
            Direction::North => Direction::East,
        }
    }
}

#[derive(Clone, Hash, Eq, PartialEq)]
struct Point {
    x: usize,
    y: usize,
    d: Direction,
}

#[derive(Hash, Eq, PartialEq, Clone)]
struct State {
    point: Point,
    cost: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.point.x.cmp(&other.point.x))
            .then_with(|| self.point.y.cmp(&other.point.y))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_grid(input: &str) -> Result<(Array2D<Tile>, State, Point), String> {
    let mut y = 0;
    let mut start = State {
        point: Point {
            x: 0,
            y: 0,
            d: Direction::East,
        },
        cost: 0,
    };
    let mut end = Point {
        x: 0,
        y: 0,
        d: Direction::East,
    };
    let mut rows = vec![];
    for line in input.lines() {
        let mut x = 0;
        let mut row = vec![];
        for tile in line.chars() {
            let mut tile = match Tile::new(tile) {
                Some(v) => v,
                None => return Err(format!("couldn't make new tile from {tile}")),
            };
            if let Tile::Start = tile {
                start = State {
                    point: Point {
                        x,
                        y,
                        d: Direction::East,
                    },
                    cost: 0,
                };
                tile = Tile::Floor;
            } else if let Tile::End = tile {
                end = Point {
                    x,
                    y,
                    d: Direction::East,
                };
                tile = Tile::Floor;
            }
            row.push(tile);
            x += 1;
        }
        rows.push(row);
        y += 1;
    }
    let grid = Array2D::from_rows(&rows).unwrap();
    Ok((grid, start, end))
}

fn next_states(grid: &Array2D<Tile>, state: State) -> Vec<State> {
    let mut states = vec![];
    let (next_y, next_x) = state.point.d.step(state.point.y, state.point.x).unwrap();
    if let Tile::Floor = grid.get(next_y, next_x).unwrap() {
        states.push(State {
            point: Point {
                x: next_x,
                y: next_y,
                d: state.point.d.clone(),
            },
            cost: 1,
        });
    }
    states.push(State {
        point: Point {
            x: state.point.x,
            y: state.point.y,
            d: state.point.d.rotate_left(),
        },
        cost: 1000,
    });
    states.push(State {
        point: Point {
            x: state.point.x,
            y: state.point.y,
            d: state.point.d.rotate_right(),
        },
        cost: 1000,
    });
    states
}

fn search(grid: &Array2D<Tile>, start: State, end: Point) -> Option<usize> {
    let mut frontier = BinaryHeap::new();
    let mut distance = HashMap::new();

    frontier.push(start.clone());
    distance.insert(start.point, 0_usize);

    while let Some(current_state) = frontier.pop() {
        if current_state.point.x == end.x && current_state.point.y == end.y {
            return Some(current_state.cost);
        }
        if current_state.cost > *distance.get(&current_state.point).unwrap() {
            continue;
        }
        for mut next_state in next_states(grid, current_state.clone()) {
            next_state.cost += current_state.cost;
            if let Some(next_distance) = distance.get(&next_state.point) {
                if next_state.cost > *next_distance {
                    continue;
                }
            }
            distance.insert(next_state.point.clone(), next_state.cost);
            frontier.push(next_state);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7036));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
