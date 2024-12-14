use std::collections::HashSet;

use array2d::Array2D;
use regex::Regex;

advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<i64> {
    let mut robots = Robot::parse(input);
    let dimensions = match robots.len() {
        12 => DIMENSIONS_SMALL,
        _ => DIMENSIONS_BIG,
    };

    Robot::all_step(&mut robots, 100, dimensions, false);
    Robot::all_print(&mut robots, dimensions);
    let quadrants = Robot::quadrants(&mut robots, dimensions);
    let factor = quadrants.0 * quadrants.1 * quadrants.2 * quadrants.3;
    Some(factor)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut robots = Robot::parse(input);
    let dimensions = match robots.len() {
        12 => DIMENSIONS_SMALL,
        _ => DIMENSIONS_BIG,
    };

    let (second, score) = Robot::all_step(&mut robots, 10000, dimensions, true);
    let second = second + 1; //to account for counting seconds from 1
    let mut robots = Robot::parse(input);
    Robot::all_step(&mut robots, second, dimensions, false);
    Robot::all_print(&mut robots, dimensions);
    println!("after {} seconds, score: {}", second, score);
    Some(second)
}

const DIMENSIONS_SMALL: (i32, i32) = (11, 7);
const DIMENSIONS_BIG: (i32, i32) = (101, 103);
const ROBOT_MATCHER: &str = r"p=(?<x>\d+),(?<y>\d+) v=(?<vx>[+-]?\d+),(?<vy>[+-]?\d+)";

#[derive(Debug)]
struct Robot {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}

impl Robot {
    fn step(&mut self, dimensions: (i32, i32)) {
        self.x += self.vx;
        if self.x < 0 {
            self.x = dimensions.0 + self.x;
        } else if self.x >= dimensions.0 {
            self.x = self.x - dimensions.0;
        }

        self.y += self.vy;
        if self.y < 0 {
            self.y = dimensions.1 + self.y;
        } else if self.y >= dimensions.1 {
            self.y = self.y - dimensions.1;
        }
    }

    fn parse(input: &str) -> Vec<Robot> {
        let mut robots = vec![];
        let re = Regex::new(ROBOT_MATCHER).unwrap();
        for line in input.lines() {
            let capture = re.captures(line).unwrap();
            robots.push(Robot {
                x: capture.name("x").unwrap().as_str().parse().unwrap(),
                y: capture.name("y").unwrap().as_str().parse().unwrap(),
                vx: capture.name("vx").unwrap().as_str().parse().unwrap(),
                vy: capture.name("vy").unwrap().as_str().parse().unwrap(),
            });
        }
        robots
    }

    fn all_step(
        robots: &mut Vec<Robot>,
        seconds: u32,
        dimensions: (i32, i32),
        calculate_score: bool,
    ) -> (u32, i64) {
        let mut maximum: i64 = 0;
        let mut second = 0;
        for i in 0..seconds {
            for robot in &mut *robots {
                robot.step(dimensions);
            }
            if calculate_score {
                let score = Robot::tree_score(robots);
                if score > maximum {
                    maximum = score;
                    second = i;
                }
            }
        }
        (second, maximum)
    }

    fn all_print(robots: &mut Vec<Robot>, dimensions: (i32, i32)) {
        let mut grid = Array2D::filled_with(
            0,
            usize::try_from(dimensions.1).unwrap(),
            usize::try_from(dimensions.0).unwrap(),
        );
        for robot in robots {
            let cell = match grid.get_mut(
                usize::try_from(robot.y).unwrap(),
                usize::try_from(robot.x).unwrap(),
            ) {
                Some(v) => v,
                None => {
                    println!(
                        "couldn't get from grid for {:?}, dimensions: {:?}",
                        robot, dimensions
                    );
                    return;
                }
            };
            *cell += 1;
        }
        for row in grid.rows_iter() {
            for cell in row {
                match cell {
                    0 => print!("."),
                    _ => print!("{cell}"),
                }
            }
            println!("");
        }
    }

    fn quadrants(robots: &mut Vec<Robot>, dimensions: (i32, i32)) -> (i64, i64, i64, i64) {
        let mut quadrants = (0, 0, 0, 0);
        for robot in robots {
            if robot.x < dimensions.0 / 2 {
                if robot.y < dimensions.1 / 2 {
                    // top left
                    quadrants.0 += 1;
                } else if robot.y > dimensions.1 / 2 {
                    // bottom left
                    quadrants.1 += 1;
                }
            } else if robot.x > dimensions.0 / 2 {
                if robot.y < dimensions.1 / 2 {
                    // top right
                    quadrants.2 += 1;
                } else if robot.y > dimensions.1 / 2 {
                    // bottom right
                    quadrants.3 += 1;
                }
            }
        }
        quadrants
    }

    fn tree_score(robots: &Vec<Robot>) -> i64 {
        let mut score = 0;
        let mut set = HashSet::new();
        for robot in robots {
            set.insert((robot.x, robot.y));
        }
        for robot in robots {
            if let Some(_) = set.get(&(robot.x, robot.y - 1)) {
                score += 1;
            }
        }
        score
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
