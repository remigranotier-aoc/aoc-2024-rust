use bitflags::bitflags;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(12);

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Fences: u32 {
        const DEFAULT = 0b00000000;
        const UP = 0b00000001;
        const DOWN = 0b00000010;
        const RIGHT = 0b00000100;
        const LEFT = 0b000001000;
    }
}

impl Fences {
    pub fn as_usize(&self) -> usize {
        self.bits() as usize
    }

    pub fn count(&self) -> usize {
        let mut c = 0;
        if *self & Fences::UP == Fences::UP {
            c += 1;
        }
        if *self & Fences::DOWN == Fences::DOWN {
            c += 1;
        }
        if *self & Fences::RIGHT == Fences::RIGHT {
            c += 1;
        }
        if *self & Fences::LEFT == Fences::LEFT {
            c += 1;
        }
        c
    }
}

fn list_fences(index: usize, value: char, field: &[char], h_size: usize, v_size: usize) -> Fences {
    let i = index / h_size;
    let j = index % h_size;
    let mut fences: Fences = Fences::DEFAULT;

    if i == 0 || field[j + (i - 1) * h_size] != value {
        fences |= Fences::UP;
    }

    if i == v_size - 1 || field[j + (i + 1) * h_size] != value {
        fences |= Fences::DOWN;
    }

    if j == 0 || field[j - 1 + i * h_size] != value {
        fences |= Fences::LEFT;
    }

    if j == h_size - 1 || field[j + 1 + i * h_size] != value {
        fences |= Fences::RIGHT;
    }

    fences
}

fn count_fences(index: usize, value: char, field: &[char], h_size: usize, v_size: usize) -> usize {
    let i = index / h_size;
    let j = index % h_size;
    let mut fences = 0;

    if i == 0 || field[j + (i - 1) * h_size] != value {
        fences += 1;
    }

    if i == v_size - 1 || field[j + (i + 1) * h_size] != value {
        fences += 1;
    }

    if j == 0 || field[j - 1 + i * h_size] != value {
        fences += 1
    }

    if j == h_size - 1 || field[j + 1 + i * h_size] != value {
        fences += 1;
    }

    fences
}

fn get_neighbors(index: usize, field: &[char], h_size: usize, v_size: usize) -> Vec<usize> {
    let mut neighbors: Vec<usize> = Vec::new();
    let value = field[index];
    let i = index / h_size;
    let j = index % h_size;

    if i > 0 && field[j + (i - 1) * h_size] == value {
        neighbors.push(index - h_size);
    }

    if i < v_size - 1 && field[j + (i + 1) * h_size] == value {
        neighbors.push(index + h_size);
    }

    if j < h_size - 1 && field[j + 1 + i * h_size] == value {
        neighbors.push(index + 1);
    }

    if j > 0 && field[j - 1 + i * h_size] == value {
        neighbors.push(index - 1);
    }

    neighbors
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut field = input
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<Vec<_>>();
    let v_size = input.lines().collect::<Vec<_>>().iter().len();
    let h_size = input.lines().next().unwrap().len();
    let mut cost_total: usize = 0;

    // While there isnt only "."
    while field.iter().counts().keys().len() > 1 {
        let (index_new_starter, value) = field.iter().enumerate().find(|(_, &v)| v != '.').unwrap();

        // maps index to number of fences
        let mut parcel: HashMap<usize, usize> = HashMap::new();
        let mut prev_size = 0;
        parcel.insert(
            index_new_starter,
            count_fences(index_new_starter, *value, &field, h_size, v_size),
        );

        // While cases keep getting added
        while parcel.keys().len() > prev_size {
            prev_size = parcel.keys().len();
            let current_parcel = parcel.clone();
            for case in current_parcel.keys() {
                let neighbors = get_neighbors(*case, &field, h_size, v_size);
                for neighbor_index in neighbors {
                    parcel.entry(neighbor_index).or_insert_with(|| {
                        count_fences(neighbor_index, *value, &field, h_size, v_size)
                    });
                }
            }
        }

        // println!(
        //     "{value} === {:?} | {} | {} ",
        //     parcel,
        //     parcel.keys().len(),
        //     parcel.values().sum::<usize>()
        // );

        for key in parcel.keys() {
            field[*key] = '.';
        }

        cost_total += parcel.keys().len() * parcel.values().sum::<usize>();
    }

    Some(cost_total)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut field = input
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<Vec<_>>();
    let v_size = input.lines().collect::<Vec<_>>().iter().len();
    let h_size = input.lines().next().unwrap().len();
    let mut cost_total: usize = 0;

    // While there isnt only "."
    while field.iter().counts().keys().len() > 1 {
        let (index_new_starter, value) = field.iter().enumerate().find(|(_, &v)| v != '.').unwrap();

        // maps index to number of fences
        let mut parcel: HashMap<usize, Fences> = HashMap::new();
        let mut prev_size = 0;
        let mut sides = 0;
        parcel.insert(
            index_new_starter,
            list_fences(index_new_starter, *value, &field, h_size, v_size),
        );
        let new_sides = parcel.get(&index_new_starter).unwrap().count();
        sides += new_sides;

        // While cases keep getting added
        while parcel.keys().len() > prev_size {
            prev_size = parcel.keys().len();
            for (case, _) in parcel.clone() {
                let neighbors = get_neighbors(case, &field, h_size, v_size);
                for neighbor_index in neighbors {
                    #[allow(clippy::map_entry)]
                    if !parcel.contains_key(&neighbor_index) {
                        let fences = list_fences(neighbor_index, *value, &field, h_size, v_size);
                        parcel.insert(neighbor_index, fences);
                        let neighneigh = get_neighbors(neighbor_index, &field, h_size, v_size);
                        let mut nn_fences = Fences::DEFAULT;
                        for nn in neighneigh {
                            nn_fences |= *parcel.get(&nn).unwrap_or(&Fences::DEFAULT);
                        }

                        let new_sides = (fences & !nn_fences).count();
                        sides += new_sides;
                    }
                }
            }
        }

        for key in parcel.keys() {
            field[*key] = '.';
        }

        cost_total += parcel.keys().len() * sides;
    }

    Some(cost_total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
