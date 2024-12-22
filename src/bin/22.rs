use std::collections::HashMap;

advent_of_code::solution!(22);

fn new_secret_number(secret_number: i64) -> i64 {
    let first_step = first_step(secret_number);
    let second_step = second_step(first_step);
    third_step(second_step)
}

fn first_step(number: i64) -> i64 {
    ((number * 64) ^ number) % 16777216
}

fn second_step(number: i64) -> i64 {
    ((number / 32) ^ number) % 16777216
}

fn third_step(number: i64) -> i64 {
    ((number * 2048) ^ number) % 16777216
}

pub fn part_one(input: &str) -> Option<i64> {
    let mut total = 0;
    for seed in input.lines().map(|l| l.parse::<i64>().unwrap()) {
        let mut current_number = seed;
        for _ in 0..2000 {
            current_number = new_secret_number(current_number);
        }
        total += current_number;
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut sequences = vec![];
    for seed in input.lines().map(|l| l.parse::<i64>().unwrap()) {
        let mut current_number = seed;
        let mut sequence = vec![];
        sequence.push(current_number);
        for _ in 0..2000 {
            current_number = new_secret_number(current_number);
            sequence.push(current_number);
        }
        sequences.push(sequence);
    }

    let mut differences = vec![];
    for sequence in &sequences {
        let mut diff_seq: Vec<i64> = vec![];
        for w in sequence.windows(2) {
            diff_seq.push(w[1] % 10 - w[0] % 10);
        }
        differences.push(diff_seq);
    }

    // Sequences of prices are filled now
    let mut seen_sequences: HashMap<(i64, i64, i64, i64), HashMap<i64, i64>> = HashMap::new(); // (1,2,3,4) seen in [(2nd buyer:index 38), (37th buyer, index 677), etc]
    for (i, diff_seq) in differences.iter().enumerate() {
        for (j, w) in diff_seq.windows(4).enumerate() {
            let seq_map = seen_sequences.entry((w[0], w[1], w[2], w[3])).or_default();
            if seq_map.get(&(i as i64)).is_none() {
                seq_map.insert(i as i64, j as i64);
            }
        }
    }

    let mut max_total = 0;
    for (_, appearances) in seen_sequences {
        let mut total = 0;
        for (seller_index, appearance_index) in appearances {
            total += sequences[seller_index as usize][(appearance_index + 4) as usize] % 10;
        }

        if total > max_total {
            max_total = total;
        }
    }

    Some(max_total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(37990510));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(23));
    }
}
