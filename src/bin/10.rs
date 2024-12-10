use std::collections::HashSet;

advent_of_code::solution!(10);

fn peak_counter(
    map: &[Vec<usize>],
    curr_i: usize,
    curr_j: usize,
    set_of_ends: &mut HashSet<(usize, usize)>,
) {
    if map[curr_i][curr_j] == 9 {
        set_of_ends.insert((curr_i, curr_j));
        return;
    }

    let v_size: usize = map.len();
    let h_size: usize = map.first().unwrap().len();
    let curr_val = map[curr_i][curr_j];

    if curr_i < v_size - 1 && map[curr_i + 1][curr_j] == curr_val + 1 {
        peak_counter(map, curr_i + 1, curr_j, set_of_ends);
    }

    if curr_i > 0 && map[curr_i - 1][curr_j] == curr_val + 1 {
        peak_counter(map, curr_i - 1, curr_j, set_of_ends);
    }

    if curr_j < h_size - 1 && map[curr_i][curr_j + 1] == curr_val + 1 {
        peak_counter(map, curr_i, curr_j + 1, set_of_ends);
    }

    if curr_j > 0 && map[curr_i][curr_j - 1] == curr_val + 1 {
        peak_counter(map, curr_i, curr_j - 1, set_of_ends);
    }
}

fn path_counter(map: &[Vec<usize>], curr_i: usize, curr_j: usize) -> usize {
    if map[curr_i][curr_j] == 9 {
        return 1;
    }

    let v_size: usize = map.len();
    let h_size: usize = map.first().unwrap().len();
    let curr_val = map[curr_i][curr_j];
    let mut total_paths = 0;

    if curr_i < v_size - 1 && map[curr_i + 1][curr_j] == curr_val + 1 {
        total_paths += path_counter(map, curr_i + 1, curr_j);
    }

    if curr_i > 0 && map[curr_i - 1][curr_j] == curr_val + 1 {
        total_paths += path_counter(map, curr_i - 1, curr_j);
    }

    if curr_j < h_size - 1 && map[curr_i][curr_j + 1] == curr_val + 1 {
        total_paths += path_counter(map, curr_i, curr_j + 1);
    }

    if curr_j > 0 && map[curr_i][curr_j - 1] == curr_val + 1 {
        total_paths += path_counter(map, curr_i, curr_j - 1);
    }

    total_paths
}

pub fn part_one(input: &str) -> Option<usize> {
    let map = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    Some(
        map.iter()
            .enumerate()
            .map(|(i, line)| {
                line.iter()
                    .enumerate()
                    .map(|(j, val)| {
                        if *val == 0 {
                            let mut set_of_accessible_peaks: HashSet<(usize, usize)> =
                                HashSet::new();
                            peak_counter(&map, i, j, &mut set_of_accessible_peaks);
                            set_of_accessible_peaks.len()
                        } else {
                            0
                        }
                    })
                    .sum::<usize>()
            })
            .sum::<usize>(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let map = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    Some(
        map.iter()
            .enumerate()
            .map(|(i, line)| {
                line.iter()
                    .enumerate()
                    .map(|(j, val)| {
                        if *val == 0 {
                            path_counter(&map, i, j)
                        } else {
                            0
                        }
                    })
                    .sum::<usize>()
            })
            .sum::<usize>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
