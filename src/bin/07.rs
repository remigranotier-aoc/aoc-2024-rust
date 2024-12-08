use std::collections::HashSet;

advent_of_code::solution!(7);

fn is_operation_possible(line: &str, part2: bool) -> Option<u64> {
    let mut line_parser = line.split(": ");
    let target = line_parser.next().unwrap().parse::<u64>().unwrap();
    let list_of_numbers = line_parser
        .next()
        .unwrap()
        .split(" ")
        .map(|n| n.parse::<u64>().unwrap());

    let mut exploration: Vec<HashSet<u64>> = vec![];
    for number in list_of_numbers {
        if let Some(set) = exploration.last() {
            let mut current_set = HashSet::new();
            for last_number in set {
                if last_number + number <= target {
                    current_set.insert(last_number + number);
                }
                if last_number * number <= target {
                    current_set.insert(last_number * number);
                }
                if part2 {
                    let mut concat = last_number.to_string();
                    concat.push_str(&number.to_string());
                    let number_to_add = concat.parse::<u64>().unwrap();
                    if number_to_add <= target {
                        current_set.insert(number_to_add);
                    }
                }
            }
            exploration.push(current_set);
        } else {
            let mut single_set = HashSet::new();
            single_set.insert(number);
            exploration.push(single_set);
        }
    }

    if exploration.last().unwrap().contains(&target) {
        Some(target)
    } else {
        None
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .filter_map(|line| is_operation_possible(line, false))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .filter_map(|line| is_operation_possible(line, true))
            .sum(),
    )
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
