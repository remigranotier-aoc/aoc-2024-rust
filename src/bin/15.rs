use itertools::Itertools;

advent_of_code::solution!(15);

enum Directions {
    Up,
    Down,
    Left,
    Right,
}

fn move_robot_p1(grid: &mut [Vec<char>], curr_x: usize, curr_y: usize, dir: Directions) -> bool {
    let curr_char = grid[curr_x][curr_y];
    match dir {
        Directions::Down => match grid[curr_x + 1][curr_y] {
            '.' => {
                // Simple robot move
                grid[curr_x][curr_y] = '.';
                grid[curr_x + 1][curr_y] = curr_char;
                true
            }
            '#' => {
                // Do nothing
                false
            }
            'O' => {
                if move_robot_p1(grid, curr_x + 1, curr_y, dir) {
                    grid[curr_x][curr_y] = '.';
                    grid[curr_x + 1][curr_y] = curr_char;
                    true
                } else {
                    false
                }
            }
            _ => unreachable!(),
        },
        Directions::Up => match grid[curr_x - 1][curr_y] {
            '.' => {
                // Simple robot move
                grid[curr_x][curr_y] = '.';
                grid[curr_x - 1][curr_y] = curr_char;
                true
            }
            '#' => {
                // Do nothing
                false
            }
            'O' => {
                if move_robot_p1(grid, curr_x - 1, curr_y, dir) {
                    grid[curr_x][curr_y] = '.';
                    grid[curr_x - 1][curr_y] = curr_char;
                    true
                } else {
                    false
                }
            }
            _ => unreachable!(),
        },
        Directions::Right => match grid[curr_x][curr_y + 1] {
            '.' => {
                // Simple robot move
                grid[curr_x][curr_y] = '.';
                grid[curr_x][curr_y + 1] = curr_char;
                true
            }
            '#' => {
                // Do nothing
                false
            }
            'O' => {
                if move_robot_p1(grid, curr_x, curr_y + 1, dir) {
                    grid[curr_x][curr_y] = '.';
                    grid[curr_x][curr_y + 1] = curr_char;
                    true
                } else {
                    false
                }
            }
            _ => unreachable!(),
        },
        Directions::Left => match grid[curr_x][curr_y - 1] {
            '.' => {
                // Simple robot move
                grid[curr_x][curr_y] = '.';
                grid[curr_x][curr_y - 1] = curr_char;
                true
            }
            '#' => {
                // Do nothing
                false
            }
            'O' => {
                if move_robot_p1(grid, curr_x, curr_y - 1, dir) {
                    grid[curr_x][curr_y] = '.';
                    grid[curr_x][curr_y - 1] = curr_char;
                    true
                } else {
                    false
                }
            }
            _ => unreachable!(),
        },
    }
}

fn can_move_robot_p2(
    grid: &mut [Vec<char>],
    curr_x: usize,
    curr_y: usize,
    dir: &Directions,
) -> bool {
    match dir {
        Directions::Down => match grid[curr_x + 1][curr_y] {
            '.' => true,
            '#' => {
                // Do nothing
                false
            }
            '[' => {
                can_move_robot_p2(grid, curr_x + 1, curr_y, dir)
                    && can_move_robot_p2(grid, curr_x + 1, curr_y + 1, dir)
            }
            ']' => {
                can_move_robot_p2(grid, curr_x + 1, curr_y, dir)
                    && can_move_robot_p2(grid, curr_x + 1, curr_y - 1, dir)
            }
            _ => unreachable!(),
        },
        Directions::Up => match grid[curr_x - 1][curr_y] {
            '.' => true,
            '#' => false,
            '[' => {
                can_move_robot_p2(grid, curr_x - 1, curr_y, dir)
                    && can_move_robot_p2(grid, curr_x - 1, curr_y + 1, dir)
            }
            ']' => {
                can_move_robot_p2(grid, curr_x - 1, curr_y, dir)
                    && can_move_robot_p2(grid, curr_x - 1, curr_y - 1, dir)
            }
            _ => unreachable!(),
        },
        Directions::Right => match grid[curr_x][curr_y + 1] {
            '.' => {
                // Simple move
                true
            }
            '#' => false,
            '[' => can_move_robot_p2(grid, curr_x, curr_y + 2, dir),
            _ => unreachable!(),
        },
        Directions::Left => match grid[curr_x][curr_y - 1] {
            '.' => {
                // Simple robot move
                true
            }
            '#' => {
                // Do nothing
                false
            }
            ']' => can_move_robot_p2(grid, curr_x, curr_y - 2, dir),
            _ => unreachable!(),
        },
    }
}

fn move_robot_p2(grid: &mut [Vec<char>], curr_x: usize, curr_y: usize, dir: &Directions) {
    let curr_char = grid[curr_x][curr_y];
    match dir {
        Directions::Down => match grid[curr_x + 1][curr_y] {
            '.' => {
                grid[curr_x][curr_y] = '.';
                grid[curr_x + 1][curr_y] = curr_char
            }
            '#' => {
                // Do nothing
            }
            '[' => {
                move_robot_p2(grid, curr_x + 1, curr_y, dir);
                move_robot_p2(grid, curr_x + 1, curr_y + 1, dir);
                grid[curr_x][curr_y] = '.';
                grid[curr_x + 1][curr_y] = curr_char
            }
            ']' => {
                move_robot_p2(grid, curr_x + 1, curr_y, dir);
                move_robot_p2(grid, curr_x + 1, curr_y - 1, dir);
                grid[curr_x][curr_y] = '.';
                grid[curr_x + 1][curr_y] = curr_char
            }
            _ => unreachable!(),
        },
        Directions::Up => match grid[curr_x - 1][curr_y] {
            '.' => {
                grid[curr_x][curr_y] = '.';
                grid[curr_x - 1][curr_y] = curr_char;
            }
            '#' => {
                // Do nothing
            }
            '[' => {
                move_robot_p2(grid, curr_x - 1, curr_y, dir);
                move_robot_p2(grid, curr_x - 1, curr_y + 1, dir);
                grid[curr_x][curr_y] = '.';
                grid[curr_x - 1][curr_y] = curr_char;
            }
            ']' => {
                move_robot_p2(grid, curr_x - 1, curr_y, dir);
                move_robot_p2(grid, curr_x - 1, curr_y - 1, dir);
                grid[curr_x][curr_y] = '.';
                grid[curr_x - 1][curr_y] = curr_char;
            }
            _ => unreachable!(),
        },
        Directions::Right => match grid[curr_x][curr_y + 1] {
            '.' => {
                // Simple move
                grid[curr_x][curr_y] = '.';
                grid[curr_x][curr_y + 1] = curr_char;
            }
            '#' => {
                // Do nothing
            }
            '[' => {
                move_robot_p2(grid, curr_x, curr_y + 2, dir);
                move_robot_p2(grid, curr_x, curr_y + 1, dir);
                grid[curr_x][curr_y] = '.';
                grid[curr_x][curr_y + 1] = curr_char;
            }
            _ => unreachable!(),
        },
        Directions::Left => match grid[curr_x][curr_y - 1] {
            '.' => {
                // Simple robot move
                grid[curr_x][curr_y] = '.';
                grid[curr_x][curr_y - 1] = curr_char;
            }
            '#' => {
                // Do nothing
            }
            ']' => {
                move_robot_p2(grid, curr_x, curr_y - 2, dir);
                move_robot_p2(grid, curr_x, curr_y - 1, dir);
                grid[curr_x][curr_y] = '.';
                grid[curr_x][curr_y - 1] = curr_char;
            }
            _ => unreachable!(),
        },
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let (grid, moves) = input.split("\n\n").next_tuple().unwrap();
    let mut grid: Vec<Vec<char>> = grid
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let (mut curr_x, line) = grid
        .iter()
        .find_position(|line| line.contains(&'@'))
        .unwrap();
    let mut curr_y = line.iter().position(|c| *c == '@').unwrap();

    let moves = moves.split("\n").join("");
    for m in moves.chars() {
        // println!("\n\n=== Moving {m} ===");
        match m {
            '^' => {
                if move_robot_p1(&mut grid, curr_x, curr_y, Directions::Up) {
                    curr_x -= 1;
                }
            }
            '>' => {
                if move_robot_p1(&mut grid, curr_x, curr_y, Directions::Right) {
                    curr_y += 1;
                }
            }
            '<' => {
                if move_robot_p1(&mut grid, curr_x, curr_y, Directions::Left) {
                    curr_y -= 1;
                }
            }
            'v' => {
                if move_robot_p1(&mut grid, curr_x, curr_y, Directions::Down) {
                    curr_x += 1;
                }
            }
            _ => unreachable!(),
        }
        // println!(
        //     "{}",
        //     grid.iter().map(|line| line.iter().join("")).join("\n")
        // );
    }

    Some(
        grid.iter()
            .enumerate()
            .map(|(i, line)| {
                line.iter()
                    .enumerate()
                    .map(|(j, char)| if *char == 'O' { 100 * i + j } else { 0 })
                    .sum::<usize>()
            })
            .sum::<usize>(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let (parsed_grid, moves) = input.split("\n\n").next_tuple().unwrap();
    let mut grid: Vec<Vec<char>> = vec![];
    for line in parsed_grid.lines() {
        let mut new_line: Vec<char> = vec![];
        for char in line.chars() {
            match char {
                '#' => {
                    new_line.push('#');
                    new_line.push('#');
                }
                'O' => {
                    new_line.push('[');
                    new_line.push(']');
                }
                '.' => {
                    new_line.push('.');
                    new_line.push('.');
                }
                '@' => {
                    new_line.push('@');
                    new_line.push('.');
                }
                _ => unreachable!(),
            }
        }
        grid.push(new_line);
    }

    // println!(
    //     "INITIAL STATE\n\n{}",
    //     grid.iter().map(|line| line.iter().join("")).join("\n")
    // );

    let (mut curr_x, line) = grid
        .iter()
        .find_position(|line| line.contains(&'@'))
        .unwrap();
    let mut curr_y = line.iter().position(|c| *c == '@').unwrap();

    let moves = moves.split("\n").join("");
    for m in moves.chars() {
        // println!("\n\n=== Moving {m} ===");
        match m {
            '^' => {
                if can_move_robot_p2(&mut grid, curr_x, curr_y, &Directions::Up) {
                    move_robot_p2(&mut grid, curr_x, curr_y, &Directions::Up);
                    curr_x -= 1;
                }
            }
            '>' => {
                if can_move_robot_p2(&mut grid, curr_x, curr_y, &Directions::Right) {
                    move_robot_p2(&mut grid, curr_x, curr_y, &Directions::Right);
                    curr_y += 1;
                }
            }
            '<' => {
                if can_move_robot_p2(&mut grid, curr_x, curr_y, &Directions::Left) {
                    move_robot_p2(&mut grid, curr_x, curr_y, &Directions::Left);
                    curr_y -= 1;
                }
            }
            'v' => {
                if can_move_robot_p2(&mut grid, curr_x, curr_y, &Directions::Down) {
                    move_robot_p2(&mut grid, curr_x, curr_y, &Directions::Down);
                    curr_x += 1;
                }
            }
            _ => unreachable!(),
        }
        // println!(
        //     "{}",
        //     grid.iter().map(|line| line.iter().join("")).join("\n")
        // );
    }

    Some(
        grid.iter()
            .enumerate()
            .map(|(i, line)| {
                line.iter()
                    .enumerate()
                    .map(|(j, char)| if *char == '[' { 100 * i + j } else { 0 })
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
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9021));
    }
}
