use std::collections::HashMap;

use itertools::Itertools;
use regex::Regex;

advent_of_code::solution!(19);

pub fn part_one(input: &str) -> Option<usize> {
    let (available_towels_part, wished_towels_part) = input.split("\n\n").next_tuple().unwrap();

    let available_towels = available_towels_part.split(", ").collect::<Vec<_>>();
    let wished_towels = wished_towels_part.lines().collect::<Vec<_>>();

    let towels_choice = format!("^({})+$", available_towels.iter().join("|"));
    let towels_regex = Regex::new(&towels_choice).unwrap();
    Some(
        wished_towels
            .iter()
            .filter(|towel| towels_regex.is_match(towel))
            .count(),
    )
}

fn possible_patterns<'a>(
    shortcut: &mut HashMap<&'a str, usize>,
    available_towels: &[&str],
    wished_towel: &'a str,
) -> usize {
    // Early return
    if let Some(c) = shortcut.get(wished_towel) {
        return *c;
    }

    if wished_towel.is_empty() {
        return 1;
    }

    let mut total_patterns = 0;

    for available_towel in available_towels {
        if wished_towel.ends_with(available_towel) {
            let future_towel = &wished_towel[0..wished_towel.len() - available_towel.len()];
            total_patterns += possible_patterns(shortcut, available_towels, future_towel);
        }
    }

    shortcut.insert(wished_towel, total_patterns);
    total_patterns
}

pub fn part_two(input: &str) -> Option<usize> {
    let (available_towels_part, wished_towels_part) = input.split("\n\n").next_tuple().unwrap();

    let available_towels = available_towels_part.split(", ").collect::<Vec<_>>();
    let wished_towels = wished_towels_part.lines().collect::<Vec<_>>();

    let mut shortcut: HashMap<&str, usize> = HashMap::new();

    Some(
        wished_towels
            .iter()
            .map(|towel| possible_patterns(&mut shortcut, &available_towels, towel))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
