use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut l1: Vec<u32> = Vec::new();
    let mut l2: Vec<u32> = Vec::new();

    input.lines().for_each(|line| {
        if line.is_empty() {
            return;
        }
        let mut locations = line.split("   ");
        l1.push(locations.next().unwrap().parse::<u32>().unwrap());
        l2.push(locations.next().unwrap().parse::<u32>().unwrap());
    });

    l1.sort();
    l2.sort();

    let total_distance: u32 = std::iter::zip(l1, l2).map(|(d1, d2)| d1.abs_diff(d2)).sum();

    Some(total_distance)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut l1: Vec<u32> = Vec::new();
    let mut l2: HashMap<u32, u32> = HashMap::new();

    input.lines().for_each(|line| {
        if line.is_empty() {
            return;
        }
        let mut locations = line.split("   ");
        l1.push(locations.next().unwrap().parse::<u32>().unwrap());
        let second_number = locations.next().unwrap().parse::<u32>().unwrap();
        if let Some(val) = l2.get(&second_number) {
            l2.insert(second_number, val + 1);
        } else {
            l2.insert(second_number, 1);
        }
    });

    let similarity_score: u32 = l1.into_iter().map(|n| n * l2.get(&n).unwrap_or(&0)).sum();

    Some(similarity_score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
