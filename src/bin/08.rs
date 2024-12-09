use std::collections::HashSet;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let mut antinodes = HashSet::new();

    input.lines().enumerate().for_each(|(y1, l1)| {
        l1.split("")
            .filter(|c| c != &"")
            .enumerate()
            .for_each(|(x1, c1)| {
                input.lines().enumerate().for_each(|(y2, l2)| {
                    l2.split("")
                        .filter(|c| c != &"")
                        .enumerate()
                        .for_each(|(x2, c2)| {
                            if y1 == y2 && x1 == x2 {
                                return;
                            }

                            if c1 == "" || c2 == "" || c1 == "." || c2 == "." || c1 != c2 {
                                return;
                            }

                            let x_diff = x1 as i32 - x2 as i32;
                            let y_diff = y1 as i32 - y2 as i32;

                            let new_y = y1 as i32 + y_diff;
                            let new_x = x1 as i32 + x_diff;

                            if new_y < 0
                                || new_y >= input.lines().count() as i32
                                || new_x < 0
                                || new_x >= l1.len() as i32
                            {
                                return;
                            }

                            antinodes.insert((new_x, new_y));
                        });
                });
            });
    });

    Some(antinodes.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut antinodes = HashSet::new();

    input.lines().enumerate().for_each(|(y1, l1)| {
        l1.split("")
            .filter(|c| c != &"")
            .enumerate()
            .for_each(|(x1, c1)| {
                input.lines().enumerate().for_each(|(y2, l2)| {
                    l2.split("")
                        .filter(|c| c != &"")
                        .enumerate()
                        .for_each(|(x2, c2)| {
                            if y1 == y2 && x1 == x2 {
                                return;
                            }

                            if c1 == "" || c2 == "" || c1 == "." || c2 == "." || c1 != c2 {
                                return;
                            }

                            let x_diff = x1 as i32 - x2 as i32;
                            let y_diff = y1 as i32 - y2 as i32;

                            let mut new_y = y1 as i32;
                            let mut new_x = x1 as i32;

                            loop {
                                if new_y < 0
                                    || new_y >= input.lines().count() as i32
                                    || new_x < 0
                                    || new_x >= l1.len() as i32
                                {
                                    break;
                                }

                                antinodes.insert((new_x, new_y));

                                new_y = new_y + y_diff;
                                new_x = new_x + x_diff;
                            }
                        });
                });
            });
    });

    Some(antinodes.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
