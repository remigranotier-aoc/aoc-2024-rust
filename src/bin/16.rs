use std::collections::{HashMap, HashSet};

advent_of_code::solution!(16);

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
enum Direction {
    East,
    West,
    North,
    South,
}

impl Direction {
    fn advancement(&self) -> (isize, isize) {
        match self {
            Self::East => (0, 1),
            Self::North => (-1, 0),
            Self::South => (1, 0),
            Self::West => (0, -1),
        }
    }

    fn rotate_clockwise(&self) -> Direction {
        match self {
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
            Self::North => Self::East,
        }
    }

    fn rotate_counterclockwise(&self) -> Direction {
        match self {
            Self::East => Self::North,
            Self::South => Self::East,
            Self::West => Self::South,
            Self::North => Self::West,
        }
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
struct State {
    x: usize,
    y: usize,
    dir: Direction,
}

impl State {
    fn advanced_theory(&self) -> State {
        State {
            x: (self.x as isize + self.dir.advancement().0)
                .try_into()
                .unwrap(),
            y: (self.y as isize + self.dir.advancement().1)
                .try_into()
                .unwrap(),
            dir: self.dir,
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn visit_another_state(
    maze: &[Vec<char>],
    global_states: &mut HashMap<(isize, isize), isize>,
    visited_states: &HashSet<&State>,
    actions: &[&State],
    current_state: &State,
    current_price: isize,
    end_x: usize,
    end_y: usize,
) -> isize {
    // println!("{global_states:?}");
    *[
        {
            // Advance
            let advanced_theory = current_state.advanced_theory();
            if advanced_theory.x == end_x
                && advanced_theory.y == end_y
                && *global_states
                    .get(&(advanced_theory.x as isize, advanced_theory.y as isize))
                    .unwrap_or(&isize::MAX)
                    > current_price + 1
            {
                global_states.insert(
                    (advanced_theory.x as isize, advanced_theory.y as isize),
                    current_price + 1,
                );
                current_price + 1
            } else if maze[advanced_theory.x][advanced_theory.y] == '.'
                && !visited_states.contains(&advanced_theory)
                && *global_states
                    .get(&(advanced_theory.x as isize, advanced_theory.y as isize))
                    .unwrap_or(&isize::MAX)
                    > current_price + 1
            {
                let new_price = current_price + 1;
                let new_actions = &mut actions.to_vec();
                new_actions.push(&advanced_theory);
                let mut new_visited_states = HashSet::new();
                new_visited_states.extend(&visited_states.iter().copied().collect::<Vec<_>>());
                new_visited_states.insert(&advanced_theory);
                global_states.insert(
                    (advanced_theory.x as isize, advanced_theory.y as isize),
                    new_price,
                );

                visit_another_state(
                    maze,
                    global_states,
                    &new_visited_states,
                    new_actions,
                    &advanced_theory,
                    current_price + 1,
                    end_x,
                    end_y,
                )
            } else {
                isize::MAX
            }
        },
        {
            // Rotate clockwise and advance
            let mut current_state = *current_state;
            let current_price = current_price + 1000;
            current_state.dir = current_state.dir.rotate_clockwise();

            let advanced_theory = current_state.advanced_theory();
            if advanced_theory.x == end_x
                && advanced_theory.y == end_y
                && *global_states
                    .get(&(advanced_theory.x as isize, advanced_theory.y as isize))
                    .unwrap_or(&isize::MAX)
                    > current_price + 1
            {
                global_states.insert(
                    (advanced_theory.x as isize, advanced_theory.y as isize),
                    current_price + 1,
                );
                current_price + 1
            } else if maze[advanced_theory.x][advanced_theory.y] == '.'
                && !visited_states.contains(&advanced_theory)
                && *global_states
                    .get(&(advanced_theory.x as isize, advanced_theory.y as isize))
                    .unwrap_or(&isize::MAX)
                    > current_price + 1
            {
                let new_price = current_price + 1;
                let new_actions = &mut actions.to_vec();
                new_actions.push(&advanced_theory);
                let mut new_visited_states = HashSet::new();
                new_visited_states.extend(&visited_states.iter().copied().collect::<Vec<_>>());
                new_visited_states.insert(&advanced_theory);
                global_states.insert(
                    (advanced_theory.x as isize, advanced_theory.y as isize),
                    new_price,
                );

                visit_another_state(
                    maze,
                    global_states,
                    &new_visited_states,
                    new_actions,
                    &advanced_theory,
                    current_price + 1,
                    end_x,
                    end_y,
                )
            } else {
                isize::MAX
            }
        },
        {
            // Rotate counterclockwise and advance
            let mut current_state = *current_state;
            let current_price = current_price + 1000;
            current_state.dir = current_state.dir.rotate_counterclockwise();

            let advanced_theory = current_state.advanced_theory();
            if advanced_theory.x == end_x
                && advanced_theory.y == end_y
                && *global_states
                    .get(&(advanced_theory.x as isize, advanced_theory.y as isize))
                    .unwrap_or(&isize::MAX)
                    > current_price + 1
            {
                global_states.insert(
                    (advanced_theory.x as isize, advanced_theory.y as isize),
                    current_price + 1,
                );
                current_price + 1
            } else if maze[advanced_theory.x][advanced_theory.y] == '.'
                && !visited_states.contains(&advanced_theory)
                && *global_states
                    .get(&(advanced_theory.x as isize, advanced_theory.y as isize))
                    .unwrap_or(&isize::MAX)
                    > current_price + 1
            {
                let new_price = current_price + 1;
                let new_actions = &mut actions.to_vec();
                new_actions.push(&advanced_theory);
                let mut new_visited_states = HashSet::new();
                new_visited_states.extend(&visited_states.iter().copied().collect::<Vec<_>>());
                new_visited_states.insert(&advanced_theory);
                global_states.insert(
                    (advanced_theory.x as isize, advanced_theory.y as isize),
                    new_price,
                );

                visit_another_state(
                    maze,
                    global_states,
                    &new_visited_states,
                    new_actions,
                    &advanced_theory,
                    current_price + 1,
                    end_x,
                    end_y,
                )
            } else {
                isize::MAX
            }
        },
    ]
    .iter()
    .min()
    .unwrap()
}

pub fn part_one(input: &str) -> Option<isize> {
    let maze = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut start_x = 0;
    let mut start_y = 0;
    let mut end_x = 0;
    let mut end_y = 0;
    for (i, line) in maze.iter().enumerate() {
        for (j, char) in line.iter().enumerate() {
            if *char == 'S' {
                start_x = i as isize;
                start_y = j as isize;
            }
            if *char == 'E' {
                end_x = i as isize;
                end_y = j as isize;
            }
        }
    }

    let initial_state: State = State {
        x: start_x.try_into().unwrap(),
        y: start_y.try_into().unwrap(),
        dir: Direction::East,
    };

    let mut global_states: HashMap<(isize, isize), isize> = HashMap::new();
    global_states.insert((initial_state.x as isize, initial_state.y as isize), 0);

    let mut visited_states: HashSet<&State> = HashSet::new();
    visited_states.insert(&initial_state);

    let actions: Vec<&State> = vec![];

    let current_price = 0;

    let shortest_price = visit_another_state(
        &maze,
        &mut global_states,
        &visited_states,
        &actions,
        &initial_state,
        current_price,
        end_x.try_into().unwrap(),
        end_y.try_into().unwrap(),
    );

    Some(shortest_price)
}

fn count_optimal_locations(
    global_states: &HashMap<(isize, isize), isize>,
    end_x: isize,
    end_y: isize,
) -> usize {
    let mut optimal_tiles: HashSet<(isize, isize)> = HashSet::new();

    optimal_tiles.insert((end_x, end_y));

    let final_min = global_states[&(end_x, end_y)];

    let curr_state_south: State = State {
        x: end_x as usize,
        y: end_y as usize,
        dir: Direction::South,
    };
    let curr_state_west: State = State {
        x: end_x as usize,
        y: end_y as usize,
        dir: Direction::West,
    };

    explore_neighbors(
        global_states,
        &mut optimal_tiles,
        &curr_state_south,
        final_min,
    );
    explore_neighbors(
        global_states,
        &mut optimal_tiles,
        &curr_state_west,
        final_min,
    );

    // temp
    optimal_tiles.len()
}

fn explore_neighbors(
    global_states: &HashMap<(isize, isize), isize>,
    optimal_tiles: &mut HashSet<(isize, isize)>,
    curr_state: &State,
    current_min: isize,
) {
    // Advance
    let advanced_state = curr_state.advanced_theory();
    if *global_states
        .get(&(advanced_state.x as isize, advanced_state.y as isize))
        .unwrap_or(&isize::MAX)
        == current_min - 1
    {
        optimal_tiles.insert((advanced_state.x as isize, advanced_state.y as isize));
        explore_neighbors(
            global_states,
            optimal_tiles,
            &advanced_state,
            current_min - 1,
        );
    }

    if *global_states
        .get(&(advanced_state.x as isize, advanced_state.y as isize))
        .unwrap_or(&isize::MAX)
        == current_min - 1001
    {
        optimal_tiles.insert((advanced_state.x as isize, advanced_state.y as isize));
        explore_neighbors(
            global_states,
            optimal_tiles,
            &advanced_state,
            current_min - 1,
        );
        let rotate_cw = State {
            x: advanced_state.x,
            y: advanced_state.y,
            dir: curr_state.dir.rotate_clockwise(),
        };
        explore_neighbors(global_states, optimal_tiles, &rotate_cw, current_min - 1001);
        let rotate_ccw = State {
            x: advanced_state.x,
            y: advanced_state.y,
            dir: curr_state.dir.rotate_counterclockwise(),
        };
        explore_neighbors(
            global_states,
            optimal_tiles,
            &rotate_ccw,
            current_min - 1001,
        );
    }
}

pub fn part_two(input: &str) -> Option<isize> {
    let maze = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut start_x = 0;
    let mut start_y = 0;
    let mut end_x = 0;
    let mut end_y = 0;
    for (i, line) in maze.iter().enumerate() {
        for (j, char) in line.iter().enumerate() {
            if *char == 'S' {
                start_x = i as isize;
                start_y = j as isize;
            }
            if *char == 'E' {
                end_x = i as isize;
                end_y = j as isize;
            }
        }
    }

    let initial_state: State = State {
        x: start_x.try_into().unwrap(),
        y: start_y.try_into().unwrap(),
        dir: Direction::East,
    };

    let mut global_states: HashMap<(isize, isize), isize> = HashMap::new();
    global_states.insert((initial_state.x as isize, initial_state.y as isize), 0);

    let mut visited_states: HashSet<&State> = HashSet::new();
    visited_states.insert(&initial_state);

    let actions: Vec<&State> = vec![];

    let current_price = 0;

    let _ = visit_another_state(
        &maze,
        &mut global_states,
        &visited_states,
        &actions,
        &initial_state,
        current_price,
        end_x.try_into().unwrap(),
        end_y.try_into().unwrap(),
    );

    // let mut minimums: Vec<Vec<usize>> = vec![];
    // for (i, line) in maze.iter().enumerate() {
    //     minimums.push(vec![]);
    //     for (j, _) in line.iter().enumerate() {
    //         let val = *global_states.get(&(i as isize, j as isize)).unwrap_or(&0);
    //         minimums[i].push(val as usize);
    //     }
    // }
    // println!(
    //     "{}",
    //     minimums
    //         .iter()
    //         .map(|line| line.iter().map(|u| u.to_string()).join("\t"))
    //         .join("\n")
    // );

    let count_of_optimal_locations = count_optimal_locations(&global_states, end_x, end_y);
    Some(count_of_optimal_locations as isize)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7036));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(45));
    }
}
