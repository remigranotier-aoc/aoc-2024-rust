use itertools::Itertools;
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<usize> {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let v_size: i32 = grid.len().try_into().unwrap();
    let h_size: i32 = grid.first().unwrap().len().try_into().unwrap();

    let mut antenna_locations: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let mut antinode_locations: HashSet<(i32, i32)> = HashSet::new();

    for (i, line) in grid.iter().enumerate() {
        for (j, char) in line.iter().enumerate() {
            match char {
                '.' => continue,
                a if a.is_alphanumeric() => {
                    if let Some(v) = antenna_locations.get_mut(a) {
                        v.push((i, j));
                    } else {
                        antenna_locations.insert(*a, vec![(i, j)]);
                    }
                }
                _ => unreachable!(),
            }
        }
    }

    for (_, locations) in antenna_locations.iter() {
        for v in locations.iter().combinations(2) {
            let v0 = v[0];
            let v1 = v[1];
            let i0: i32 = v0.0.try_into().unwrap();
            let j0: i32 = v0.1.try_into().unwrap();
            let i1: i32 = v1.0.try_into().unwrap();
            let j1: i32 = v1.1.try_into().unwrap();

            let diff_i: i32 = i1 - i0;
            let diff_j: i32 = j1 - j0;

            if (i1 + diff_i) < v_size
                && (i1 + diff_i) >= 0
                && (j1 + diff_j) < h_size
                && (j1 + diff_j) >= 0
            {
                antinode_locations.insert((i1 + diff_i, j1 + diff_j));
            }

            if (i0 - diff_i) < v_size
                && (i0 - diff_i) >= 0
                && (j0 - diff_j) < h_size
                && (j0 - diff_j) >= 0
            {
                antinode_locations.insert((i0 - diff_i, j0 - diff_j));
            }
        }
    }

    Some(antinode_locations.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let v_size: i32 = grid.len().try_into().unwrap();
    let h_size: i32 = grid.first().unwrap().len().try_into().unwrap();

    let mut antenna_locations: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let mut antinode_locations: HashSet<(i32, i32)> = HashSet::new();

    for (i, line) in grid.iter().enumerate() {
        for (j, char) in line.iter().enumerate() {
            match char {
                '.' => continue,
                a if a.is_alphanumeric() => {
                    if let Some(v) = antenna_locations.get_mut(a) {
                        v.push((i, j));
                    } else {
                        antenna_locations.insert(*a, vec![(i, j)]);
                    }
                }
                _ => unreachable!(),
            }
        }
    }

    for (_, locations) in antenna_locations.iter() {
        for v in locations.iter().combinations(2) {
            let v0 = v[0];
            let v1 = v[1];
            let mut i0: i32 = v0.0.try_into().unwrap();
            let mut j0: i32 = v0.1.try_into().unwrap();
            let mut i1: i32 = v1.0.try_into().unwrap();
            let mut j1: i32 = v1.1.try_into().unwrap();
            antinode_locations.insert((i0, j0));
            antinode_locations.insert((i1, j1));

            let diff_i: i32 = i1 - i0;
            let diff_j: i32 = j1 - j0;

            while (i1 + diff_i) < v_size
                && (i1 + diff_i) >= 0
                && (j1 + diff_j) < h_size
                && (j1 + diff_j) >= 0
            {
                antinode_locations.insert((i1 + diff_i, j1 + diff_j));
                i1 += diff_i;
                j1 += diff_j;
            }

            while (i0 - diff_i) < v_size
                && (i0 - diff_i) >= 0
                && (j0 - diff_j) < h_size
                && (j0 - diff_j) >= 0
            {
                antinode_locations.insert((i0 - diff_i, j0 - diff_j));
                i0 -= diff_i;
                j0 -= diff_j;
            }
        }
    }

    Some(antinode_locations.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
