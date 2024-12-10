use std::collections::HashSet;

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<i32>> = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as i32).collect())
        .collect();

    let result: u32 = grid
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, number)| {
                    if number != &0 {
                        return 0;
                    }

                    let mut reachable_nine_coordinates = HashSet::new();

                    find_reachable_nine_coordinates(
                        &grid,
                        x as i32,
                        y as i32,
                        *number,
                        &mut reachable_nine_coordinates,
                    );

                    reachable_nine_coordinates.len() as u32
                })
                .sum::<u32>()
        })
        .sum();

    Some(result)
}

fn find_reachable_nine_coordinates(
    grid: &Vec<Vec<i32>>,
    x: i32,
    y: i32,
    number: i32,
    reachable_nine_coordinates: &mut HashSet<(i32, i32)>,
) -> () {
    if number == 9 {
        reachable_nine_coordinates.insert((x, y));
        return;
    }

    [(0, -1 as i32), (0, 1), (-1 as i32, 0), (1, 0)]
        .iter()
        .for_each(|(x_diff, y_diff)| {
            let new_x = x + (*x_diff);
            let new_y = y + *y_diff;

            let next_cell = grid
                .get(new_y as usize)
                .and_then(|row| row.get(new_x as usize));

            if next_cell == Some(&(number + 1)) {
                find_reachable_nine_coordinates(
                    grid,
                    new_x,
                    new_y,
                    number + 1,
                    reachable_nine_coordinates,
                );
            }
        })
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: Vec<Vec<i32>> = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as i32).collect())
        .collect();

    let result: u32 = grid
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, number)| {
                    if number != &0 {
                        return 0;
                    }

                    let mut count = 0;

                    find_distinct_paths(&grid, x as i32, y as i32, *number, &mut count);

                    count as u32
                })
                .sum::<u32>()
        })
        .sum();

    Some(result)
}

fn find_distinct_paths(grid: &Vec<Vec<i32>>, x: i32, y: i32, number: i32, count: &mut i32) -> () {
    if number == 9 {
        *count += 1;
        return;
    }

    [(0, -1 as i32), (0, 1), (-1 as i32, 0), (1, 0)]
        .iter()
        .for_each(|(x_diff, y_diff)| {
            let new_x = x + (*x_diff);
            let new_y = y + *y_diff;

            let next_cell = grid
                .get(new_y as usize)
                .and_then(|row| row.get(new_x as usize));

            if next_cell == Some(&(number + 1)) {
                find_distinct_paths(grid, new_x, new_y, number + 1, count);
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
