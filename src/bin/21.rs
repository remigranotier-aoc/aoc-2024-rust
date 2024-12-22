advent_of_code::solution!(21);

fn get_keypad_char(i: isize, j: isize) -> char {
    match (i, j) {
        (3, 1) => '0',
        (2, 0) => '1',
        (2, 1) => '2',
        (2, 2) => '3',
        (1, 0) => '4',
        (1, 1) => '5',
        (1, 2) => '6',
        (0, 0) => '7',
        (0, 1) => '8',
        (0, 2) => '9',
        (3, 2) => 'A',
        _ => unreachable!(),
    }
}

fn get_keypad_loc(c: char) -> (isize, isize) {
    match c {
        '0' => (3, 1),
        '1' => (2, 0),
        '2' => (2, 1),
        '3' => (2, 2),
        '4' => (1, 0),
        '5' => (1, 1),
        '6' => (1, 2),
        '7' => (0, 0),
        '8' => (0, 1),
        '9' => (0, 2),
        'A' => (3, 2),
        _ => unreachable!(),
    }
}

fn get_dpad_loc(c: char) -> (isize, isize) {
    match c {
        '^' => (0, 1),
        '<' => (1, 0),
        'v' => (1, 1),
        '>' => (1, 2),
        'A' => (0, 2),
        _ => unreachable!(),
    }
}

fn get_dpad_char(i: isize, j: isize) -> char {
    match (i, j) {
        (0, 1) => '^',
        (1, 0) => '<',
        (1, 1) => 'v',
        (1, 2) => '>',
        (0, 2) => 'A',
        _ => unreachable!(),
    }
}
// fn get_keypad_moves(source: char, target: char) -> String {
//     let mut moves: String = String::new();
//     let (mut i_source, j_source) = get_keypad_loc(source);
//     let (i_target, j_target) = get_keypad_loc(target);
//     if i_source == 3 && j_target == 0 {
//         moves.push('^');
//         i_source -= 1;
//         for _ in 0..((i_source - i_target).abs()) {
//             moves.push(if i_source > i_target { '^' } else { 'v' });
//         }
//         for _ in 0..((j_source - j_target).abs()) {
//             moves.push(if j_source > j_target { '<' } else { '>' });
//         }
//         return moves;
//     }
//     for _ in 0..((j_source - j_target).abs()) {
//         moves.push(if j_source > j_target { '<' } else { '>' });
//     }
//     for _ in 0..((i_source - i_target).abs()) {
//         moves.push(if i_source > i_target { '^' } else { 'v' });
//     }
//     moves
// }

fn get_dpad_moves(source: char, target: char) -> String {
    let mut moves: String = String::new();
    let (mut i_source, j_source) = get_dpad_loc(source);
    let (i_target, j_target) = get_dpad_loc(target);
    if i_source == 0 && j_target == 0 {
        moves.push('v');
        i_source += 1;
        for _ in 0..((i_source - i_target).abs()) {
            moves.push(if i_source > i_target { '^' } else { 'v' });
        }
        for _ in 0..((j_source - j_target).abs()) {
            moves.push(if j_source > j_target { '<' } else { '>' });
        }
        return moves;
    }
    for _ in 0..((j_source - j_target).abs()) {
        moves.push(if j_source > j_target { '<' } else { '>' });
    }
    for _ in 0..((i_source - i_target).abs()) {
        moves.push(if i_source > i_target { '^' } else { 'v' });
    }
    moves
}

fn get_existing_keypad_moves(source: char, target: char) -> Vec<String> {
    let mut existing_moves: Vec<String> = vec![];
    let (i_source, j_source) = get_keypad_loc(source);
    let (i_target, j_target) = get_keypad_loc(target);
    if source == target {
        existing_moves.push("".to_owned());
        return existing_moves;
    }

    #[allow(clippy::comparison_chain)]
    if i_target < i_source {
        let new_source = get_keypad_char(i_source - 1, j_source);
        existing_moves.append(
            &mut get_existing_keypad_moves(new_source, target)
                .iter()
                .map(|m| {
                    let mut start = "^".to_owned();
                    start.push_str(m);
                    start
                })
                .collect::<Vec<_>>(),
        );
    } else if i_target > i_source && !(i_target == 3 && j_source == 0) {
        let new_source = get_keypad_char(i_source + 1, j_source);
        existing_moves.append(
            &mut get_existing_keypad_moves(new_source, target)
                .iter()
                .map(|m| {
                    let mut start = "v".to_owned();
                    start.push_str(m);
                    start
                })
                .collect::<Vec<_>>(),
        );
    }

    if j_target < j_source && !(j_target == 0 && i_source == 3) {
        let new_source = get_keypad_char(i_source, j_source - 1);
        existing_moves.append(
            &mut get_existing_keypad_moves(new_source, target)
                .iter()
                .map(|m| {
                    let mut start = "<".to_owned();
                    start.push_str(m);
                    start
                })
                .collect::<Vec<_>>(),
        );
    } else if j_target > j_source {
        let new_source = get_keypad_char(i_source, j_source + 1);
        existing_moves.append(
            &mut get_existing_keypad_moves(new_source, target)
                .iter()
                .map(|m| {
                    let mut start = ">".to_owned();
                    start.push_str(m);
                    start
                })
                .collect::<Vec<_>>(),
        );
    }

    existing_moves
}

fn get_optimal_dpad_moves(source: char, target: char, depth: usize) -> String {
    if depth == 1 {
        let mut basic_moves = get_dpad_moves(source, target);
        basic_moves.push('A');
        // println!(
        //     "Optimal DPAD moves for {source} to {target} with depth {depth} are {basic_moves}"
        // );
        return basic_moves;
    }

    let current_moves: String = get_optimal_dpad_moves(source, target, depth - 1);
    let mut current_key = 'A';
    let mut future_moves = String::new();
    for key in current_moves.chars() {
        let mut basic_moves = get_dpad_moves(current_key, key);
        basic_moves.push('A');
        future_moves.push_str(&basic_moves);
        current_key = key
    }

    // println!("Optimal DPAD moves for {source} to {target} with depth {depth} are {future_moves}");
    future_moves
}

fn get_optimal_keypad_moves(current_key: char, key: char, depth: usize) -> String {
    // optimal moves to move from current_key to key in a keypad ex A -> 4
    let possible_moves = get_existing_keypad_moves(current_key, key);
    possible_moves
        .iter()
        .map(|m| {
            // move m is for example <^^<A
            let mut new_move = m.to_owned();
            new_move.push('A');
            let mut current_key = 'A';
            let mut total_input: String = String::new();

            for key in new_move.chars() {
                total_input.push_str(&get_optimal_dpad_moves(current_key, key, depth));
                current_key = key;
            }
            total_input
        })
        .min_by_key(|m| m.len())
        .unwrap()
}

fn order_keypad_input(keypad_input: &str, depth: usize) -> String {
    let mut current_key: char = 'A';
    let mut total_input: String = String::new();
    for key in keypad_input.chars() {
        total_input.push_str(&get_optimal_keypad_moves(current_key, key, depth - 1));
        current_key = key;
    }

    total_input
}

pub fn part_one(input: &str) -> Option<isize> {
    // let keypad: [[char; 3]; 4] = initialize_keypad();
    // let dpad: [[char; 3]; 2] = initialize_dpad();

    Some(
        input
            .lines()
            .map(|line| {
                let robot_input = order_keypad_input(line, 3);

                let num_part = line
                    .chars()
                    .filter(|c| c.is_numeric())
                    .collect::<String>()
                    .parse::<isize>()
                    .unwrap();

                println!(
                    "\n\n\n{line}\n{robot_input}\n{num_part}*{}={}",
                    robot_input.len(),
                    (robot_input.len() as isize) * num_part
                );

                (robot_input.len() as isize) * num_part
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(126384));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
