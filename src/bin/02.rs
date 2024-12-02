advent_of_code::solution!(2);

fn is_instantly_valid(levels: &[i32]) -> i32 {
    let first_diff = levels[1] - levels[0];
    if first_diff == 0 {
        0
    } else if first_diff > 0 {
        // Increasing
        if levels
            .windows(2)
            .all(|w| w[1] - w[0] < 4 && w[1] - w[0] > 0)
        {
            1
        } else {
            0
        }
    } else {
        // Decreasing
        if levels
            .windows(2)
            .all(|w| w[0] - w[1] < 4 && w[0] - w[1] > 0)
        {
            1
        } else {
            0
        }
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let number_of_safe_reports = input
        .lines()
        .map(|line| {
            let levels = line
                .split(" ")
                .map(|level| level.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            if levels.len() <= 1 {
                return 0;
            }
            is_instantly_valid(&levels)
        })
        .sum();

    Some(number_of_safe_reports)
}

pub fn part_two(input: &str) -> Option<i32> {
    let number_of_safe_reports = input
        .lines()
        .map(|line| {
            let levels = line
                .split(" ")
                .map(|level| level.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            if levels.len() <= 1 {
                return 0;
            }
            if is_instantly_valid(&levels) == 1 {
                return 1;
            } else {
                for i in 0..levels.len() {
                    let i_removed = &mut levels.to_vec();
                    i_removed.remove(i);
                    if is_instantly_valid(i_removed) == 1 {
                        return 1;
                    }
                }
                return 0;
            }
        })
        .sum();

    Some(number_of_safe_reports)
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
