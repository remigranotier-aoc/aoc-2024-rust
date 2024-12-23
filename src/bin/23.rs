use std::collections::{HashMap, HashSet};

use itertools::Itertools;

advent_of_code::solution!(23);

pub fn part_one(input: &str) -> Option<usize> {
    let mut network: HashMap<&str, HashSet<&str>> = HashMap::new();
    for line in input.lines() {
        let computers: (&str, &str) = line.split("-").next_tuple().unwrap();
        network.entry(computers.0).or_default().insert(computers.1);
        network.entry(computers.1).or_default().insert(computers.0);
    }

    Some(
        network
            .keys()
            .combinations(3)
            .filter(|comb| {
                let (&node1, &node2, &node3) = comb.iter().next_tuple().unwrap();
                network
                    .get(node1.to_owned())
                    .unwrap()
                    .contains(node2.to_owned())
                    && network
                        .get(node1.to_owned())
                        .unwrap()
                        .contains(node3.to_owned())
                    && network
                        .get(node2.to_owned())
                        .unwrap()
                        .contains(node3.to_owned())
                    && comb.iter().any(|node| node.starts_with("t"))
            })
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<String> {
    let mut network: HashMap<&str, HashSet<&str>> = HashMap::new();
    for line in input.lines() {
        let computers: (&str, &str) = line.split("-").next_tuple().unwrap();
        network.entry(computers.0).or_default().insert(computers.1);
        network.entry(computers.1).or_default().insert(computers.0);
    }

    let mut current_cliques = network
        .keys()
        .combinations(3)
        .filter(|comb| {
            let (&node1, &node2, &node3) = comb.iter().next_tuple().unwrap();
            network
                .get(node1.to_owned())
                .unwrap()
                .contains(node2.to_owned())
                && network
                    .get(node1.to_owned())
                    .unwrap()
                    .contains(node3.to_owned())
                && network
                    .get(node2.to_owned())
                    .unwrap()
                    .contains(node3.to_owned())
        })
        .collect::<HashSet<_>>();

    loop {
        let mut bigger_cliques: HashSet<Vec<&&str>> = HashSet::new();
        for clique in current_cliques.iter() {
            for (node, neighbors) in network.iter() {
                if clique.clone().iter().all(|&n| neighbors.contains(n)) {
                    let mut new_clique = clique.to_vec();
                    new_clique.push(node);
                    new_clique.sort();
                    bigger_cliques.insert(new_clique.to_vec());
                }
            }
        }

        if bigger_cliques.is_empty() {
            break;
        } else {
            current_cliques = bigger_cliques;
        }
    }

    let mut biggest_clique = current_cliques.iter().next().unwrap().to_vec();
    biggest_clique.sort();

    Some(biggest_clique.iter().join(",").to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("co,de,ka,ta".to_string()));
    }
}
