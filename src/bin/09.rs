use std::usize;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<usize> {
    let mut blocks = parse_blocks(input)?;
    // print_blocks(blocks.clone());
    fragment_blocks(&mut blocks)?;
    // print_blocks(blocks.clone());
    Some(checksum_blocks(blocks))
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut files = parse_files(input)?;
    // print_files(files.clone());
    fragment_files(&mut files)?;
    // print_files(files.clone());
    Some(checksum_files(files))
}

fn parse_blocks(input: &str) -> Option<Vec<Option<usize>>> {
    let mut is_file = true;
    let mut file_index = 0;
    let mut blocks = vec![];
    for c in input.chars() {
        if c == '\n' {
            break;
        }
        let Some(size) = c.to_digit(10) else {
            println!("character {c} doesn't parse to digit");
            return None;
        };
        let size = match size.try_into() {
            Ok(v) => v,
            Err(e) => {
                println!("character {c} doesn't try into usize: {e}");
                return None;
            }
        };
        match is_file {
            true => {
                blocks.append(&mut vec![Some(file_index); size]);
                is_file = false;
                file_index += 1;
            }
            false => {
                blocks.append(&mut vec![None; size]);
                is_file = true;
            }
        }
    }
    Some(blocks)
}

fn parse_files(input: &str) -> Option<Vec<(Option<usize>, usize)>> {
    let mut is_file = true;
    let mut file_index = 0;
    let mut files = vec![];
    for c in input.chars() {
        if c == '\n' {
            break;
        }
        let Some(size) = c.to_digit(10) else {
            println!("character {c} doesn't parse to digit");
            return None;
        };
        let size = match size.try_into() {
            Ok(v) => v,
            Err(e) => {
                println!("character {c} doesn't try into usize: {e}");
                return None;
            }
        };
        match is_file {
            true => {
                files.push((Some(file_index), size));
                is_file = false;
                file_index += 1;
            }
            false => {
                files.push((None, size));
                is_file = true;
            }
        }
    }
    Some(files)
}

fn print_blocks(blocks: Vec<Option<usize>>) {
    for block in blocks {
        match block {
            Some(v) => print!("{v}"),
            None => print!("."),
        }
    }
    println!();
}

fn print_files(files: Vec<(Option<usize>, usize)>) {
    for (index, size) in files {
        match index {
            Some(v) => print!("{}", vec![v.to_string(); size].join("")),
            None => print!("{}", vec!['.'.to_string(); size].join("")),
        }
    }
    println!();
}

fn fragment_blocks(blocks: &mut Vec<Option<usize>>) -> Option<()> {
    if blocks.len() < 1 {
        return None;
    }

    let mut cursor_left = 0;
    let mut cursor_right = blocks.len() - 1;
    while cursor_left < cursor_right {
        if let None = blocks[cursor_right] {
            cursor_right -= 1;
            continue;
        }
        match blocks[cursor_left] {
            Some(_) => cursor_left += 1,
            None => {
                blocks.swap(cursor_left, cursor_right);
                cursor_right -= 1;
            }
        }
    }
    Some(())
}

fn fragment_files(files: &mut Vec<(Option<usize>, usize)>) -> Option<()> {
    if files.len() < 1 {
        return None;
    }

    let mut cursor_left = 0;
    let mut cursor_right = files.len() - 1;
    while 0 < cursor_right {
        cursor_left = 0;
        while cursor_left < cursor_right {
            // print!("({cursor_left}, {cursor_right})\t");
            // print_files(files.clone());
            let (index_right, size_right) = files[cursor_right];
            if let None = index_right {
                break;
            }
            let (index_left, size_left) = files[cursor_left];
            if let Some(_) = index_left {
                cursor_left += 1;
                continue;
            }
            match size_left.cmp(&size_right) {
                std::cmp::Ordering::Less => cursor_left += 1,
                std::cmp::Ordering::Equal => {
                    files.swap(cursor_left, cursor_right);
                    break;
                }
                std::cmp::Ordering::Greater => {
                    // split free file into two free files
                    files[cursor_left].1 = size_right;
                    files.insert(
                        cursor_left + 1,
                        (
                            None,
                            match size_left.checked_sub(size_right) {
                                Some(v) => v,
                                None => {
                                    println!("can't substract");
                                    return None;
                                }
                            },
                        ),
                    );
                    files.swap(cursor_left, cursor_right + 1);
                    break;
                }
            }
        }
        cursor_right -= 1;
    }
    Some(())
}

fn checksum_blocks(blocks: Vec<Option<usize>>) -> usize {
    let mut sum = 0;
    for i in 0..blocks.len() {
        if let Some(v) = blocks[i] {
            sum += i * v;
        }
    }
    sum
}

fn checksum_files(files: Vec<(Option<usize>, usize)>) -> usize {
    let mut sum = 0;
    let mut block_position = 0;
    for (index, size) in files {
        for _ in 0..size {
            if let Some(v) = index {
                sum += block_position * v;
            }
            block_position += 1;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
