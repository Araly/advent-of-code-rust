use std::num::ParseIntError;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let reports = get_reports(input)?;
    Some(get_safe_reports_count(reports, false))?
}

pub fn part_two(input: &str) -> Option<u32> {
    let reports = get_reports(input)?;
    Some(get_safe_reports_count(reports, true))?
}

#[derive(Debug)]
enum Direction {
    Unknown,
    Increasing,
    Decreasing,
}

fn get_reports(input: &str) -> Option<Vec<Vec<u32>>> {
    let lines = input.lines();
    let mut reports = vec![];
    for line in lines {
        let mut report = vec![];
        for number in line.split(" ") {
            let n: Result<u32, ParseIntError> = number.parse();
            match n {
                Ok(n) => report.push(n),
                Err(_) => {
                    println!("Couldn't parse number to u32: {}", number);
                    return None;
                }
            }
        }
        reports.push(report);
    }
    Some(reports)
}

fn get_safe_reports_count(reports: Vec<Vec<u32>>, problem_dampler: bool) -> Option<u32> {
    let mut count = 0;
    for report in reports {
        if is_safe_report(report, problem_dampler) {
            count += 1;
        }
    }

    Some(count)
}

fn is_safe_report(report: Vec<u32>, problem_dampler: bool) -> bool {
    // let mut output = "".to_string();
    if report.is_empty() {
        return false;
    }
    if report.len() == 1 {
        return true;
    }
    let mut safe = true;
    let mut direction = Direction::Unknown;
    for i in 1..report.len() {
        let previous: Result<i32, _> = report[i - 1].try_into();
        let next: Result<i32, _> = report[i].try_into();
        let previous = match previous {
            Ok(v) => v,
            Err(_) => return false,
        };
        let next = match next {
            Ok(v) => v,
            Err(_) => return false,
        };
        let difference: i32 = next - previous;
        match difference {
            -3..0 => {}
            0 => safe = false,
            1..4 => {}
            _ => safe = false,
        }
        match direction {
            Direction::Unknown => {
                if difference > 0 {
                    direction = Direction::Increasing;
                } else {
                    direction = Direction::Decreasing
                }
            }
            Direction::Increasing => {
                if difference < 0 {
                    safe = false;
                }
            }
            Direction::Decreasing => {
                if difference > 0 {
                    safe = false;
                }
            }
        }
        if !problem_dampler && !safe {
            return false;
        }
        if problem_dampler && !safe {
            // output = format!("{}damp {}", output, report[i - 1]);
            let mut report_removed = report.clone();
            report_removed.remove(i - 1);
            safe = is_safe_report(report_removed, false);
            if !safe {
                // output = format!("{}\t    bump {}", output, report[i]);
                let mut report_removed = report.clone();
                report_removed.remove(i);
                safe = is_safe_report(report_removed, false);
            }
            break;
        }
    }
    if !safe {
        // println!("{}\t    {:?}", output, report);
    }
    safe
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
