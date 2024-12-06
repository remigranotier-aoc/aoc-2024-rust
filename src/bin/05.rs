advent_of_code::solution!(5);
use std::{cmp::Ordering, collections::HashMap};

use itertools::Itertools;

fn parse_first_section(first_section: &str) -> HashMap<&str, Vec<&str>> {
    let mut rules: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in first_section.lines().filter(|l| !l.is_empty()) {
        let (before, after) = line.split('|').next_tuple().unwrap();
        if let Some(v) = rules.get_mut(before) {
            v.push(after);
        } else {
            rules.insert(before, vec![after]);
        }
    }

    rules
}

fn parse_second_section(second_section: &str) -> Vec<Vec<&str>> {
    second_section
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.split(',').collect::<Vec<&str>>())
        .collect::<Vec<_>>()
}

fn custom_sort(a: &str, b: &str, rules: &HashMap<&str, Vec<&str>>) -> Ordering {
    if let Some(rule_a) = rules.get(a) {
        if rule_a.contains(&b) {
            return Ordering::Less;
        }
    }

    if let Some(rule_b) = rules.get(b) {
        if rule_b.contains(&a) {
            return Ordering::Greater;
        }
    }

    Ordering::Equal
}

pub fn part_one(input: &str) -> Option<u32> {
    let (first_section, second_section) = input.split("\n\n").next_tuple().unwrap();

    let rules = parse_first_section(first_section);

    let updates = parse_second_section(second_section);

    Some(
        updates
            .iter()
            .filter(|update| {
                update
                    .windows(2)
                    .all(|w| custom_sort(w[0], w[1], &rules).is_lt())
            })
            .map(|update| update[update.len() / 2].parse::<u32>().unwrap()) // take middle element
            .sum(), // sum all these middle elements
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (first_section, second_section) = input.split("\n\n").next_tuple().unwrap();

    let rules = parse_first_section(first_section);

    let updates = parse_second_section(second_section);

    Some(
        updates
            .iter()
            .filter(|update| {
                !update
                    .windows(2)
                    .all(|w| custom_sort(w[0], w[1], &rules).is_lt())
            })
            .map(|update| {
                let sorted_update = &mut update.to_vec();
                sorted_update.sort_by(|a, b| custom_sort(a, b, &rules));
                sorted_update[sorted_update.len() / 2]
                    .parse::<u32>()
                    .unwrap()
            })
            .sum(),
    )
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
