#![allow(clippy::type_complexity)]
use std::collections::HashMap;

advent_of_code::solution!(20);

// const THRESHOLD: usize = 100;
const THRESHOLD: usize = 20;

fn neighbors(
    grid: &[Vec<char>],
    queue: &[(usize, usize)],
    current_node: (usize, usize),
) -> Vec<(usize, usize)> {
    let height = grid.len();
    let width = grid[0].len();
    let mut neighbors: Vec<(usize, usize)> = vec![];
    if grid[current_node.0][current_node.1] == '#' {
        return neighbors;
    }
    if current_node.0 < height - 1
        && queue.contains(&(current_node.0 + 1, current_node.1))
        && grid[current_node.0 + 1][current_node.1] != '#'
    {
        neighbors.push((current_node.0 + 1, current_node.1));
    }

    if current_node.0 > 0
        && queue.contains(&(current_node.0 - 1, current_node.1))
        && grid[current_node.0 - 1][current_node.1] != '#'
    {
        neighbors.push((current_node.0 - 1, current_node.1));
    }

    if current_node.1 < width - 1
        && queue.contains(&(current_node.0, current_node.1 + 1))
        && grid[current_node.0][current_node.1 + 1] != '#'
    {
        neighbors.push((current_node.0, current_node.1 + 1));
    }

    if current_node.1 > 0
        && queue.contains(&(current_node.0, current_node.1 - 1))
        && grid[current_node.0][current_node.1 - 1] != '#'
    {
        neighbors.push((current_node.0, current_node.1 - 1));
    }

    neighbors
}

fn dijkstra(
    grid: &[Vec<char>],
    source: (usize, usize),
    target: (usize, usize),
) -> (
    HashMap<(usize, usize), usize>,
    HashMap<(usize, usize), (usize, usize)>,
) {
    let mut dist: HashMap<(usize, usize), usize> = HashMap::new();
    let mut prev: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    let mut queue: Vec<(usize, usize)> = vec![];

    for (i, line) in grid.iter().enumerate() {
        for (j, _) in line.iter().enumerate() {
            dist.insert((i, j), usize::MAX);
            queue.push((i, j));
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
                prev.insert(neighbor, current_node);
            }
        }
    }

    (dist, prev)
}

fn two_distance(tile: (usize, usize), height: usize, width: usize) -> Vec<(usize, usize)> {
    let mut list = vec![];
    if tile.0 > 1 {
        list.push((tile.0 - 2, tile.1))
    }
    if tile.0 < height - 2 {
        list.push((tile.0 + 2, tile.1));
    }
    if tile.1 > 1 {
        list.push((tile.0, tile.1 - 2));
    }
    if tile.1 < width - 2 {
        list.push((tile.0, tile.1 + 2));
    }
    list
}

fn find_timesaves(
    optimal_path: &[(usize, usize)],
    height: usize,
    width: usize,
) -> HashMap<((usize, usize), (usize, usize)), usize> {
    let mut timesaves: HashMap<((usize, usize), (usize, usize)), usize> = HashMap::new();
    let mut distances: HashMap<(usize, usize), usize> = HashMap::new();
    for (i, tile) in optimal_path.iter().enumerate() {
        distances.insert(*tile, i);
        for target in two_distance(*tile, height, width) {
            if let Some(target_dist) = distances.get(&target) {
                if *target_dist < i - 2 {
                    timesaves.insert((target, *tile), i - *target_dist - 2);
                }
            }
        }
    }

    timesaves
}

fn distance(tile: (usize, usize), target: (usize, usize)) -> usize {
    let mut d: usize = 0;
    if tile.0 >= target.0 {
        d += tile.0 - target.0;
    } else {
        d += target.0 - tile.0;
    }
    if tile.1 >= target.1 {
        d += tile.1 - target.1;
    } else {
        d += target.1 - tile.1;
    }
    d
}

fn find_timesaves_p2(
    optimal_path: &[(usize, usize)],
) -> HashMap<((usize, usize), (usize, usize)), usize> {
    let mut timesaves: HashMap<((usize, usize), (usize, usize)), usize> = HashMap::new();
    let mut distances: HashMap<(usize, usize), usize> = HashMap::new();
    for (i, tile) in optimal_path.iter().enumerate() {
        distances.insert(*tile, i);
        for (target, dist) in &distances {
            if let Some(target_dist) = distances.get(target) {
                if *dist < i - distance(*tile, *target) && distance(*tile, *target) <= 20 {
                    timesaves.insert(
                        (*target, *tile),
                        i - *target_dist - distance(*tile, *target),
                    );
                }
            }
        }
    }

    timesaves
}

pub fn part_one(input: &str) -> Option<usize> {
    let maze = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let height: usize = maze.len();
    let width: usize = maze[0].len();

    let mut start_pos: (usize, usize) = (0, 0);
    let mut end_pos: (usize, usize) = (0, 0);

    for (i, line) in maze.iter().enumerate() {
        for (j, char) in line.iter().enumerate() {
            if *char == 'S' {
                start_pos = (i, j);
            }
            if *char == 'E' {
                end_pos = (i, j);
            }
        }
    }

    let (_, prev) = dijkstra(&maze, start_pos, end_pos);

    // Get optimal path
    let mut optimal_path: Vec<(usize, usize)> = vec![];
    let mut current_node = end_pos;
    optimal_path.push(current_node);
    while current_node != start_pos {
        current_node = prev[&current_node];
        optimal_path.push(current_node);
    }

    optimal_path.reverse();

    let timesaves: HashMap<((usize, usize), (usize, usize)), usize> =
        find_timesaves(&optimal_path, height, width);

    Some(timesaves.iter().filter(|(_, v)| **v >= THRESHOLD).count())
}

pub fn part_two(input: &str) -> Option<usize> {
    let maze = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut start_pos: (usize, usize) = (0, 0);
    let mut end_pos: (usize, usize) = (0, 0);

    for (i, line) in maze.iter().enumerate() {
        for (j, char) in line.iter().enumerate() {
            if *char == 'S' {
                start_pos = (i, j);
            }
            if *char == 'E' {
                end_pos = (i, j);
            }
        }
    }

    let (_, prev) = dijkstra(&maze, start_pos, end_pos);

    // Get optimal path
    let mut optimal_path: Vec<(usize, usize)> = vec![];
    let mut current_node = end_pos;
    optimal_path.push(current_node);
    while current_node != start_pos {
        current_node = prev[&current_node];
        optimal_path.push(current_node);
    }

    optimal_path.reverse();

    let timesaves: HashMap<((usize, usize), (usize, usize)), usize> =
        find_timesaves_p2(&optimal_path);

    Some(timesaves.iter().filter(|(_, v)| **v >= THRESHOLD).count())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1449));
    }
}
