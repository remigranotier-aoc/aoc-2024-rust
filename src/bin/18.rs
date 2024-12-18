use std::{cmp::min, collections::HashMap, usize};

use itertools::Itertools;

advent_of_code::solution!(18);

// const GRID_SIZE: usize = 7;
// const MEMORY_CORRUPTED: usize = 12;

const GRID_SIZE: usize = 71;
const MEMORY_CORRUPTED: usize = 1024;

fn neighbors(
    grid: &[[&str; GRID_SIZE]; GRID_SIZE],
    queue: &[(usize, usize)],
    current_node: (usize, usize),
) -> Vec<(usize, usize)> {
    let mut neighbors: Vec<(usize, usize)> = vec![];
    if grid[current_node.1][current_node.0] == "#" {
        return neighbors;
    }
    if current_node.0 < GRID_SIZE - 1
        && queue.contains(&(current_node.0 + 1, current_node.1))
        && grid[current_node.1][current_node.0 + 1] != "#"
    {
        neighbors.push((current_node.0 + 1, current_node.1));
    }

    if current_node.0 > 0
        && queue.contains(&(current_node.0 - 1, current_node.1))
        && grid[current_node.1][current_node.0 - 1] != "#"
    {
        neighbors.push((current_node.0 - 1, current_node.1));
    }

    if current_node.1 < GRID_SIZE - 1
        && queue.contains(&(current_node.0, current_node.1 + 1))
        && grid[current_node.1 + 1][current_node.0] != "#"
    {
        neighbors.push((current_node.0, current_node.1 + 1));
    }

    if current_node.1 > 0
        && queue.contains(&(current_node.0, current_node.1 - 1))
        && grid[current_node.1 - 1][current_node.0] != "#"
    {
        neighbors.push((current_node.0, current_node.1 - 1));
    }

    neighbors
}

fn dijkstra(
    grid: &[[&str; GRID_SIZE]; GRID_SIZE],
    source: (usize, usize),
    target: (usize, usize),
) -> usize {
    let mut dist: HashMap<(usize, usize), usize> = HashMap::new();
    let mut queue: Vec<(usize, usize)> = vec![];

    for (i, line) in grid.iter().enumerate() {
        for (j, _) in line.iter().enumerate() {
            dist.insert((j, i), usize::MAX);
            queue.push((j, i));
        }
    }
    dist.insert(source, 0);

    while !queue.is_empty() {
        let current_node = *queue
            .iter()
            .min_by_key(|node| dist.get(node).unwrap())
            .unwrap();
        // println!(
        //     "Now watching node {current_node:?} with dist {}",
        //     dist.get(&current_node).unwrap()
        // );
        if current_node == target || dist[&current_node] == usize::MAX {
            break;
        }
        queue.remove(queue.iter().position(|x| *x == current_node).unwrap());
        for neighbor in neighbors(grid, &queue, current_node) {
            let possible_distance = dist.get(&current_node).unwrap() + 1;
            if possible_distance < *dist.get(&neighbor).unwrap() {
                dist.insert(neighbor, possible_distance);
            }
        }
    }

    return *dist.get(&target).unwrap();
}

pub fn part_one(input: &str) -> Option<usize> {
    let falling_bytes: Vec<(usize, usize)> = input
        .lines()
        .map(|line| {
            line.split(",")
                .map(|n| n.parse::<usize>().unwrap())
                .next_tuple()
                .unwrap()
        })
        .collect::<Vec<_>>();

    let mut grid: [[&str; GRID_SIZE]; GRID_SIZE] = [["."; GRID_SIZE]; GRID_SIZE];

    for (x, y) in falling_bytes.iter().take(MEMORY_CORRUPTED) {
        grid[*y][*x] = "#";
    }

    Some(dijkstra(&grid, (0, 0), (GRID_SIZE - 1, GRID_SIZE - 1)))
}

pub fn part_two(input: &str) -> Option<String> {
    let falling_bytes: Vec<(usize, usize)> = input
        .lines()
        .map(|line| {
            line.split(",")
                .map(|n| n.parse::<usize>().unwrap())
                .next_tuple()
                .unwrap()
        })
        .collect::<Vec<_>>();

    let mut grid: [[&str; GRID_SIZE]; GRID_SIZE] = [["."; GRID_SIZE]; GRID_SIZE];

    let mut bottom_side = MEMORY_CORRUPTED;
    let mut top_side = falling_bytes.len();

    while bottom_side < top_side - 1 {
        grid = [["."; GRID_SIZE]; GRID_SIZE];
        let middle = bottom_side + (top_side - bottom_side) / 2;
        for (x, y) in falling_bytes.iter().take(middle) {
            grid[*y][*x] = "#";
        }
        if dijkstra(&grid, (0, 0), (GRID_SIZE - 1, GRID_SIZE - 1)) < usize::MAX {
            bottom_side = middle;
        } else {
            top_side = middle;
        }
    }

    Some(format!(
        "{},{}",
        falling_bytes[bottom_side].0, falling_bytes[bottom_side].1
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("6,1".to_owned()));
    }
}
