advent_of_code::solution!(3);
use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let mul_op = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let total = mul_op
        .captures_iter(input)
        .map(|c| {
            let numbers = c
                .extract::<2>()
                .1
                .into_iter()
                .map(|substr| substr.parse::<u32>().unwrap())
                .collect::<Vec<_>>();
            numbers[0] * numbers[1]
        })
        .sum();
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let ignored_blocks = Regex::new(r"don't\(\)[\S\s]*?do\(\)") // .*? matches shortest possible
        .unwrap()
        .replace_all(input, " ")
        .to_string();
    let ignore_end = Regex::new(r"don't\(\)[\S\s]*")
        .unwrap()
        .replace(&ignored_blocks, " ")
        .to_string();
    part_one(&ignore_end)
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
