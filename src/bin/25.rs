use itertools::Itertools;

advent_of_code::solution!(25);

pub fn part_one(input: &str) -> Option<usize> {
    let mut locks: Vec<[usize; 5]> = vec![];
    let mut keys: Vec<[usize; 5]> = vec![];

    for device in input.split("\n\n") {
        let is_lock = device.starts_with("#####");
        let mut new_vec = [0; 5];
        for (i, line) in device.lines().filter(|l| !l.is_empty()).enumerate() {
            for (j, char) in line.chars().enumerate() {
                if is_lock && char == '#' {
                    new_vec[j] = i;
                }
                if !is_lock && char == '#' && (new_vec[j] < 6 - i || new_vec[j] == 0) {
                    new_vec[j] = 6 - i
                }
            }
        }
        if is_lock {
            locks.push(new_vec);
        } else {
            keys.push(new_vec);
        }
    }

    Some(
        locks
            .iter()
            .cartesian_product(keys)
            .filter(|(&lock, key)| lock.iter().zip(key).all(|(l, k)| l + k <= 5))
            .count(),
    )
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
