use itertools::Itertools;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let mut current_id = 0;
    let mut is_mem = true;
    let mut disk_memory: Vec<Option<u64>> = vec![];

    // Fill disk_memory
    for char in input
        .trim()
        .chars()
        .map(|c| c.to_string().parse::<u64>().unwrap())
    {
        if is_mem {
            for _ in 0..char {
                disk_memory.push(Some(current_id));
            }
            current_id += 1;
        } else {
            for _ in 0..char {
                disk_memory.push(None);
            }
        }
        is_mem = !is_mem;
    }

    let mut start_index = 0;
    let mut end_index = disk_memory.len() - 1;
    while start_index < end_index {
        while disk_memory[start_index].is_some() {
            start_index += 1;
        }
        while disk_memory[end_index].is_none() {
            end_index -= 1;
        }

        // Once swaps are found
        if start_index < end_index {
            disk_memory.swap(start_index, end_index);
        }
    }

    Some(
        disk_memory
            .iter()
            .enumerate()
            .map(|(i, v)| i as u64 * v.unwrap_or(0))
            .sum(),
    )
}

#[derive(Clone, Copy, Debug)]
struct Block {
    value: Option<u64>,
    size: u64,
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut current_id = 0;
    let mut is_mem = true;
    let mut disk_memory: Vec<Block> = vec![];

    // Fill disk_memory
    for char in input
        .trim()
        .chars()
        .map(|c| c.to_string().parse::<u64>().unwrap())
    {
        if is_mem {
            disk_memory.push(Block {
                value: Some(current_id),
                size: char,
            });
            current_id += 1;
        } else {
            disk_memory.push(Block {
                value: None,
                size: char,
            });
        }
        is_mem = !is_mem;
    }

    // iterating on blocks by reverse
    let mut last_id = current_id - 1;
    while last_id > 0 {
        let mem_copy = &disk_memory.to_vec();
        // try to bring last_id ahead
        let (last_block_position, last_block_value) = mem_copy
            .iter()
            .find_position(|c| c.value.unwrap_or(0) == last_id)
            .unwrap();

        // find earliest available block possible
        let empty_block_result = mem_copy
            .iter()
            .take(last_block_position)
            .find_position(|block| block.value.is_none() && block.size >= last_block_value.size);

        // data movement
        if let Some((empty_block_position, _)) = empty_block_result {
            disk_memory[empty_block_position].size -= last_block_value.size;
            disk_memory[last_block_position].value = None;
            disk_memory.insert(empty_block_position, *last_block_value);
        }

        last_id -= 1;
    }

    // Final sum computation
    let mut index = 0;
    let mut final_sum = 0;
    for block in disk_memory {
        for _ in 0..block.size {
            final_sum += index * block.value.unwrap_or(0);
            index += 1;
        }
    }

    Some(final_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
