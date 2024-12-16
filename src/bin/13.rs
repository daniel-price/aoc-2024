use regex::Regex;
advent_of_code::solution!(13);

#[derive(Debug)]
struct Button {
    x_increase: i64,
    y_increase: i64,
}

#[derive(Debug)]
struct Coordinate {
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct Machine {
    button_a: Button,
    button_b: Button,
    prize: Coordinate,
}

fn parse_input(input: &str) -> Vec<Machine> {
    input
        .split("\n\n")
        .map(|m| {
            let lines = m.lines().collect::<Vec<&str>>();

            let re = Regex::new(r".*X.(\d+), Y.(\d+)").unwrap();
            let a_captures = re.captures(lines[0]).unwrap();
            let b_captures = re.captures(lines[1]).unwrap();
            let prize_captures = re.captures(lines[2]).unwrap();

            let a_x_increase = a_captures[1].parse::<i64>().unwrap();
            let a_y_increase = a_captures[2].parse::<i64>().unwrap();

            let b_x_increase = b_captures[1].parse::<i64>().unwrap();
            let b_y_increase = b_captures[2].parse::<i64>().unwrap();

            let prize_x = prize_captures[1].parse::<i64>().unwrap();
            let prize_y = prize_captures[2].parse::<i64>().unwrap();

            Machine {
                button_a: Button {
                    x_increase: a_x_increase,
                    y_increase: a_y_increase,
                },
                button_b: Button {
                    x_increase: b_x_increase,
                    y_increase: b_y_increase,
                },
                prize: Coordinate {
                    x: prize_x,
                    y: prize_y,
                },
            }
        })
        .collect::<Vec<Machine>>()
}

pub fn part_one(input: &str) -> Option<i64> {
    let machines = parse_input(input);
    solve(&machines)
}

fn solve(machines: &[Machine]) -> Option<i64> {
    let result = machines
        .iter()
        .map(|m| {
            let b_count = (m.button_a.x_increase * m.prize.y - m.button_a.y_increase * m.prize.x)
                / (m.button_b.y_increase * m.button_a.x_increase
                    - m.button_b.x_increase * m.button_a.y_increase);

            let a_count = (m.prize.x - m.button_b.x_increase * b_count) / m.button_a.x_increase;

            if (m.button_a.x_increase * a_count + m.button_b.x_increase * b_count) != m.prize.x
                || (m.button_a.y_increase * a_count + m.button_b.y_increase * b_count) != m.prize.y
            {
                return 0;
            }

            3 * a_count as i64 + b_count as i64
        })
        .sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut machines = parse_input(input);
    machines.iter_mut().for_each(|m| {
        m.prize.x += 10000000000000;
        m.prize.y += 10000000000000;
    });

    solve(&machines)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }
}
