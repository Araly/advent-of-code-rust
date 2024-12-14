use regex::Regex;

advent_of_code::solution!(13);

const BUTTON_MATCHER: &str = r".*X.(?<x>\d+),.Y.(?<y>\d+)";
const BIG_NUMBER: f64 = 10000000000000_f64;

#[derive(Debug)]
struct Equation {
    x: f64,
    y: f64,
    x_a: f64,
    x_b: f64,
    y_a: f64,
    y_b: f64,
}

impl Equation {
    fn calculate_b(&self) -> f64 {
        (self.y * self.x_a - self.y_a * self.x) / (-self.y_a * self.x_b + self.x_a * self.y_b)
    }

    fn calculate_a(&self, b: f64) -> f64 {
        (self.x - b * self.x_b) / self.x_a
    }
}

pub fn part_one(input: &str) -> Option<f64> {
    let equations = parse_equations(input, 0_f64).unwrap();
    let mut count = 0_f64;
    for equation in equations {
        println!("equation: {:?}", equation);
        let b = equation.calculate_b();
        let a = equation.calculate_a(b);
        print!("\ta: {a}, b: {b}");
        if a.floor() == a && b.floor() == b {
            println!(" is possible !");
            count += a * 3_f64 + b;
        } else {
            println!(" is a scam...");
        }
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<f64> {
    let equations = parse_equations(input, BIG_NUMBER).unwrap();
    let mut count = 0_f64;
    for equation in equations {
        println!("equation: {:?}", equation);
        let b = equation.calculate_b();
        let a = equation.calculate_a(b);
        print!("\ta: {a}, b: {b}");
        if a.floor() == a && b.floor() == b {
            println!(" is possible !");
            count += a * 3_f64 + b;
        } else {
            println!(" is a scam...");
        }
    }
    Some(count)
}

fn parse_equations(input: &str, offset: f64) -> Result<Vec<Equation>, String> {
    let mut equations = vec![];
    let re = Regex::new(BUTTON_MATCHER).unwrap();
    let mut lines = input.lines();
    loop {
        let line = match lines.next() {
            Some(v) => v,
            None => break,
        };
        let capture = re.captures(line).unwrap();
        let x_a = capture.name("x").unwrap().as_str().parse().unwrap();
        let y_a = capture.name("y").unwrap().as_str().parse().unwrap();
        let line = lines.next().unwrap();
        let capture = re.captures(line).unwrap();
        let x_b = capture.name("x").unwrap().as_str().parse().unwrap();
        let y_b = capture.name("y").unwrap().as_str().parse().unwrap();
        let line = lines.next().unwrap();
        let capture = re.captures(line).unwrap();
        let x = offset + capture.name("x").unwrap().as_str().parse::<f64>().unwrap();
        let y = offset + capture.name("y").unwrap().as_str().parse::<f64>().unwrap();
        equations.push(Equation {
            x,
            y,
            x_a,
            x_b,
            y_a,
            y_b,
        });
        lines.next();
    }

    Ok(equations)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480_f64));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908_f64));
    }
}
