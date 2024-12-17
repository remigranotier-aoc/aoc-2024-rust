use std::ffi::IntoStringError;

use itertools::Itertools;

advent_of_code::solution!(17);

fn combo(operand: u64, registers: &[u64; 3]) -> u64 {
    match operand {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 3,
        4 => registers[0],
        5 => registers[1],
        6 => registers[2],
        _ => unreachable!(),
    }
}

fn execute_instruction(
    program: &[u64],
    registers: &mut [u64; 3],
    instruction_pointer: &mut usize,
    outputs: &mut Vec<u64>,
) -> Option<usize> {
    let opcode = program[*instruction_pointer];
    let operand = program[*instruction_pointer + 1];
    let mut is_pb_detected = None;

    // println!("Executing {opcode} with operand {operand}");

    match opcode {
        0 => {
            // adv
            registers[0] /= 2_u64.pow(combo(operand, registers) as u32);
            *instruction_pointer += 2;
        }
        1 => {
            // bxl
            registers[1] ^= operand;
            *instruction_pointer += 2;
        }
        2 => {
            // bst
            registers[1] = combo(operand, registers) % 8;
            *instruction_pointer += 2;
        }
        3 => {
            // jnz
            if registers[0] == 0 {
                *instruction_pointer += 2;
            } else {
                *instruction_pointer = operand as usize;
            }
        }
        4 => {
            // bxc
            registers[1] ^= registers[2];
            *instruction_pointer += 2;
        }
        5 => {
            // out
            let n = outputs.len();
            let val_to_out = combo(operand, registers) % 8;
            if n >= program.len() || program[n] != val_to_out {
                is_pb_detected = Some(n);
            }
            outputs.push(val_to_out);
            *instruction_pointer += 2;
        }
        6 => {
            // bdv
            registers[1] = registers[0] / 2_u64.pow(combo(operand, registers) as u32);
            *instruction_pointer += 2;
        }
        7 => {
            // cdv
            registers[2] = registers[0] / 2_u64.pow(combo(operand, registers) as u32);
            *instruction_pointer += 2;
        }
        _ => unreachable!(),
    }

    is_pb_detected
}

fn reverse(
    program: &[u64],
    registers: &mut [u64; 3],
    instruction_pointer: &mut usize,
    outputs: &mut Vec<u64>,
) {
    let opcode = program[*instruction_pointer];
    let operand = program[*instruction_pointer + 1];

    // println!("Executing {opcode} with operand {operand}");

    match opcode {
        0 => {
            // adv
            registers[0] /= 2_u64.pow(combo(operand, registers) as u32);
            *instruction_pointer += 2;
        }
        1 => {
            // bxl
            registers[1] ^= operand;
            *instruction_pointer += 2;
        }
        2 => {
            // bst
            registers[1] = combo(operand, registers) % 8;
            *instruction_pointer += 2;
        }
        3 => {
            // jnz
            if registers[0] == 0 {
                *instruction_pointer += 2;
            } else {
                *instruction_pointer = operand as usize;
            }
        }
        4 => {
            // bxc
            registers[1] ^= registers[2];
            *instruction_pointer += 2;
        }
        5 => {
            // out
            outputs.push(combo(operand, registers) % 8);
            *instruction_pointer += 2;
        }
        6 => {
            // bdv
            registers[1] = registers[0] / 2_u64.pow(combo(operand, registers) as u32);
            *instruction_pointer += 2;
        }
        7 => {
            // cdv
            registers[2] = registers[0] / 2_u64.pow(combo(operand, registers) as u32);
            *instruction_pointer += 2;
        }
        _ => unreachable!(),
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let (registers_text, program_text) = input.split("\n\n").next_tuple().unwrap();
    let mut registers: [u64; 3] = [0; 3];
    for (i, line) in registers_text.lines().enumerate() {
        registers[i] = line.split(": ").collect::<Vec<_>>()[1]
            .parse::<u64>()
            .unwrap();
    }

    let mut program: Vec<u64> = vec![];
    for number in program_text.trim().split(": ").collect::<Vec<_>>()[1].split(",") {
        program.push(number.parse::<u64>().unwrap());
    }

    let mut instruction_pointer: usize = 0;
    let mut outputs: Vec<u64> = vec![];

    while instruction_pointer < program.len() {
        // println!("{instruction_pointer} - {registers:?}");
        execute_instruction(
            &program,
            &mut registers,
            &mut instruction_pointer,
            &mut outputs,
        );
    }

    // println!("{instruction_pointer} - {registers:?}");

    Some(outputs.into_iter().join(","))
}

pub fn part_two(input: &str) -> Option<u64> {
    let (registers_text, program_text) = input.split("\n\n").next_tuple().unwrap();
    let mut registers: [u64; 3] = [0; 3];
    for (i, line) in registers_text.lines().enumerate() {
        registers[i] = line.split(": ").collect::<Vec<_>>()[1]
            .parse::<u64>()
            .unwrap();
    }

    let mut program: Vec<u64> = vec![];
    for number in program_text.trim().split(": ").collect::<Vec<_>>()[1].split(",") {
        program.push(number.parse::<u64>().unwrap());
    }

    let mut outputs: Vec<u64> = vec![];
    let mut advancement = 1;

    let mut test_a = 0;
    while advancement < program.len() {
        // println!(
        //     "Looking for output equal to {:?}",
        //     program[(program.len() - advancement)..].iter().join(",")
        // );
        while outputs.iter().join(",") != program[(program.len() - advancement)..].iter().join(",")
        {
            let mut temp_registers: [u64; 3] = [0; 3];
            let mut instruction_pointer: usize = 0;
            temp_registers[0] = test_a;
            outputs = vec![];
            while instruction_pointer < program.len() {
                // println!("{instruction_pointer} - {registers:?}");
                execute_instruction(
                    &program,
                    &mut temp_registers,
                    &mut instruction_pointer,
                    &mut outputs,
                );
            }

            // println!("Output for a={test_a} is {}", outputs.iter().join(","));
            test_a += 1
        }
        // println!(
        //     "Number {:o} worked for advancement {}",
        //     test_a - 1,
        //     advancement
        // );
        test_a = (test_a - 1) * 8;
        advancement += 1;
    }

    let mut answer = 0;
    for i in 0..8 {
        let final_a = test_a + i;
        let mut temp_registers: [u64; 3] = [0; 3];
        let mut instruction_pointer: usize = 0;
        temp_registers[0] = final_a;
        outputs = vec![];
        while instruction_pointer < program.len() {
            // println!("{instruction_pointer} - {registers:?}");
            execute_instruction(
                &program,
                &mut temp_registers,
                &mut instruction_pointer,
                &mut outputs,
            );
        }

        if outputs.iter().join(",") == program.iter().join(",") {
            answer = final_a;
            // println!("{final_a}");
        }
    }

    Some(answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let binding = advent_of_code::template::read_file("examples", DAY);
        let result = part_one(&binding);
        assert_eq!(result, Some("5,7,3,0".to_owned()));
    }

    #[test]
    fn test_part_two() {
        let binding = advent_of_code::template::read_file("examples", DAY);
        let result = part_two(&binding);
        assert_eq!(result, Some(117440));
    }
}
