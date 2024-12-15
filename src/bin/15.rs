use core::fmt;

use array2d::Array2D;

advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<usize> {
    let (mut warehouse, directions) = match parse_input(input) {
        Ok(v) => v,
        Err(e) => {
            println!("couldn't parse input: {e}");
            return None;
        }
    };
    println!("initial state:");
    warehouse.print_map();
    for d in directions {
        println!("\nmove {}:", d);
        warehouse.step(d);
        warehouse.print_map();
    }
    Some(warehouse.sum_box_coordinates())
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[derive(Clone)]
enum Tile {
    Floor,
    Wall,
    Box,
    Robot,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Floor => '.',
                Tile::Wall => '#',
                Tile::Box => 'O',
                Tile::Robot => '@',
            }
        )
    }
}

impl Tile {
    fn new(tile: char) -> Option<Tile> {
        match tile {
            '#' => Some(Tile::Wall),
            '.' => Some(Tile::Floor),
            'O' => Some(Tile::Box),
            '@' => Some(Tile::Robot),
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
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

    fn new(input: char) -> Option<Direction> {
        match input {
            '>' => Some(Direction::East),
            'v' => Some(Direction::South),
            '<' => Some(Direction::West),
            '^' => Some(Direction::North),
            _ => None,
        }
    }
}

struct Robot {
    x: usize,
    y: usize,
}

struct Warehouse {
    robot: Robot,
    map: Array2D<Tile>,
}

impl Warehouse {
    fn print_map(&self) {
        let mut y = 0;
        for row in self.map.rows_iter() {
            let mut x = 0;
            for tile in row {
                if x == self.robot.x && y == self.robot.y {
                    print!("@");
                } else {
                    print!("{tile}");
                }
                x += 1;
            }
            println!("");
            y += 1;
        }
    }

    fn step(&mut self, direction: Direction) {
        let (next_y, next_x) = direction.step(self.robot.y, self.robot.x).unwrap();
        let next_tile = self.map.get(next_y, next_x).unwrap();
        match next_tile {
            Tile::Floor => {
                self.robot = Robot {
                    x: next_x,
                    y: next_y,
                }
            }
            Tile::Box => {
                if self.push_box(next_x, next_y, direction) {
                    self.robot = Robot {
                        x: next_x,
                        y: next_y,
                    }
                }
            }
            Tile::Wall => return,
            Tile::Robot => {
                println!("found a Robot tile while stepping");
                return;
            }
        }
    }
    fn push_box(&mut self, x: usize, y: usize, d: Direction) -> bool {
        let (next_y, next_x) = d.step(y, x).unwrap();
        let next_tile = self.map.get(next_y, next_x).unwrap();
        match next_tile {
            Tile::Floor => {
                self.map.set(y, x, Tile::Floor).unwrap();
                self.map.set(next_y, next_x, Tile::Box).unwrap();
                return true;
            }
            Tile::Box => {
                if self.push_box(next_x, next_y, d) {
                    self.map.set(y, x, Tile::Floor).unwrap();
                    self.map.set(next_y, next_x, Tile::Box).unwrap();
                    return true;
                } else {
                    return false;
                }
            }
            _ => return false,
        }
    }

    fn sum_box_coordinates(&self) -> usize {
        let mut sum = 0;
        let mut y = 0;
        for row in self.map.rows_iter() {
            let mut x = 0;
            for tile in row {
                if let Tile::Box = tile {
                    sum += 100 * y + x;
                }
                x += 1;
            }
            y += 1;
        }
        sum
    }
}

fn parse_input(input: &str) -> Result<(Warehouse, Vec<Direction>), String> {
    let mut parsing_map = true;
    let mut y = 0;
    let mut robot = Robot { x: 0, y: 0 };
    let mut rows = vec![];
    let mut directions = vec![];
    for line in input.lines() {
        let mut x = 0;
        if parsing_map {
            if line == "" {
                parsing_map = false;
                continue;
            }
            let mut row = vec![];
            for tile in line.chars() {
                let mut tile = match Tile::new(tile) {
                    Some(v) => v,
                    None => return Err(format!("couldn't make new tile from {tile}")),
                };
                if let Tile::Robot = tile {
                    robot = Robot { x, y };
                    tile = Tile::Floor;
                }
                row.push(tile);
                x += 1;
            }
            rows.push(row);
            y += 1;
        } else {
            for c in line.chars() {
                let direction = match Direction::new(c) {
                    Some(v) => v,
                    None => return Err(format!("couldn't make new direction from {c}")),
                };
                directions.push(direction);
            }
        }
    }
    Ok((
        Warehouse {
            robot,
            map: Array2D::from_rows(&rows).unwrap(),
        },
        directions,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
