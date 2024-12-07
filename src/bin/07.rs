advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let lines = input.lines();
    let mut count = 0;
    for line in lines {
        let mut equation = Equation::new(line)?;
        if equation.generate_operations(false) {
            count += equation.result;
        };
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines = input.lines();
    let mut count = 0;
    for line in lines {
        let equation = Equation::new(line)?;
        if equation.generate_operations_recursive(true) {
            count += equation.result;
        };
    }
    Some(count)
}

#[derive(Debug, Clone)]
enum Operation {
    Add,
    Multiply,
    Concatenate,
}

impl Operation {
    fn next(operations: &mut Vec<Operation>, include_concat: bool) -> Option<()> {
        let mut i = 0;
        while i < operations.len() {
            match operations[i] {
                Operation::Add => {
                    operations[i] = Operation::Multiply;
                    return Some(());
                }
                Operation::Multiply => {
                    if include_concat {
                        operations[i] = Operation::Concatenate;
                        return Some(());
                    } else {
                        operations[i] = Operation::Add;
                        i += 1;
                    }
                }
                Self::Concatenate => {
                    operations[i] = Operation::Add;
                    i += 1;
                }
            }
        }
        None
    }

    fn calculate(&self, n1: u64, n2: u64) -> Option<u64> {
        let ten: u64 = 10;
        match self {
            Operation::Add => Some(n1 + n2),
            Operation::Multiply => Some(n1 * n2),
            Operation::Concatenate => Some(
                n1 * match ten.checked_pow(
                    match n2.checked_ilog10() {
                        Some(v) => v,
                        None => {
                            println!("log10 broke for {n2}");
                            return None;
                        }
                    } + 1,
                ) {
                    Some(v) => v,
                    None => {
                        println!("pow10 broke for {n2}");
                        return None;
                    }
                } + n2,
            ),
        }
    }
}

#[derive(Debug)]
struct Equation {
    result: u64,
    numbers: Vec<u64>,
    operations: Vec<Operation>,
}

impl Equation {
    fn new(input: &str) -> Option<Equation> {
        let mut parts = input.split(":");
        let result = parts.next()?.parse().unwrap();
        let numbers = parts
            .next()?
            .trim()
            .split(" ")
            .map(|v| v.parse::<u64>().unwrap())
            .collect();
        Some(Equation {
            result,
            numbers,
            operations: vec![],
        })
    }

    fn generate_operations(&mut self, include_concat: bool) -> bool {
        let mut operations = vec![];
        for _ in 1..self.numbers.len() {
            operations.push(Operation::Add);
        }
        // println!("\tdefault operations: {:?}", operations);
        self.operations = operations;
        loop {
            if self.check() {
                // println!("\tequation is good");
                return true;
            }
            if let None = Operation::next(&mut self.operations, include_concat) {
                // println!("\tno more operation permutations: {:?}", self.operations);
                return false;
            }
            // println!("\t{:?}", self);
        }
    }

    fn check(&self) -> bool {
        let mut numbers = self.numbers.iter();
        let mut operations = self.operations.iter();
        let mut result = *match numbers.next() {
            Some(v) => v,
            None => return false,
        };
        loop {
            let operation = match operations.next() {
                Some(v) => v,
                None => break,
            };
            let number = *match numbers.next() {
                Some(v) => v,
                None => break,
            };
            result = operation.calculate(result, number).unwrap_or(0);
        }
        self.result == result
    }

    fn generate_operations_recursive(&self, include_concat: bool) -> bool {
        if self.numbers.len() <= 1 {
            println!("not enough numbers");
            return false;
        }
        self.recursive_generate(
            match include_concat {
                false => [Operation::Add, Operation::Multiply].to_vec(),
                true => [Operation::Add, Operation::Multiply, Operation::Concatenate].to_vec(),
            },
            1,
            self.numbers[0],
        )
    }

    fn recursive_generate(&self, operations: Vec<Operation>, depth: usize, total: u64) -> bool {
        if depth == self.numbers.len() {
            if total == self.result {
                return true;
            }
            return false;
        }
        if total > self.result {
            return false;
        }

        for operation in operations.clone() {
            let current_total = match operation.calculate(total, self.numbers[depth]) {
                Some(v) => v,
                None => return false,
            };
            if self.recursive_generate(operations.clone(), depth + 1, current_total) {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
