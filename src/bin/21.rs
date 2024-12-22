use std::collections::HashMap;

use itertools::Itertools;

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

fn get_existing_dpad_moves(source: char, target: char) -> Vec<String> {
    let mut existing_moves: Vec<String> = vec![];
    let (i_source, j_source) = get_dpad_loc(source);
    let (i_target, j_target) = get_dpad_loc(target);
    if source == target {
        existing_moves.push("".to_owned());
        return existing_moves;
    }

    #[allow(clippy::comparison_chain)]
    if i_target < i_source && !(i_target == 0 && j_source == 0) {
        let new_source = get_dpad_char(i_source - 1, j_source);
        existing_moves.append(
            &mut get_existing_dpad_moves(new_source, target)
                .iter()
                .map(|m| {
                    let mut start = "^".to_owned();
                    start.push_str(m);
                    start
                })
                .collect::<Vec<_>>(),
        );
    } else if i_target > i_source {
        let new_source = get_dpad_char(i_source + 1, j_source);
        existing_moves.append(
            &mut get_existing_dpad_moves(new_source, target)
                .iter()
                .map(|m| {
                    let mut start = "v".to_owned();
                    start.push_str(m);
                    start
                })
                .collect::<Vec<_>>(),
        );
    }

    if j_target < j_source && !(j_target == 0 && i_source == 0) {
        let new_source = get_dpad_char(i_source, j_source - 1);
        existing_moves.append(
            &mut get_existing_dpad_moves(new_source, target)
                .iter()
                .map(|m| {
                    let mut start = "<".to_owned();
                    start.push_str(m);
                    start
                })
                .collect::<Vec<_>>(),
        );
    } else if j_target > j_source {
        let new_source = get_dpad_char(i_source, j_source + 1);
        existing_moves.append(
            &mut get_existing_dpad_moves(new_source, target)
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

fn build_shortest_path_keypad() -> HashMap<(char, char), Vec<String>> {
    let mut possible_paths = HashMap::new();
    let possible_keys = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A'];
    for (source, target) in possible_keys.iter().cartesian_product(possible_keys) {
        possible_paths.insert(
            (*source, target),
            get_existing_keypad_moves(*source, target),
        );
    }

    possible_paths
}

fn build_shortest_path_dpad() -> HashMap<(char, char), Vec<String>> {
    let mut possible_paths = HashMap::new();
    let possible_keys = ['<', '>', 'v', '^', 'A'];
    for (source, target) in possible_keys.iter().cartesian_product(possible_keys) {
        possible_paths.insert((*source, target), get_existing_dpad_moves(*source, target));
    }

    possible_paths
}

fn build_seq(
    keys: &String,
    index: usize,
    prev_key: char,
    current_path: &String,
    result: &mut Vec<String>,
    graph: &HashMap<(char, char), Vec<String>>,
) {
    if index == keys.len() {
        result.push(current_path.to_owned());
        return;
    }

    let keys_vec = keys.chars().collect::<Vec<_>>();

    for path in graph.get(&(prev_key, keys_vec[index])).unwrap() {
        let mut new_path = current_path.to_owned();
        new_path.push_str(path);
        new_path.push('A');
        build_seq(
            keys,
            index + 1,
            keys_vec[index],
            &new_path.to_owned(),
            result,
            graph,
        )
    }
}

pub fn shortest_seq(
    keys: &String,
    depth: usize,
    cache: &mut HashMap<(String, usize), usize>,
    shortest_path_dpad: &HashMap<(char, char), Vec<String>>,
) -> usize {
    if depth == 0 {
        return keys.len();
    }

    if let Some(s) = cache.get(&(keys.to_string(), depth)) {
        return *s;
    }

    let subkeys = keys.split('A').collect::<Vec<_>>();
    let mut total_size = 0;
    for subkey in subkeys.iter().take(subkeys.len() - 1) {
        let mut final_subkey = subkey.to_string();
        final_subkey.push('A');
        let mut sequences = vec![];
        build_seq(
            &final_subkey,
            0,
            'A',
            &"".to_owned(),
            &mut sequences,
            shortest_path_dpad,
        );

        let minimal_size = sequences
            .iter()
            .map(|seq| shortest_seq(&seq.to_string(), depth - 1, cache, shortest_path_dpad))
            .min()
            .unwrap();
        total_size += minimal_size;
    }
    cache.insert((keys.to_string(), depth), total_size);
    total_size
}

pub fn part_one(input: &str) -> Option<usize> {
    let shortest_path_keypad: HashMap<(char, char), Vec<String>> = build_shortest_path_keypad();
    let shortest_path_dpad: HashMap<(char, char), Vec<String>> = build_shortest_path_dpad();
    let total_depth = 3;
    let mut total_size = 0;
    let mut cache: HashMap<(String, usize), usize> = HashMap::new();

    for input in input.lines() {
        let mut sequences = vec![];
        build_seq(
            &input.to_string(),
            0,
            'A',
            &"".to_string(),
            &mut sequences,
            &shortest_path_keypad,
        );

        let minimal_size = sequences
            .iter()
            .map(|sequence| {
                shortest_seq(
                    &sequence.to_string(),
                    total_depth - 1,
                    &mut cache,
                    &shortest_path_dpad,
                )
            })
            .min()
            .unwrap();
        total_size += minimal_size
            * input
                .chars()
                .filter(|c| c.is_numeric())
                .collect::<String>()
                .parse::<usize>()
                .unwrap();
    }

    Some(total_size)
}

pub fn part_two(input: &str) -> Option<usize> {
    let shortest_path_keypad: HashMap<(char, char), Vec<String>> = build_shortest_path_keypad();
    let shortest_path_dpad: HashMap<(char, char), Vec<String>> = build_shortest_path_dpad();
    let total_depth = 26;
    let mut total_size = 0;
    let mut cache: HashMap<(String, usize), usize> = HashMap::new();

    for input in input.lines() {
        let mut sequences = vec![];
        build_seq(
            &input.to_string(),
            0,
            'A',
            &"".to_string(),
            &mut sequences,
            &shortest_path_keypad,
        );

        let minimal_size = sequences
            .iter()
            .map(|sequence| {
                shortest_seq(
                    &sequence.to_string(),
                    total_depth - 1,
                    &mut cache,
                    &shortest_path_dpad,
                )
            })
            .min()
            .unwrap();
        total_size += minimal_size
            * input
                .chars()
                .filter(|c| c.is_numeric())
                .collect::<String>()
                .parse::<usize>()
                .unwrap();
    }

    Some(total_size)
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
