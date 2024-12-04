use regex::Regex;
advent_of_code::solution!(4);

fn vertical_lines(input: &str, width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    for i in 0..width {
        let line = input
            .lines()
            .flat_map(|line| {
                line.split("")
                    .enumerate()
                    .filter(|(j, _)| j % width == i)
                    .map(|(_, letter)| letter)
                    .collect::<Vec<&str>>()
            })
            .collect::<Vec<&str>>()
            .join("");

        lines.push(line);
    }
    lines
}

fn get_diagonals(grid: Vec<Vec<char>>, bltr: bool) -> Vec<String> {
    let dim = grid.len();
    assert_eq!(dim, grid[0].len());
    let mut return_grid = vec![Vec::new(); 2 * dim - 1];
    for row in 0..dim {
        for col in 0..dim {
            if bltr {
                return_grid[row + col].push(grid[col][row]);
            } else {
                let a = col + (dim - 1) - row;
                return_grid[a].push(grid[row][col]);
            }
        }
    }
    return_grid
        .iter()
        .map(|line| line.iter().collect())
        .collect()
}

fn diagonal_lines(input: &str) -> Vec<String> {
    let mut lines = Vec::new();

    let grid = input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();

    lines.append(&mut get_diagonals(grid.clone(), true));
    lines.append(&mut get_diagonals(grid.clone(), false));

    lines
}

pub fn part_one(input: &str) -> Option<u32> {
    let width = input.lines().next()?.len();

    let mut horizontal_lines = input
        .lines()
        .map(|line| line.to_string())
        .collect::<Vec<String>>();
    let mut vertical_lines = vertical_lines(input, width);
    let mut diagonal_lines = diagonal_lines(input);

    let mut candidate_lines = Vec::new();
    candidate_lines.append(&mut horizontal_lines);
    candidate_lines.append(&mut vertical_lines);
    candidate_lines.append(&mut diagonal_lines);

    let re_forward = Regex::new(r"XMAS").unwrap();
    let re_backward = Regex::new(r"SAMX").unwrap();
    let result = candidate_lines
        .iter()
        .map(|line| {
            let count_forward = re_forward.find_iter(line).count() as u32;
            let count_backward = re_backward.find_iter(line).count() as u32;
            count_forward + count_backward
        })
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut count = 0;
    grid.iter().enumerate().for_each(|(i, row)| {
        row.iter().enumerate().for_each(|(j, letter)| {
            if *letter != 'A' {
                return;
            }
            if i < 1 || j < 1 {
                return;
            }
            if (i == grid.len() - 1) || (j == grid.len() - 1) {
                return;
            }

            let row_above = grid.get(i - 1);
            let row_below = grid.get(i + 1);

            match (row_above, row_below) {
                (Some(row_above), Some(row_below)) => {
                    let top_left = row_above.get(j - 1);
                    let top_right = row_above.get(j + 1);
                    let bottom_left = row_below.get(j - 1);
                    let bottom_right = row_below.get(j + 1);

                    match (top_left, top_right, bottom_left, bottom_right) {
                        (
                            Some(top_left),
                            Some(top_right),
                            Some(bottom_left),
                            Some(bottom_right),
                        ) => {
                            let first_mas = (top_left == &'M' && bottom_right == &'S')
                                || (top_left == &'S' && bottom_right == &'M');
                            let second_mas = (top_right == &'M' && bottom_left == &'S')
                                || (top_right == &'S' && bottom_left == &'M');
                            if first_mas && second_mas {
                                count += 1;
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        });
    });

    Some(count)
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
