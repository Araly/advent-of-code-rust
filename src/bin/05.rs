use core::panic;
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let (rules, updates) = parse_printing_instructions(input)?;
    let mut count = 0;
    for update in updates {
        count += check_update(update, &rules, false).unwrap_or(0);
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (rules, updates) = parse_printing_instructions(input)?;
    let mut count = 0;
    for update in updates {
        count += check_update(update, &rules, true).unwrap_or(0);
    }
    Some(count)
}

fn parse_printing_instructions(input: &str) -> Option<(HashMap<u32, Vec<u32>>, Vec<Vec<u32>>)> {
    let mut lines = input.lines();
    let mut rules = HashMap::new();
    let mut updates = vec![];
    let mut parsing_rules = true;
    loop {
        let line = lines.next();
        let line = match line {
            Some(v) => v,
            None => break,
        };
        if line == "" {
            parsing_rules = false;
            continue;
        }
        if parsing_rules {
            let mut line = line.split("|");
            let page1 = line.next()?.parse();
            let page1: u32 = match page1 {
                Ok(v) => v,
                Err(_) => return None,
            };
            let page2 = line.next()?.parse();
            let page2 = match page2 {
                Ok(v) => v,
                Err(_) => return None,
            };
            let rule = rules.entry(page2).or_insert(vec![]);
            rule.push(page1);
        } else {
            let line = line.split(",");
            let mut pages = vec![];
            for page in line {
                let page = page.parse();
                let page = match page {
                    Ok(v) => v,
                    Err(_) => return None,
                };
                pages.push(page);
            }
            updates.push(pages);
        }
    }
    Some((rules, updates))
}

fn check_update(update: Vec<u32>, rules: &HashMap<u32, Vec<u32>>, sort: bool) -> Option<u32> {
    println!("checking\t{:?}", update);
    let mut relevant_rules = HashSet::new();
    for page in &update {
        if let Some(_) = relevant_rules.get(&page) {
            if sort {
                return sort_update(update, rules);
            } else {
                return None;
            }
        }
        let rules = match rules.get(&page) {
            Some(v) => v,
            None => continue,
        };
        for rule in rules {
            relevant_rules.insert(rule);
        }
    }
    match sort {
        true => None,
        false => Some(update[update.len() / 2]),
    }
}

fn compare_page(page1: u32, page2: u32, rules: &HashMap<u32, Vec<u32>>) -> Ordering {
    let rule = match rules.get(&page2) {
        Some(v) => v,
        None => return Ordering::Equal,
    };
    for page in rule {
        if *page == page1 {
            return Ordering::Less;
        }
    }
    Ordering::Equal
}

fn sort_update(mut update: Vec<u32>, rules: &HashMap<u32, Vec<u32>>) -> Option<u32> {
    loop {
        let mut swapped = false;
        for i in 1..update.len() {
            match compare_page(update[i - 1], update[i], rules) {
                Ordering::Greater => panic!(
                    "Greater was found comparing {} and {}",
                    update[i - 1],
                    update[i]
                ),
                Ordering::Equal => continue,
                Ordering::Less => {
                    let swap = update[i];
                    update[i] = update[i - 1];
                    update[i - 1] = swap;
                    swapped = true;
                }
            }
        }
        if !swapped {
            break;
        }
        println!("sorted\t\t{:?}", update);
    }
    Some(update[update.len() / 2])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
