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

pub fn part_two(input: &str) -> Option<u32> {
    let mut plots = match parse_farm(input) {
        Ok(v) => v,
        Err(e) => {
            println!("couldn't parse farm: {e}");
            return None;
        }
    };

    // let (x, y) = (0, 0);
    let mut count = 0;
    for y in 0..plots.row_len() {
        for x in 0..plots.column_len() {
            let plot = plots.get(y, x)?;
            if plot.processed {
                continue;
            }
            let plant = plot.plant;
            let mut edges = Array2D::filled_with(None, plots.num_rows(), plots.num_columns());
            let area = match find_edges(&mut plots, plant, x, y, Direction::North, &mut edges) {
                Ok(v) => v,
                Err(e) => {
                    println!("couldn't find edges: {e}");
                    return None;
                }
            };
            let rotations = match follow_all_edges(&mut edges) {
                Ok(v) => v,
                Err(e) => {
                    println!("couldn't follow edge: {e}");
                    return None;
                }
            };
            // println!(
            //     "A region of {} plants with price {} * {}",
            //     plant, area, rotations
            // );
            count += area * rotations;
        }
    }
    Some(count)
}

const MAXIMUM_LOOP: u32 = 500;

#[derive(Clone)]
struct Plot {
    plant: char,
    processed: bool,
}

#[derive(Clone, Debug, PartialEq)]
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

#[derive(Clone, Debug)]
struct Edge {
    outside: Vec<Direction>,
    processed: bool,
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

fn find_edges(
    plots: &mut Array2D<Plot>,
    friend: char,
    x: usize,
    y: usize,
    d: Direction,
    edges: &mut Array2D<Option<Edge>>,
) -> Result<u32, String> {
    let plot = match plots.get_mut(y, x) {
        Some(v) => v,
        None => {
            add_edge(edges, x, y, d)?;
            return Ok(0);
        }
    };
    // stop if we've already taken this plot into account
    if plot.processed {
        return Ok(0);
    }

    let mut area = 1;
    plot.processed = true;
    for d in Direction::all() {
        let (next_y, next_x) = match d.step(y, x) {
            Some(v) => v,
            None => {
                add_edge(edges, x, y, d)?;
                continue;
            }
        };
        let next_plot = match plots.get(next_y, next_x) {
            Some(v) => v,
            None => {
                add_edge(edges, x, y, d)?;
                continue;
            }
        };
        if next_plot.plant != friend {
            add_edge(edges, x, y, d)?;
            continue;
        }
        let a = find_edges(plots, friend, next_x, next_y, d, edges)?;
        area += a;
    }
    Ok(area)
}

fn add_edge(
    edges: &mut Array2D<Option<Edge>>,
    x: usize,
    y: usize,
    d: Direction,
) -> Result<(), String> {
    let current_edge = match edges.get_mut(y, x) {
        Some(v) => v,
        None => return Err(format!("couldn't get edge for ({x},{y})")),
    };
    match current_edge {
        Some(current_edge) => {
            current_edge.outside.push(d);
        }
        None => {
            match edges.set(
                y,
                x,
                Some(Edge {
                    outside: vec![d],
                    processed: false,
                }),
            ) {
                Ok(v) => v,
                Err(e) => return Err(format!("couldn't set edge: {e}")),
            };
            return Ok(());
        }
    };
    Ok(())
}

fn print_edges(edges: Array2D<Option<Edge>>) {
    for y in 0..edges.row_len() {
        for x in 0..edges.column_len() {
            let edge = edges.get(y, x).unwrap();
            match edge {
                Some(edge) => {
                    print!("{}", edge.outside.len());
                }
                None => {
                    print!(".");
                }
            }
        }
        println!("");
    }
}

fn follow_edge(
    edges: &mut Array2D<Option<Edge>>,
    start_x: usize,
    start_y: usize,
    start_d: Direction,
) -> Result<u32, String> {
    let (mut x, mut y, mut d) = (start_x, start_y, start_d.clone());
    let mut rotations = 0;
    let mut i = 0;
    loop {
        // println!(
        //     "following edge at ({x},{y},{:?}): {rotations} rotations, i: {i}",
        //     d
        // );
        if i > MAXIMUM_LOOP {
            return Err("caught in a loop".to_string());
        }
        i += 1;
        let edge = match edges.get_mut(y, x).unwrap() {
            Some(v) => v,
            None => {
                d = d.rotate_left();
                rotations += 1;
                (y, x) = match d.rotate_right().step(y, x) {
                    Some(v) => v,
                    None => return Err(format!("couldn't step towards {:?} at ({x},{y})", d)),
                };
                continue;
            }
        };
        if x == start_x && y == start_y && d == start_d && edge.processed {
            break;
        }
        edge.processed = true;
        if !edge.outside.contains(&d) {
            d = d.rotate_left();
            rotations += 1;
        }
        if edge.outside.contains(&d.rotate_right()) {
            d = d.rotate_right();
            rotations += 1;
            continue;
        }
        (y, x) = match d.rotate_right().step(y, x) {
            Some(v) => v,
            None => return Err(format!("couldn't step towards {:?} at ({x},{y})", d)),
        };
    }
    Ok(rotations)
}

fn follow_all_edges(edges: &mut Array2D<Option<Edge>>) -> Result<u32, String> {
    let mut rotations = 0;
    for y in 0..edges.row_len() {
        for x in 0..edges.column_len() {
            let edge = edges.get(y, x).unwrap();
            match edge {
                Some(edge) => {
                    if !edge.processed {
                        rotations += follow_edge(edges, x, y, edge.outside[0].clone())?;
                    }
                }
                None => continue,
            }
        }
    }
    Ok(rotations)
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
        assert_eq!(result, Some(1206));
    }
}
