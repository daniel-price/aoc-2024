use core::num;
use regex::Regex;
use std::collections::HashMap;
advent_of_code::solution!(14);

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Velocity {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Robot {
    position: Position,
    velocity: Velocity,
}

fn parse_input(input: &str) -> Vec<Robot> {
    input
        .lines()
        .map(|line| {
            let re = Regex::new(r"p=(\d+),(\d+) v=(.*),(.*)").unwrap();
            let captures = re.captures(line).unwrap();
            let position_x = captures[1].parse::<i32>().unwrap();
            let position_y = captures[2].parse::<i32>().unwrap();
            let velocity_x = captures[3].parse::<i32>().unwrap();
            let velocity_y = captures[4].parse::<i32>().unwrap();

            Robot {
                position: Position {
                    x: position_x,
                    y: position_y,
                },
                velocity: Velocity {
                    x: velocity_x,
                    y: velocity_y,
                },
            }
        })
        .collect()
}

fn part_one_with_grid(input: &str, width: i32, height: i32) -> Option<u32> {
    let mut robots = parse_input(input);
    let num_seconds = 100;
    for i in 0..num_seconds {
        println!("{}", i);
        robots.iter_mut().for_each(|robot| {
            robot.position.x += robot.velocity.x;
            robot.position.y += robot.velocity.y;

            if robot.position.x < 0 {
                robot.position.x += width;
            }

            if robot.position.y < 0 {
                robot.position.y += height;
            }

            if robot.position.x >= width {
                robot.position.x -= width;
            }

            if robot.position.y >= height {
                robot.position.y -= height;
            }
        });
    }

    let mut quadrant_counts: HashMap<i64, i64> = HashMap::new();
    quadrant_counts.insert(1, 0);
    quadrant_counts.insert(2, 0);
    quadrant_counts.insert(3, 0);
    quadrant_counts.insert(4, 0);

    robots.iter().for_each(|robot| {
        println!("({},{})", robot.position.x, robot.position.y);
        if robot.position.x < width / 2 && robot.position.y < height / 2 {
            println!("1");
            *quadrant_counts.get_mut(&1).unwrap() += 1;
        } else if robot.position.x > width / 2 && robot.position.y < height / 2 {
            println!("2");
            *quadrant_counts.get_mut(&2).unwrap() += 1;
        } else if robot.position.x < width / 2 && robot.position.y > height / 2 {
            println!("3");
            *quadrant_counts.get_mut(&3).unwrap() += 1;
        } else if robot.position.x > width / 2 && robot.position.y > height / 2 {
            println!("4");
            *quadrant_counts.get_mut(&4).unwrap() += 1;
        } else {
            println!("none");
        }
    });

    println!("{:?}", quadrant_counts);
    let result = quadrant_counts
        .iter()
        .map(|(_, count)| *count as u32)
        .product();
    Some(result)
}

pub fn part_one(input: &str) -> Option<u32> {
    let width = 101;
    let height = 103;
    part_one_with_grid(input, width, height)
}

pub fn part_two(input: &str) -> Option<u32> {
    let width = 101;
    let height = 103;
    let mut robots = parse_input(input);
    let mut num_seconds = 0;
    loop {
        num_seconds += 1;
        robots.iter_mut().for_each(|robot| {
            robot.position.x += robot.velocity.x;
            robot.position.y += robot.velocity.y;

            if robot.position.x < 0 {
                robot.position.x += width;
            }

            if robot.position.y < 0 {
                robot.position.y += height;
            }

            if robot.position.x >= width {
                robot.position.x -= width;
            }

            if robot.position.y >= height {
                robot.position.y -= height;
            }
        });

        let mut count_by_coordinate: HashMap<(i32, i32), i64> = HashMap::new();
        robots.iter().for_each(|robot| {
            count_by_coordinate
                .entry((robot.position.x, robot.position.y))
                .and_modify(|e| *e += 1)
                .or_insert(1);
        });

        if count_by_coordinate
            .iter()
            .find(|(x, count)| *count > &1)
            .is_none()
        {
            break;
        }
    }

    for y in 0..height {
        for x in 0..width {
            if robots
                .iter()
                .find(|robot| robot.position.x == x && robot.position.y == y)
                .is_some()
            {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }

    Some(num_seconds)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result =
            part_one_with_grid(&advent_of_code::template::read_file("examples", DAY), 11, 7);
        assert_eq!(result, Some(12));
    }
}
