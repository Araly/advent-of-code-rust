use regex::{Captures, Regex};

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();
    let mut total_sum = 0;
    for line in lines {
        let instructions = parse_instruction(line)?;
        let mut sum = 0;
        for m in instructions {
            if let Instruction::Multiplication(_, _) = m {
                sum += m.multiply()?;
            }
        }
        total_sum += sum;
    }

    Some(total_sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines();
    let mut total_sum = 0;
    for line in lines {
        let instructions = parse_instruction(line)?;
        let mut sum = 0;
        let mut is_doing = true;
        for i in instructions {
            match i {
                Instruction::Multiplication(_, _) => {
                    if is_doing {
                        sum += i.multiply()?;
                    }
                }
                Instruction::Do => {
                    is_doing = true;
                }
                Instruction::Dont => {
                    is_doing = false;
                }
            }
        }
        total_sum += sum;
    }

    Some(total_sum)
}

const INSTRUCTION_MATCHER: &str =
    r"(?<mul>mul\((?<x>\d{1,3}),(?<y>\d{1,3})\))|(?<do>do\(\))|(?<dont>don't\(\))";

#[derive(Debug)]
enum Instruction {
    Multiplication(u32, u32),
    Do,
    Dont,
}

impl Instruction {
    fn new(capture: Captures<'_>) -> Option<Instruction> {
        match capture.name("do") {
            Some(_) => return Some(Instruction::Do),
            None => {}
        }

        match capture.name("dont") {
            Some(_) => return Some(Instruction::Dont),
            None => {}
        }

        match capture.name("mul") {
            Some(_) => {
                let x = capture.name("x")?.as_str();
                let y = capture.name("y")?.as_str();

                let x = x.parse();
                let y = y.parse();

                let x = match x {
                    Ok(v) => v,
                    Err(e) => {
                        println!("Couldn't parse u32 for {:#?}: {e}", capture);
                        return None;
                    }
                };
                let y = match y {
                    Ok(v) => v,
                    Err(e) => {
                        println!("Couldn't parse u32 for {:#?}: {e}", capture);
                        return None;
                    }
                };

                return Some(Instruction::Multiplication(x, y));
            }
            None => return None,
        }
    }

    fn multiply(&self) -> Option<u32> {
        match self {
            Instruction::Multiplication(n1, n2) => Some(n1 * n2),
            _ => None,
        }
    }
}

fn parse_instruction(input: &str) -> Option<Vec<Instruction>> {
    let re = Regex::new(INSTRUCTION_MATCHER);
    let re = match re {
        Ok(v) => v,
        Err(e) => {
            println!("{e}");
            return None;
        }
    };
    let mut instructions = vec![];
    for capture in re.captures_iter(input) {
        instructions.push(Instruction::new(capture)?);
    }
    Some(instructions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
