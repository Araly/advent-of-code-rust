use colored::{self, Colorize};
use std::collections::{HashMap, HashSet};

use array2d::Array2D;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<usize> {
    let (antennas_by_type, grid_size) = parse_antennas(input);
    let mut antinodes = vec![];
    for antennas in antennas_by_type {
        let mut antinodes_found = find_antinodes(antennas.1, false, grid_size);
        antinodes.append(&mut antinodes_found);
    }
    let antinodes = clean_results(antinodes, grid_size);
    // show(antennas_by_type, antinodes.clone(), grid_size);
    Some(antinodes.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let (antennas_by_type, grid_size) = parse_antennas(input);
    let mut antinodes = vec![];
    for antennas in antennas_by_type {
        let mut antinodes_found = find_antinodes(antennas.1, true, grid_size);
        antinodes.append(&mut antinodes_found);
    }
    let antinodes = clean_results(antinodes, grid_size);
    // show(antennas_by_type, antinodes.clone(), grid_size);
    Some(antinodes.len())
}

const EMPTY_CHARACTER: char = '.';

fn parse_antennas(input: &str) -> (HashMap<char, Vec<(i32, i32)>>, (i32, i32)) {
    let mut antennas_by_type = HashMap::new();
    let mut x = 0;
    let mut y = 0;
    for line in input.lines() {
        y = 0;
        for character in line.chars() {
            if character != EMPTY_CHARACTER {
                let antenna: &mut Vec<(i32, i32)> = antennas_by_type.entry(character).or_default();
                antenna.push((x, y));
            }
            y += 1;
        }
        x += 1;
    }
    (antennas_by_type, (x, y))
}

fn find_antinodes(
    antennas: Vec<(i32, i32)>,
    harmonics: bool,
    grid_size: (i32, i32),
) -> Vec<(i32, i32)> {
    if antennas.len() <= 1 {
        return vec![];
    }
    if antennas.len() == 2 {
        match harmonics {
            true => return calculate_harmonics(antennas[0], antennas[1], grid_size),
            false => return calculate_antinodes(antennas[0], antennas[1]),
        }
    }
    let mut antinodes = vec![];
    for i in 1..antennas.len() {
        let mut antinodes_calculated = match harmonics {
            true => calculate_harmonics(antennas[0], antennas[i], grid_size),
            false => calculate_antinodes(antennas[0], antennas[i]),
        };
        antinodes.append(&mut antinodes_calculated);
    }
    let mut antinodes_from_recursing = find_antinodes(antennas[1..].to_vec(), harmonics, grid_size);
    antinodes.append(&mut antinodes_from_recursing);
    antinodes
}

fn calculate_antinodes(antenna1: (i32, i32), antenna2: (i32, i32)) -> Vec<(i32, i32)> {
    let difference = (antenna2.0 - antenna1.0, antenna2.1 - antenna1.1);

    let antinode1 = (antenna1.0 - difference.0, antenna1.1 - difference.1);
    let antinode2 = (antenna2.0 + difference.0, antenna2.1 + difference.1);
    vec![antinode1, antinode2]
}

fn calculate_harmonics(
    antenna1: (i32, i32),
    antenna2: (i32, i32),
    grid_size: (i32, i32),
) -> Vec<(i32, i32)> {
    let difference = (antenna2.0 - antenna1.0, antenna2.1 - antenna1.1);
    let mut harmonics = vec![antenna1, antenna2];
    let mut harmonic = (antenna1.0 - difference.0, antenna1.1 - difference.1);
    loop {
        if !is_in_grid(harmonic, grid_size) {
            break;
        }
        harmonics.push(harmonic);
        harmonic = (harmonic.0 - difference.0, harmonic.1 - difference.1);
    }
    let mut harmonic = (antenna2.0 + difference.0, antenna2.1 + difference.1);
    loop {
        if !is_in_grid(harmonic, grid_size) {
            break;
        }
        harmonics.push(harmonic);
        harmonic = (harmonic.0 + difference.0, harmonic.1 + difference.1);
    }
    harmonics
}

fn is_in_grid(cell: (i32, i32), grid_size: (i32, i32)) -> bool {
    cell.0 >= 0 && cell.0 < grid_size.0 && cell.1 >= 0 && cell.1 < grid_size.1
}

fn clean_results(cells: Vec<(i32, i32)>, grid_size: (i32, i32)) -> HashSet<(i32, i32)> {
    let mut cells_cleaned = HashSet::new();
    for cell in cells {
        if is_in_grid(cell, grid_size) {
            cells_cleaned.insert(cell);
        }
    }
    cells_cleaned
}

fn show(
    antennas_by_character: HashMap<char, Vec<(i32, i32)>>,
    antinodes: HashSet<(i32, i32)>,
    grid_size: (i32, i32),
) {
    println!("grid_size: {:?}", grid_size);
    let mut grid = Array2D::filled_with(
        ".".white(),
        grid_size.0.try_into().unwrap(),
        grid_size.1.try_into().unwrap(),
    );
    for (character, antennas) in antennas_by_character {
        for antenna in antennas {
            if let Err(_) = grid.set(
                antenna.0.try_into().unwrap(),
                antenna.1.try_into().unwrap(),
                character.to_string().blue(),
            ) {
                println!("Couldn't set antenna{:?}", antenna);
            }
        }
    }
    for antinode in antinodes {
        let character = match grid.get(
            antinode.0.try_into().unwrap(),
            antinode.1.try_into().unwrap(),
        ) {
            Some(v) => v,
            None => &".".red(),
        };
        if character == &".".white() {
            if let Err(_) = grid.set(
                antinode.0.try_into().unwrap(),
                antinode.1.try_into().unwrap(),
                "#".green(),
            ) {
                println!("Couldn't set antinode{:?}", antinode);
            }
        } else {
            if let Err(_) = grid.set(
                antinode.0.try_into().unwrap(),
                antinode.1.try_into().unwrap(),
                character.clone().green(),
            ) {
                println!("Couldn't set antinode{:?}", antinode);
            }
        }
    }
    for row in grid.rows_iter() {
        for element in row {
            print!("{element}");
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
