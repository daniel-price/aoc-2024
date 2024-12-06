use std::collections::HashSet;

advent_of_code::solution!(6);

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid = input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let mut direction_option = None;
    let mut current_coords_option: Option<(usize, usize)> = None;
    grid.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, c)| match c {
            '^' => {
                direction_option = Some(Direction::Up);
                current_coords_option = Some((x, y));
            }
            '>' => {
                direction_option = Some(Direction::Right);
                current_coords_option = Some((x, y));
            }
            'v' => {
                direction_option = Some(Direction::Down);
                current_coords_option = Some((x, y));
            }
            '<' => {
                direction_option = Some(Direction::Left);
                current_coords_option = Some((x, y));
            }
            _ => {}
        })
    });

    let mut direction = direction_option.unwrap();
    let mut current_coords = current_coords_option.unwrap();

    loop {
        let next_coords = match direction {
            Direction::Up => (current_coords.0, current_coords.1 - 1),

            Direction::Right => (current_coords.0 + 1, current_coords.1),

            Direction::Down => (current_coords.0, current_coords.1 + 1),

            Direction::Left => (current_coords.0 - 1, current_coords.1),
        };
        let next_cell = grid.get(next_coords.1).and_then(|o| o.get(next_coords.0));
        match next_cell {
            None => {
                break;
            }

            Some('#') => {
                direction = match direction {
                    Direction::Up => Direction::Right,
                    Direction::Right => Direction::Down,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                }
            }

            _ => {
                current_coords = next_coords;
                grid[next_coords.1][next_coords.0] = 'X';
            }
        }
    }

    let result = grid
        .iter()
        .flatten()
        .filter(|c| ['X', '^', '>', 'v', '<'].contains(*c))
        .count() as u32;
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let original_grid = input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let mut direction_option = None;
    let mut starting_coords_option: Option<(usize, usize)> = None;
    original_grid.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, c)| match c {
            '^' => {
                direction_option = Some(Direction::Up);
                starting_coords_option = Some((x, y));
            }
            '>' => {
                direction_option = Some(Direction::Right);
                starting_coords_option = Some((x, y));
            }
            'v' => {
                direction_option = Some(Direction::Down);
                starting_coords_option = Some((x, y));
            }
            '<' => {
                direction_option = Some(Direction::Left);
                starting_coords_option = Some((x, y));
            }
            _ => {}
        })
    });

    let starting_direction = direction_option.unwrap();
    let starting_coords = starting_coords_option.unwrap();
    let mut direction = starting_direction;
    let mut current_coords = starting_coords;
    let mut number_of_positions = 0;
    let mut already_tested = HashSet::new();
    let mut grid = original_grid.clone();

    loop {
        if current_coords != starting_coords && !already_tested.contains(&current_coords) {
            already_tested.insert(current_coords);
            let mut grid_with_obstruction = original_grid.clone();
            grid_with_obstruction[current_coords.1][current_coords.0] = '#';

            if does_grid_loop(
                grid_with_obstruction.clone(),
                starting_coords,
                starting_direction,
            ) {
                number_of_positions = number_of_positions + 1;
            }
        }
        let next_coords = match direction {
            Direction::Up => (current_coords.0, current_coords.1 - 1),

            Direction::Right => (current_coords.0 + 1, current_coords.1),

            Direction::Down => (current_coords.0, current_coords.1 + 1),

            Direction::Left => (current_coords.0 - 1, current_coords.1),
        };
        let next_cell = grid.get(next_coords.1).and_then(|o| o.get(next_coords.0));
        match next_cell {
            None => {
                break;
            }

            Some('#') => {
                direction = match direction {
                    Direction::Up => Direction::Right,
                    Direction::Right => Direction::Down,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                }
            }

            _ => {
                current_coords = next_coords;
                grid[next_coords.1][next_coords.0] = 'X';
            }
        }
    }

    Some(number_of_positions)
}

fn does_grid_loop(
    mut grid: Vec<Vec<char>>,
    mut current_coords: (usize, usize),
    mut direction: Direction,
) -> bool {
    let mut already_seen = HashSet::new();

    loop {
        if already_seen.contains(&(current_coords, direction)) {
            return true;
        }
        already_seen.insert((current_coords, direction));

        let next_coords = match direction {
            Direction::Up => {
                if current_coords.1 == 0 {
                    return false;
                }
                (current_coords.0, current_coords.1 - 1)
            }

            Direction::Right => (current_coords.0 + 1, current_coords.1),

            Direction::Down => (current_coords.0, current_coords.1 + 1),

            Direction::Left => {
                if current_coords.0 == 0 {
                    return false;
                }
                (current_coords.0 - 1, current_coords.1)
            }
        };

        let next_cell = grid.get(next_coords.1).and_then(|o| o.get(next_coords.0));

        match next_cell {
            None => {
                return false;
            }

            Some('#') => {
                direction = match direction {
                    Direction::Up => Direction::Right,
                    Direction::Right => Direction::Down,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                }
            }

            _ => {
                current_coords = next_coords;
                grid[next_coords.1][next_coords.0] = 'X';
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
