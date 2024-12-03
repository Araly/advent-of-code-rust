use std::{
    cmp::{max, min},
    collections::HashMap,
    num::ParseIntError,
};

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let (mut lefts, mut rights) = get_lists(input)?;

    lefts.sort();
    rights.sort();

    let mut differences = vec![];
    for i in 0..lefts.len() {
        let max = max(lefts[i], rights[i]);
        let min = min(lefts[i], rights[i]);

        differences.push(max - min);
    }

    Some(differences.iter().sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let (lefts, rights) = get_lists(input)?;

    let mut rights_count: HashMap<u32, u32> = HashMap::new();
    for right in rights {
        *rights_count.entry(right).or_default() += 1;
    }

    let mut count = 0;
    for left in lefts {
        let right_count = rights_count.get(&left).unwrap_or(&0);
        count += left * right_count;
    }

    Some(count)
}

fn get_lists(input: &str) -> Option<(Vec<u32>, Vec<u32>)> {
    let lines = input.lines();
    let mut lefts = vec![];
    let mut rights = vec![];
    for line in lines {
        let mut split = line.split(" ").filter(|&s| !s.is_empty());

        let left = split.next()?;
        let right = split.next()?;

        let left: Result<u32, ParseIntError> = left.parse();
        let left: u32 = match left {
            Ok(v) => v,
            Err(e) => {
                println!("Couldn't get u32 from left: {e}");
                return None;
            }
        };
        let right: Result<u32, ParseIntError> = right.parse();
        let right: u32 = match right {
            Ok(v) => v,
            Err(e) => {
                println!("Couldn't get u32 from right: {e}");
                return None;
            }
        };

        lefts.push(left);
        rights.push(right);
    }

    Some((lefts, rights))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
