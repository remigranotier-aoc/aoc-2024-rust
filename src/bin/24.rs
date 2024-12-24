#![allow(unreachable_code)]

use std::collections::HashMap;

use itertools::Itertools;
use regex::Regex;

advent_of_code::solution!(24);

fn get_wire_value(
    wire: &str,
    values: &mut HashMap<&str, bool>,
    branchment_map: &HashMap<&str, (&str, &str, &str)>,
) -> bool {
    if let Some(v) = values.get(wire) {
        return *v;
    }

    let (wire1, op, wire2) = branchment_map.get(wire).unwrap();
    let val1 = get_wire_value(wire1, values, branchment_map);
    let val2 = get_wire_value(wire2, values, branchment_map);
    match *op {
        "AND" => val1 && val2,
        "OR" => val1 || val2,
        "XOR" => (val1 && !val2) || (!val1 && val2),
        _ => unreachable!(),
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let (inputs, branchments) = input.split("\n\n").next_tuple().unwrap();
    let mut values: HashMap<&str, bool> = HashMap::new();

    for input in inputs.lines() {
        let (initial_wire, value) = input.split(": ").next_tuple().unwrap();
        values.insert(initial_wire, value == "1");
    }

    let mut branchment_map: HashMap<&str, (&str, &str, &str)> = HashMap::new();
    let mut max_z = 0;
    for branchment in branchments.lines() {
        let (branchment_options, target) = branchment.split(" -> ").next_tuple().unwrap();
        let (wire1, operator, wire2) = branchment_options.split(" ").next_tuple().unwrap();

        let z_regex = Regex::new(r"^z([0-9]{2})$").unwrap();
        if let Some(c) = z_regex.captures(target) {
            let new_z = c.get(1).unwrap().as_str().parse::<usize>().unwrap();
            if new_z > max_z {
                max_z = new_z;
            }
        }

        if values.contains_key(wire1) && values.contains_key(wire2) {
            let val1 = *values.get(wire1).unwrap();
            let val2 = *values.get(wire2).unwrap();
            let new_val = match operator {
                "AND" => val1 && val2,
                "OR" => val1 || val2,
                "XOR" => (val1 && !val2) || (!val1 && val2),
                _ => unreachable!(),
            };
            values.insert(target, new_val);
            continue;
        }

        branchment_map.insert(target, (wire1, operator, wire2));
    }

    let mut total_z = 0;
    for i in 0..=max_z {
        let current_z = get_wire_value(&format!("z{:02}", i), &mut values, &branchment_map);
        if current_z {
            total_z += 2_usize.pow(i as u32);
        }
    }
    Some(total_z)
}

pub fn part_two(_input: &str) -> Option<u32> {
    // THIS IS NOT AN ACTUAL CODING ANSWER. BY CONSTRUCTING THE TREE AND MATCHING ERRORS WITH EXPECTED NAMES, I'VE FOUND THE NEEDED SWAPS AND DONE THEM LINE 98 BELOW.

    // ZN = SN XOR CN-1
    // CN = AN OR BN
    // SN = XN XOR YN OK
    // AN = XN AND YN OK
    // BN = CN-1 AND SN
    // (x|y)([0-9]{2}) XOR (x|y)([0-9]{2})
    // calcul de B9 problématique
    // nnt, gws - z13, npf - z19, cph - z33, hgj
    // cph,gws,hgj,nnt,npf,z13,z19,z33

    return None;

    let (inputs, mut branchments) = _input.split("\n\n").next_tuple().unwrap();
    let mut values: HashMap<&str, bool> = HashMap::new();

    for input in inputs.lines() {
        let (initial_wire, value) = input.split(": ").next_tuple().unwrap();
        values.insert(initial_wire, value == "1");
    }

    let binding = branchments
        .replace("-> nnt", "-> temp")
        .replace("-> gws", "-> nnt")
        .replace("-> temp", "-> gws")
        .replace("-> z13", "-> temp")
        .replace("-> npf", "-> z13")
        .replace("-> temp", "-> npf")
        .replace("-> z19", "-> temp")
        .replace("-> cph", "-> z19")
        .replace("-> temp", "-> cph")
        .replace("-> z33", "-> temp")
        .replace("-> hgj", "-> z33")
        .replace("-> temp", "-> hgj");
    branchments = &binding;

    let mut branchment_map: HashMap<&str, (&str, &str, &str)> = HashMap::new();
    let mut max_z = 0;
    for branchment in branchments.lines() {
        let (branchment_options, target) = branchment.split(" -> ").next_tuple().unwrap();
        let (wire1, operator, wire2) = branchment_options.split(" ").next_tuple().unwrap();

        let z_regex = Regex::new(r"^z([0-9]{2})$").unwrap();
        if let Some(c) = z_regex.captures(target) {
            let new_z = c.get(1).unwrap().as_str().parse::<usize>().unwrap();
            if new_z > max_z {
                max_z = new_z;
            }
        }

        branchment_map.insert(target, (wire1, operator, wire2));
    }

    let mut s_series: HashMap<usize, &str> = HashMap::new();
    for i in 1..max_z {
        let fstring = format!("(?:x|y){i:02} XOR (?:x|y){i:02} -> (.*)");
        let s_regex = Regex::new(&fstring).unwrap();
        s_series.insert(
            i,
            s_regex
                .captures(branchments)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str(),
        );
    }
    println!("S SERIES {s_series:?}");

    let mut a_series: HashMap<usize, &str> = HashMap::new();
    for i in 0..max_z {
        let fstring = format!("(?:x|y){i:02} AND (?:x|y){i:02} -> (.*)");
        let a_regex = Regex::new(&fstring).unwrap();
        a_series.insert(
            i,
            a_regex
                .captures(branchments)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str(),
        );
    }
    println!("A SERIES {a_series:?}");

    let mut c_series: HashMap<usize, &str> = HashMap::new();
    let mut b_series: HashMap<usize, &str> = HashMap::new();
    c_series.insert(0, a_series.get(&0).unwrap());
    b_series.insert(0, "0");
    for i in 1..max_z {
        let prev_c = c_series.get(&(i - 1)).unwrap();
        let curr_s = s_series.get(&i).unwrap();
        let fstring = format!("(?:{prev_c}|{curr_s}) AND (?:{prev_c}|{curr_s}) -> (.*)");
        let b_regex = Regex::new(&fstring).unwrap();
        let b_captures = b_regex.captures(branchments);
        if b_captures.is_none() {
            println!("PREV_C = {prev_c}, CURR_S = {curr_s}");
        }
        b_series.insert(i, b_captures.unwrap().get(1).unwrap().as_str());

        let curr_a = a_series.get(&i).unwrap();
        let curr_b = b_series.get(&i).unwrap();
        let fstring = format!("(?:{curr_a}|{curr_b}) OR (?:{curr_a}|{curr_b}) -> (.*)");
        let c_regex = Regex::new(&fstring).unwrap();
        let c_captures = c_regex.captures(branchments);
        if c_captures.is_none() {
            println!("CURR_A = {curr_a}, CURR_B = {curr_b}");
        }
        c_series.insert(
            i,
            c_regex
                .captures(branchments)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str(),
        );
        println!("OK for i={i}");
    }

    println!("B SERIES {b_series:?}");
    println!("C SERIES {c_series:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2024));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
