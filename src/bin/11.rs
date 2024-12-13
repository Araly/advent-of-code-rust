use std::collections::HashMap;

advent_of_code::solution!(11);

const BIG_NUMBER: u64 = 2024;

pub fn part_one(input: &str) -> Option<u64> {
    let stones = match parse_stones(input) {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            return None;
        }
    };
    let mut count = 0;
    let mut cache = HashMap::new();
    for stone in stones {
        count += match blink(stone, 25, &mut cache) {
            Ok(v) => v,
            Err(e) => {
                println!("{}", e);
                return None;
            }
        };
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let stones = match parse_stones(input) {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            return None;
        }
    };
    let mut count = 0;
    let mut cache = HashMap::new();
    for stone in stones {
        count += match blink(stone, 75, &mut cache) {
            Ok(v) => v,
            Err(e) => {
                println!("{}", e);
                return None;
            }
        };
    }
    Some(count)
}

fn parse_stones(input: &str) -> Result<Vec<u64>, String> {
    let mut stones = vec![];
    for word in input.split(" ") {
        let stone = match word.trim().parse() {
            Ok(v) => v,
            Err(e) => return Err(format!("couldn't parse word \"{}\": {}", word, e)),
        };
        stones.push(stone);
    }
    Ok(stones)
}

fn blink(stone: u64, depth: u32, cache: &mut HashMap<(u64, u32), u64>) -> Result<u64, String> {
    // println!("({stone}, {depth})");
    if let Some(cached) = cache.get(&(stone, depth)) {
        return Ok(*cached);
    }

    let mut result = 0;
    if depth == 0 {
        result = 1;
    } else if stone == 0 {
        result = blink(1, depth - 1, cache)?;
    } else if (stone.ilog10() + 1) % 2 == 0 {
        let len = stone.ilog10() + 1;
        let left = stone
            / match 10_u64.checked_pow(len / 2) {
                Some(v) => v,
                None => {
                    return Err(format!("checked pow fails on len: {len}"));
                }
            };
        let left_count = blink(left, depth - 1, cache)?;
        let right_count = blink(
            stone
                - left
                    * match 10_u64.checked_pow(len / 2) {
                        Some(v) => v,
                        None => {
                            return Err(format!("checked pow fails on len: {len}"));
                        }
                    },
            depth - 1,
            cache,
        )?;
        result = match left_count.checked_add(right_count) {
            Some(v) => v,
            None => {
                return Err(format!(
                    "checked add fails on: ({stone}, {depth}), {left_count} + {right_count}"
                ));
            }
        };
    } else {
        result = blink(
            match stone.checked_mul(BIG_NUMBER) {
                Some(v) => v,
                None => return Err(format!("{stone} is too big for multiplication")),
            },
            depth - 1,
            cache,
        )?;
    }

    cache.insert((stone, depth), result);
    return Ok(result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
