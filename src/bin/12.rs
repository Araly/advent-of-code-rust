use std::collections::HashMap;

use array2d::{Array2D, Error};

advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<u32> {
    let (plots, regions) = match parse_farm(input) {
        Ok(v) => v,
        Err(e) => {
            println!("couldn't parse farm: {e}");
            return None;
        }
    };
    let mut count = 0;
    for ((y, x), region) in regions {
        let plot = plots.get(y, x).unwrap();
        let price = region.area * region.perimeter;
        println!(
            "A region of {} at ({},{}) with price {} * {} = {}",
            plot.plant, x, y, region.area, region.perimeter, price
        );
        count += price;
    }
    Some(count)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[derive(Clone)]
struct Plot {
    plant: char,
    region: (usize, usize),
}

struct Region {
    area: u32,
    perimeter: u32,
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
        vec![Direction::East, Direction::South, Direction::West, Direction::North]
    }
}

fn parse_farm(input: &str) -> Result<(Array2D<Plot>, HashMap<(usize, usize), Region>), Error> {
    let mut rows = vec![];
    for line in input.lines() {
        let mut row = vec![];
        for c in line.chars() {
            row.push(Plot {
                plant: c,
                region: (0, 0),
            });
        }
        rows.push(row);
    }
    let mut plots: Array2D<Plot> = Array2D::from_rows(&rows)?;
    let mut regions: HashMap<(usize, usize), Region> = HashMap::new();
    for y in 0..plots.row_len() {
        for x in 0..plots.column_len() {
            let mut region_index = (y, x);
            let mut region_index_up = None;
            let mut region_index_left = None;
            let mut region_index_merge = None;
            let mut region = Region {
                area: 1,
                perimeter: 0,
            };
            let mut region_up = Region {
                area: 0,
                perimeter: 0,
            };
            let mut region_left = Region {
                area: 0,
                perimeter: 0,
            };
            let plot_current = plots.get(y, x).unwrap();

            let mut found_up = false;
            if y == 0 {
                region.perimeter += 1;
            } else {
                let plot_up = plots.get(y - 1, x).unwrap();
                if plot_up.plant == plot_current.plant {
                    region_index = plot_up.region;
                    found_up = true;
                } else {
                    region_index_up = Some(plot_up.region);
                    region.perimeter += 1;
                    region_up.perimeter += 1;
                }
            }
            if x == 0 {
                region.perimeter += 1;
            } else {
                let plot_left = plots.get(y, x - 1).unwrap();
                if plot_left.plant == plot_current.plant {
                    region_index = plot_left.region;
                    if found_up {
                        region_index_merge = Some(plot_left.region);
                    }
                } else {
                    region_index_left = Some(plot_left.region);
                    region.perimeter += 1;
                    region_left.perimeter += 1;
                }
            }
            if y == plots.row_len() - 1 {
                region.perimeter += 1;
            }
            if x == plots.column_len() - 1 {
                region.perimeter += 1;
            }

            let plot_current = plots.get_mut(y, x).unwrap();
            plot_current.region = region_index;
            regions
                .entry(region_index)
                .and_modify(|r| r.area += region.area)
                .and_modify(|r| r.perimeter += region.perimeter)
                .or_insert(region);
            if let Some(region_index_up) = region_index_up {
                regions
                    .entry(region_index_up)
                    .and_modify(|r| r.area += region_up.area)
                    .and_modify(|r| r.perimeter += region_up.perimeter)
                    .or_insert(region_up);
            }
            if let Some(region_index_left) = region_index_left {
                regions
                    .entry(region_index_left)
                    .and_modify(|r| r.area += region_left.area)
                    .and_modify(|r| r.perimeter += region_left.perimeter)
                    .or_insert(region_left);
            }
            if let Some(region_index_discard) = region_index_merge {
                let region_merge = regions.get(&region_index_discard).unwrap();
                let area = region_merge.area;
                let perimeter = region_merge.perimeter;
                regions
                    .entry(region_index)
                    .and_modify(|r| r.area += area)
                    .and_modify(|r| r.perimeter += perimeter);
                regions.remove(&region_index_discard).unwrap();
            }
        }
    }

    Ok((plots, regions))
}

fn find_friends(plots: Array2D<Plot>) -> (u32, u32) {
    let mut area = 0;
    let mut perimeter = 0;
    for
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
