use array2d::Array2D;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let mut map = Map::new(input)?;
    // println!(
    //     "guard: {:?},\tvisited: {}",
    //     map.guard, map.cells_visited_count
    // );
    let mut done = false;
    while !done {
        done = map.step(false)?;
        // println!(
        //     "guard: {:?},\tvisited: {}",
        //     map.guard, map.cells_visited_count
        // );
    }
    Some(map.cells_visited_count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut map = Map::new(input)?;
    // println!("guard: {:?},\tobstacles: {:?}", map.guard, map.obstacles);
    let mut done = false;
    while !done {
        done = map.step(true)?;
        // println!("guard: {:?},\tobstacles: {:?}", map.guard, map.obstacles);
    }

    let obstacles: u32 = u32::try_from(map.obstacles.len()).unwrap_or(0);
    Some(obstacles)
}

const TILE: char = '.';
const WALL: char = '#';
const GUARD_EAST: char = '>';
const GUARD_SOUTH: char = 'v';
const GUARD_WEST: char = '<';
const GUARD_NORTH: char = '^';

#[derive(Clone, Debug, PartialEq)]
enum Direction {
    East,
    South,
    West,
    North,
    None,
}

impl Direction {
    fn rotate_right(&self) -> Direction {
        match self {
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
            Direction::North => Direction::East,
            Direction::None => Direction::None,
        }
    }

    fn step(&self, row: usize, column: usize) -> Option<(usize, usize)> {
        let (row, column) = match self {
            Direction::East => (Some(row), column.checked_add(1)),
            Direction::South => (row.checked_add(1), Some(column)),
            Direction::West => (Some(row), column.checked_sub(1)),
            Direction::North => (row.checked_sub(1), Some(column)),
            Direction::None => return None,
        };
        Some((row?, column?))
    }
}

#[derive(Clone, Debug)]
enum Cell {
    Guard(Direction),
    Tile(bool, Direction),
    Wall,
}

impl Cell {
    fn new(input: char) -> Option<Cell> {
        match input {
            TILE => Some(Cell::Tile(false, Direction::None)),
            WALL => Some(Cell::Wall),
            GUARD_EAST => Some(Cell::Guard(Direction::East)),
            GUARD_SOUTH => Some(Cell::Guard(Direction::South)),
            GUARD_WEST => Some(Cell::Guard(Direction::West)),
            GUARD_NORTH => Some(Cell::Guard(Direction::North)),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct Map {
    grid: Array2D<Cell>,
    guard: (usize, usize, Direction),
    cells_visited_count: u32,
    obstacles: Vec<(usize, usize)>,
}

impl Map {
    fn new(input: &str) -> Option<Map> {
        let lines = input.lines();
        let mut rows = vec![];
        let mut x = 0;
        let mut guard = (0, 0, Direction::East);
        for line in lines {
            let mut y = 0;
            let mut row = vec![];
            for c in line.chars() {
                let cell = match Cell::new(c)? {
                    Cell::Guard(d) => {
                        guard = (x, y, d);
                        Cell::Tile(false, Direction::None)
                    }
                    v => v,
                };
                row.push(cell);
                y += 1;
            }
            rows.push(row);
            x += 1;
        }

        let grid = match Array2D::from_rows(&rows) {
            Ok(v) => v,
            Err(e) => {
                println!("Couldn't make grid from rows: {}", e);
                return None;
            }
        };
        Some(Map {
            grid,
            guard,
            cells_visited_count: 0,
            obstacles: vec![],
        })
    }

    /// Returns whether the step was out of the grid
    fn step(&mut self, obstacles: bool) -> Option<bool> {
        // have the guard visit the current tile
        let current_cell = match self.grid.get(self.guard.0, self.guard.1) {
            Some(v) => v,
            None => return Some(true),
        };
        match current_cell {
            Cell::Tile(visited, direction) => {
                if direction != &Direction::None {
                    let guard_direction_rotated = &self.guard.2.rotate_right();
                    // println!(
                    //     "guard: {:?}, cell: {:?}, rotated: {:?}",
                    //     self.guard, current_cell, guard_direction_rotated
                    // );
                    if guard_direction_rotated == direction {
                        self.obstacles.push((self.guard.0, self.guard.1));
                    }
                }
                if !visited {
                    self.cells_visited_count += 1;
                    match self.grid.set(
                        self.guard.0,
                        self.guard.1,
                        Cell::Tile(true, self.guard.2.clone()),
                    ) {
                        Ok(_) => {}
                        Err(_) => {
                            println!("can't set tile");
                            return None;
                        }
                    }
                }
            }
            _ => {
                println!("visiting cell is not tile");
                return None;
            }
        };

        // have the guard step once
        let (next_x, next_y) = match self.guard.2.step(self.guard.0, self.guard.1) {
            Some(v) => v,
            None => return Some(true),
        };
        let next_cell = match self.grid.get(next_x, next_y) {
            Some(v) => v,
            None => return Some(true),
        };
        match next_cell {
            Cell::Tile(_, _) => {
                self.guard.0 = next_x;
                self.guard.1 = next_y;
            }
            Cell::Wall => {
                if obstacles {
                    self.paint_leading_line(self.guard.0, self.guard.1, self.guard.2.clone())?;
                }
                self.guard.2 = self.guard.2.rotate_right()
            }
            _ => {
                println!("next cell was not a tile or a wall");
                return None;
            }
        }
        Some(false)
    }

    fn paint_leading_line(&mut self, x: usize, y: usize, d: Direction) -> Option<()> {
        let mut x = x;
        let mut y = y;
        loop {
            let current_cell = match self.grid.get(x, y) {
                Some(v) => v,
                None => return Some(()),
            };
            match current_cell {
                Cell::Tile(visited, _) => {
                    match self.grid.set(x, y, Cell::Tile(*visited, d.clone())) {
                        Ok(_) => {}
                        Err(_) => return None,
                    }
                    (x, y) = match d.rotate_right().rotate_right().step(x, y) {
                        Some(v) => v,
                        None => return Some(()),
                    };
                }
                _ => break,
            }
        }
        Some(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
