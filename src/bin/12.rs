use std::{collections::HashSet, usize};

advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<u32> {
    let grid = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let regions = get_regions(&grid);

    let result = regions
        .iter()
        .map(|r| calculate_region_cost(&grid, r))
        .sum();
    Some(result)
}

fn calculate_region_cost(grid: &[Vec<char>], r: &[(usize, usize)]) -> u32 {
    let area = r.len() as u32;
    let perimeter: u32 = r
        .iter()
        .map(|(x, y)| {
            let above = if y > &0 { Some((*x, y - 1)) } else { None };

            let below = if y < &(grid.len() - 1) {
                Some((*x, y + 1))
            } else {
                None
            };

            let left = if x > &0 { Some((x - 1, *y)) } else { None };

            let right = if x < &(grid[0].len() - 1) {
                Some((x + 1, *y))
            } else {
                None
            };

            vec![above, below, left, right]
                .iter()
                .filter(|c| match c {
                    Some(coord) => {
                        if r.contains(coord) {
                            return false;
                        }
                        return true;
                    }
                    None => true,
                })
                .count() as u32
        })
        .sum();

    perimeter * area
}

fn get_regions(grid: &[Vec<char>]) -> Vec<Vec<(usize, usize)>> {
    let mut already_visited_coords = HashSet::new();
    let mut regions = Vec::new();
    grid.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, plant)| {
            let coords = (x, y);
            if already_visited_coords.contains(&coords) {
                return;
            }

            already_visited_coords.insert(coords);

            let mut region =
                get_surrounding_coords_in_region(&grid, &mut already_visited_coords, coords, plant);
            region.push(coords);
            regions.push(region);
        });
    });

    regions
}

fn get_surrounding_coords_in_region(
    grid: &&[Vec<char>],
    already_visited_coords: &mut HashSet<(usize, usize)>,
    coords: (usize, usize),
    plant: &char,
) -> Vec<(usize, usize)> {
    //above, below, left, right
    let above = if coords.1 > 0 {
        Some((coords.0, coords.1 - 1))
    } else {
        None
    };

    let below = if coords.1 < grid.len() - 1 {
        Some((coords.0, coords.1 + 1))
    } else {
        None
    };

    let left = if coords.0 > 0 {
        Some((coords.0 - 1, coords.1))
    } else {
        None
    };

    let right = if coords.0 < grid[0].len() - 1 {
        Some((coords.0 + 1, coords.1))
    } else {
        None
    };

    vec![above, below, left, right]
        .iter()
        .filter_map(|c| {
            c.and_then(|coord| {
                if already_visited_coords.contains(&coord) {
                    return None;
                }

                if grid[coord.1][coord.0] == *plant {
                    already_visited_coords.insert(coord);
                    let surrounding = get_surrounding_coords_in_region(
                        grid,
                        already_visited_coords,
                        coord,
                        plant,
                    );
                    let all = [coord]
                        .into_iter()
                        .chain(surrounding.into_iter())
                        .collect::<Vec<(usize, usize)>>();
                    return Some(all);
                }

                None
            })
        })
        .flatten()
        .collect::<Vec<(usize, usize)>>()
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let regions = get_regions(&grid);

    let result = regions
        .iter()
        .map(|r| calculate_part_1_region_cost(&grid, r))
        .sum();
    Some(result)
}

fn calculate_part_1_region_cost(grid: &[Vec<char>], r: &[(usize, usize)]) -> u32 {
    let area = r.len() as u32;
    let number_of_corners: u32 = r
        .iter()
        .map(|(x, y)| {
            let above = if y > &0 { Some((*x, y - 1)) } else { None };

            let above_in_region: bool = r.contains(&above.unwrap_or((usize::MAX, usize::MAX)));

            let below = if y < &(grid.len() - 1) {
                Some((*x, y + 1))
            } else {
                None
            };
            let below_in_region: bool = r.contains(&below.unwrap_or((usize::MAX, usize::MAX)));

            let left = if x > &0 { Some((x - 1, *y)) } else { None };
            let left_in_region: bool = r.contains(&left.unwrap_or((usize::MAX, usize::MAX)));

            let right = if x < &(grid[0].len() - 1) {
                Some((x + 1, *y))
            } else {
                None
            };
            let right_in_region: bool = r.contains(&right.unwrap_or((usize::MAX, usize::MAX)));

            let above_left = if y > &0 && x > &0 {
                Some((x - 1, y - 1))
            } else {
                None
            };
            let above_left_in_region: bool =
                r.contains(&above_left.unwrap_or((usize::MAX, usize::MAX)));

            let above_right = if y > &0 && x < &(grid[0].len() - 1) {
                Some((x + 1, y - 1))
            } else {
                None
            };
            let above_right_in_region: bool =
                r.contains(&above_right.unwrap_or((usize::MAX, usize::MAX)));

            let below_left = if y < &(grid.len() - 1) && x > &0 {
                Some((x - 1, y + 1))
            } else {
                None
            };
            let below_left_in_region: bool =
                r.contains(&below_left.unwrap_or((usize::MAX, usize::MAX)));

            let below_right = if y < &(grid.len() - 1) && x < &(grid[0].len() - 1) {
                Some((x + 1, y + 1))
            } else {
                None
            };
            let below_right_in_region: bool =
                r.contains(&below_right.unwrap_or((usize::MAX, usize::MAX)));

            let mut corners = 0;

            if left_in_region && above_in_region && !above_left_in_region {
                corners += 1;
            }
            if left_in_region && below_in_region && !below_left_in_region {
                corners += 1;
            }
            if right_in_region && above_in_region && !above_right_in_region {
                corners += 1;
            }
            if right_in_region && below_in_region && !below_right_in_region {
                corners += 1;
            }

            if !above_in_region && !left_in_region && !above_left_in_region {
                corners += 1;
            }

            if !above_in_region && !right_in_region && !above_right_in_region {
                corners += 1;
            }

            if !below_in_region && !left_in_region && !below_left_in_region {
                corners += 1;
            }

            if !below_in_region && !right_in_region && !below_right_in_region {
                corners += 1;
            }

            if !right_in_region && !below_in_region && below_right_in_region {
                corners += 1;
            }

            if !right_in_region && !above_in_region && above_right_in_region {
                corners += 1;
            }

            if !left_in_region && !below_in_region && below_left_in_region {
                corners += 1;
            }

            if !left_in_region && !above_in_region && above_left_in_region {
                corners += 1;
            }

            corners
        })
        .sum();

    number_of_corners * area
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
