use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<usize> {
    let mut stones: Vec<u64> = input
        .trim()
        .split(" ")
        .map(|n| n.parse::<u64>().unwrap())
        .collect();

    for _ in 0..25 {
        let mut new_stones: Vec<u64> = vec![];
        for stone in stones {
            if stone == 0 {
                new_stones.push(1);
                continue;
            }

            let s = stone.to_string();
            let l = s.len();
            if l % 2 == 0 {
                let first_half = &s[0..l / 2];
                let last_half = &s[l / 2..];
                new_stones.push(first_half.parse::<u64>().unwrap());
                new_stones.push(last_half.parse::<u64>().unwrap());
                continue;
            }

            new_stones.push(stone * 2024);
        }

        stones = new_stones;
    }

    Some(stones.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut stones: HashMap<u64, usize> = input
        .trim()
        .split(" ")
        .map(|n| n.parse::<u64>().unwrap())
        .counts();

    for _ in 0..75 {
        let mut new_stones: HashMap<u64, usize> = HashMap::new();
        for (stone, amount) in &stones {
            if *stone == 0 {
                *new_stones.entry(1).or_default() += amount;
                continue;
            }

            let s = stone.to_string();
            let l = s.len();
            if l % 2 == 0 {
                let first_half = &s[0..l / 2];
                let last_half = &s[l / 2..];
                *new_stones
                    .entry(first_half.parse::<u64>().unwrap())
                    .or_default() += amount;
                *new_stones
                    .entry(last_half.parse::<u64>().unwrap())
                    .or_default() += amount;
                continue;
            }

            *new_stones.entry(*stone * 2024).or_default() += amount;
        }

        stones = new_stones;
    }

    Some(stones.values().sum())
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
