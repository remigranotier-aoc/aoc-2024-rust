use std::{collections::HashMap, thread::current};

use itertools::Itertools;

advent_of_code::solution!(6);

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn turn_right(dir: Direction) -> Direction {
    match dir {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}

fn is_getting_out(
    curr_i: usize,
    curr_j: usize,
    current_direction: &Direction,
    v_size: usize,
    h_size: usize,
) -> bool {
    match current_direction {
        Direction::Down => curr_i == v_size - 1,
        Direction::Left => curr_j == 0,
        Direction::Right => curr_j == h_size - 1,
        Direction::Up => curr_i == 0,
    }
}

fn is_wall_ahead(
    curr_i: usize,
    curr_j: usize,
    current_direction: &Direction,
    game_board: &[Vec<char>],
) -> bool {
    match current_direction {
        Direction::Down => game_board[curr_i + 1][curr_j] == '#',
        Direction::Up => game_board[curr_i - 1][curr_j] == '#',
        Direction::Left => game_board[curr_i][curr_j - 1] == '#',
        Direction::Right => game_board[curr_i][curr_j + 1] == '#',
    }
}

fn one_step_forward(curr_i: &mut usize, curr_j: &mut usize, current_direction: &Direction) {
    match current_direction {
        Direction::Down => {
            *curr_i += 1;
        }
        Direction::Up => {
            *curr_i -= 1;
        }
        Direction::Left => {
            *curr_j -= 1;
        }
        Direction::Right => {
            *curr_j += 1;
        }
    }
}

fn find_starting_position(game_board: &[Vec<char>]) -> (usize, usize) {
    for (i, element_i) in game_board.iter().enumerate() {
        for (j, element_j) in element_i.iter().enumerate() {
            if *element_j == '^' {
                return (i, j);
            }
        }
    }
    (0, 0)
}

fn is_loop_generated(start_i: usize, start_j: usize, game_board: &[Vec<char>]) -> bool {
    let v_size = game_board.len();
    let h_size = game_board[0].len();

    let mut curr_i = start_i;
    let mut curr_j = start_j;
    let mut current_direction = Direction::Up;
    let mut visited_states: HashMap<(usize, usize), Vec<Direction>> = HashMap::new();

    while !is_getting_out(curr_i, curr_j, &current_direction, v_size, h_size) {
        // Add current position to visited state
        if let Some(v) = visited_states.get_mut(&(curr_i, curr_j)) {
            if v.contains(&current_direction) {
                // Already visited this exact state
                return true;
            }
            v.push(current_direction);
        } else {
            visited_states.insert((curr_i, curr_j), vec![current_direction]);
        }

        if is_wall_ahead(curr_i, curr_j, &current_direction, game_board) {
            current_direction = turn_right(current_direction);
            continue;
        }

        one_step_forward(&mut curr_i, &mut curr_j, &current_direction);
    }

    false
}

pub fn part_one(input: &str) -> Option<usize> {
    let game_board = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let v_size = game_board.len();
    let h_size = game_board[0].len();

    let mut seen_positions = vec![vec!['.'; h_size]; v_size];

    // Initial state
    let (start_i, start_j) = find_starting_position(&game_board);
    let mut curr_i = start_i;
    let mut curr_j = start_j;
    let mut current_direction = Direction::Up;
    seen_positions[curr_i][curr_j] = 'X';

    // Loop
    while !is_getting_out(curr_i, curr_j, &current_direction, v_size, h_size) {
        if is_wall_ahead(curr_i, curr_j, &current_direction, &game_board) {
            current_direction = turn_right(current_direction);
            continue;
        }

        one_step_forward(&mut curr_i, &mut curr_j, &current_direction);
        seen_positions[curr_i][curr_j] = 'X';
    }

    // Count Xs
    Some(
        seen_positions
            .iter()
            .map(|line| line.iter().filter(|&c| *c == 'X').count())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut game_board = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let v_size = game_board.len();
    let h_size = game_board[0].len();

    let mut seen_positions: Vec<(usize, usize)> = vec![];

    // Initial state
    let (start_i, start_j) = find_starting_position(&game_board);
    let mut curr_i = start_i;
    let mut curr_j = start_j;
    let mut current_direction = Direction::Up;
    seen_positions.push((curr_i, curr_j));

    // Initial Loop
    while !is_getting_out(curr_i, curr_j, &current_direction, v_size, h_size) {
        if is_wall_ahead(curr_i, curr_j, &current_direction, &game_board) {
            current_direction = turn_right(current_direction);
            continue;
        }

        one_step_forward(&mut curr_i, &mut curr_j, &current_direction);
        if !seen_positions.contains(&(curr_i, curr_j)) {
            seen_positions.push((curr_i, curr_j));
        }
    }

    // Test Loops
    let mut looper_count = 0;

    for (o_i, o_j) in seen_positions {
        if o_i == start_i && o_j == start_j {
            continue;
        }

        game_board[o_i][o_j] = '#';

        // Test if there is a loop generated
        if is_loop_generated(start_i, start_j, &game_board) {
            looper_count += 1;
        }

        // Put back initial state
        game_board[o_i][o_j] = '.';
    }

    Some(looper_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
