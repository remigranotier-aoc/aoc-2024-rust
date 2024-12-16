use itertools::Itertools;
use regex::Regex;

advent_of_code::solution!(13);

fn find_minimum_tokens(xa: i64, xb: i64, ya: i64, yb: i64, tx: i64, ty: i64) -> Option<(i64, i64)> {
    let mut combinations: Vec<(i64, i64)> = vec![];
    for a in 0..101 {
        if (tx - a * xa) % xb == 0 && (tx - a * xa) / xb > 0 && (tx - a * xa) / xb <= 100 {
            let b = (tx - a * xa) / xb;
            if ty == a * ya + yb * b {
                combinations.push((a, (tx - a * xa) / xb));
            }
        }
    }

    let min_tokens_comb = combinations
        .iter()
        .min_by(|ca, cb| (ca.0 + ca.1).cmp(&(cb.0 + cb.1)))
        .copied();
    min_tokens_comb
}

pub fn part_one(input: &str) -> Option<i64> {
    let input_regex: Regex = Regex::new(
        r"Button A: X\+([0-9]{1,3}), Y\+([0-9]{1,3})\nButton B: X\+([0-9]{1,3}), Y\+([0-9]{1,3})\nPrize: X=([0-9]+), Y=([0-9]+)",
    ).unwrap();
    let inputs = input.split("\n\n");
    let mut total_tokens = 0;

    for command in inputs {
        let Some((_, parameters)) = input_regex.captures(command).map(|caps| {
            let (full, extracts): (&str, [&str; 6]) = caps.extract();
            let parameters: Vec<i64> = extracts.iter().map(|v| v.parse::<i64>().unwrap()).collect();
            (full, parameters)
        }) else {
            continue;
        };
        let (xa, ya, xb, yb, tx, ty) = parameters.into_iter().tuples().next().unwrap();
        if let Some((a, b)) = find_minimum_tokens(xa, xb, ya, yb, tx, ty) {
            total_tokens += 3 * a + b;
        }
    }

    Some(total_tokens)
}

pub fn part_two(input: &str) -> Option<i64> {
    let input_regex: Regex = Regex::new(
        r"Button A: X\+([0-9]{1,3}), Y\+([0-9]{1,3})\nButton B: X\+([0-9]{1,3}), Y\+([0-9]{1,3})\nPrize: X=([0-9]+), Y=([0-9]+)",
    ).unwrap();
    let inputs = input.split("\n\n");
    let mut total_tokens = 0;

    for command in inputs {
        let Some((_, parameters)) = input_regex.captures(command).map(|caps| {
            let (full, extracts): (&str, [&str; 6]) = caps.extract();
            let mut parameters: Vec<i64> =
                extracts.iter().map(|v| v.parse::<i64>().unwrap()).collect();
            parameters[4] += 10000000000000;
            parameters[5] += 10000000000000;
            (full, parameters)
        }) else {
            continue;
        };
        let (xa, ya, xb, yb, tx, ty) = parameters.into_iter().tuples().next().unwrap();
        if yb * xa == xb * ya {
            println!("AU SECOURS C'EST UNE DIOPHANTIENNE");
            continue;
        }

        let a_rem = (ty * xb - yb * tx) % (ya * xb - yb * xa);
        let b_rem = (ty * xa - ya * tx) % (yb * xa - ya * xb);
        if a_rem != 0 || b_rem != 0 {
            continue;
        }

        let a = (ty * xb - yb * tx) / (ya * xb - yb * xa);
        let b = (ty * xa - ya * tx) / (yb * xa - ya * xb);
        total_tokens += 3 * a + b;
    }

    Some(total_tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908));
    }
}
