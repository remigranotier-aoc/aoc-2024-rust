use itertools::Itertools;
use regex::Regex;

advent_of_code::solution!(14);

const X_SIZE: i32 = 101;
const Y_SIZE: i32 = 103;
const N_SECONDS: i32 = 100;

pub fn part_one(input: &str) -> Option<u32> {
    let input_regex: Regex =
        Regex::new(r"p=([0-9]{1,3}),([0-9]{1,3}) v=(\-?[0-9]{1,3}),(\-?[0-9]{1,3})").unwrap();
    let mut top_left_counter: u32 = 0;
    let mut top_right_counter: u32 = 0;
    let mut bottom_left_counter: u32 = 0;
    let mut bottom_right_counter: u32 = 0;
    for line in input.lines() {
        let Some((_, parameters)) = input_regex.captures(line).map(|caps| {
            let (full, extracts): (&str, [&str; 4]) = caps.extract();
            let parameters: Vec<i32> = extracts.iter().map(|v| v.parse::<i32>().unwrap()).collect();
            (full, parameters)
        }) else {
            continue;
        };
        let (x, y, vx, vy) = parameters.into_iter().tuples().next().unwrap();
        let final_x = (x + N_SECONDS * vx).rem_euclid(X_SIZE);
        let final_y = (y + N_SECONDS * vy).rem_euclid(Y_SIZE);
        if final_x < X_SIZE / 2 {
            if final_y < Y_SIZE / 2 {
                top_left_counter += 1;
            }
            if final_y >= Y_SIZE - Y_SIZE / 2 {
                top_right_counter += 1;
            }
        }
        if final_x >= X_SIZE - X_SIZE / 2 {
            if final_y < Y_SIZE / 2 {
                bottom_left_counter += 1;
            }
            if final_y >= Y_SIZE - Y_SIZE / 2 {
                bottom_right_counter += 1;
            }
        }
    }

    Some(top_left_counter * top_right_counter * bottom_left_counter * bottom_right_counter)
}

pub fn part_two(input: &str) -> Option<u32> {
    let input_regex: Regex =
        Regex::new(r"p=([0-9]{1,3}),([0-9]{1,3}) v=(\-?[0-9]{1,3}),(\-?[0-9]{1,3})").unwrap();
    let mut robots: Vec<(i32, i32, i32, i32)> = vec![];
    for line in input.lines() {
        let Some((_, parameters)) = input_regex.captures(line).map(|caps| {
            let (full, extracts): (&str, [&str; 4]) = caps.extract();
            let parameters: Vec<i32> = extracts.iter().map(|v| v.parse::<i32>().unwrap()).collect();
            (full, parameters)
        }) else {
            continue;
        };
        let (x, y, vx, vy) = parameters.into_iter().tuples().next().unwrap();
        robots.push((x, y, vx, vy));
    }

    for i in 0..10000 {
        let mut room: [[char; X_SIZE as usize]; Y_SIZE as usize] =
            [['.'; X_SIZE as usize]; Y_SIZE as usize];
        for robot in robots.iter_mut() {
            room[robot.1 as usize][robot.0 as usize] = '#';
            robot.0 = (robot.0 + robot.2).rem_euclid(X_SIZE);
            robot.1 = (robot.1 + robot.3).rem_euclid(Y_SIZE);
        }
        if room
            .map(|l| l.iter().join(""))
            .join("\n")
            .contains("###############################")
        {
            // println!("Pattern found after {i} seconds");
            // println!("{}", room.map(|l| l.iter().join("")).join("\n"));
            return Some(i);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
