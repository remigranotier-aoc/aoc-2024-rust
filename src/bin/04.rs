advent_of_code::solution!(4);

fn count_xmas_from(char_array: Vec<Vec<char>>, i: usize, j: usize) -> u32 {
    let v_size = char_array.len();
    let h_size = char_array.first().unwrap().len();
    let mut count = 0;

    // Right
    if j < h_size - 3
        && char_array[i][j + 1] == 'M'
        && char_array[i][j + 2] == 'A'
        && char_array[i][j + 3] == 'S'
    {
        count += 1;
    }

    // Left
    if j > 2
        && char_array[i][j - 1] == 'M'
        && char_array[i][j - 2] == 'A'
        && char_array[i][j - 3] == 'S'
    {
        count += 1;
    }

    // Up
    if i > 2
        && char_array[i - 1][j] == 'M'
        && char_array[i - 2][j] == 'A'
        && char_array[i - 3][j] == 'S'
    {
        count += 1;
    }

    // Down
    if i < v_size - 3
        && char_array[i + 1][j] == 'M'
        && char_array[i + 2][j] == 'A'
        && char_array[i + 3][j] == 'S'
    {
        count += 1;
    }

    // Down-Right
    if i < v_size - 3
        && j < h_size - 3
        && char_array[i + 1][j + 1] == 'M'
        && char_array[i + 2][j + 2] == 'A'
        && char_array[i + 3][j + 3] == 'S'
    {
        count += 1;
    }

    // Down-Left
    if i < v_size - 3
        && j > 2
        && char_array[i + 1][j - 1] == 'M'
        && char_array[i + 2][j - 2] == 'A'
        && char_array[i + 3][j - 3] == 'S'
    {
        count += 1;
    }

    // Up-Left
    if i > 2
        && j > 2
        && char_array[i - 1][j - 1] == 'M'
        && char_array[i - 2][j - 2] == 'A'
        && char_array[i - 3][j - 3] == 'S'
    {
        count += 1;
    }

    // Up-Right
    if i > 2
        && j < h_size - 3
        && char_array[i - 1][j + 1] == 'M'
        && char_array[i - 2][j + 2] == 'A'
        && char_array[i - 3][j + 3] == 'S'
    {
        count += 1;
    }

    count
}

fn is_x_mas_from(char_array: Vec<Vec<char>>, i: usize, j: usize) -> bool {
    let v_size = char_array.len();
    let h_size = char_array.first().unwrap().len();

    if i == 0 || i == v_size - 1 || j == 0 || j == h_size - 1 {
        return false;
    }

    let bot_right_diag = [
        char_array[i - 1][j - 1],
        char_array[i][j],
        char_array[i + 1][j + 1],
    ]
    .into_iter()
    .collect::<String>();

    let bot_left_diag = [
        char_array[i + 1][j - 1],
        char_array[i][j],
        char_array[i - 1][j + 1],
    ]
    .into_iter()
    .collect::<String>();

    (bot_right_diag == "MAS" || bot_right_diag == "SAM")
        && (bot_left_diag == "MAS" || bot_left_diag == "SAM")
}

pub fn part_one(input: &str) -> Option<u32> {
    let char_array = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();

    let mut number_of_xmas = 0;

    for (i, line) in char_array.iter().enumerate() {
        for (j, char) in line.iter().enumerate() {
            // line i, column j
            if *char == 'X' {
                number_of_xmas += count_xmas_from(char_array.clone(), i, j)
            }
        }
    }

    Some(number_of_xmas)
}

pub fn part_two(input: &str) -> Option<u32> {
    let char_array = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();

    let mut number_of_x_mas = 0;

    for (i, line) in char_array.iter().enumerate() {
        for (j, char) in line.iter().enumerate() {
            // line i, column j
            if *char == 'A' && is_x_mas_from(char_array.clone(), i, j) {
                number_of_x_mas += 1;
            }
        }
    }

    Some(number_of_x_mas)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
