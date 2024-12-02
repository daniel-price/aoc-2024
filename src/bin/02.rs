advent_of_code::solution!(2);

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|level| level.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let reports = parse_input(input);
    let result = reports.iter().filter(|report| is_safe(report)).count() as u32;
    Some(result)
}

fn is_safe(report: &Vec<i32>) -> bool {
    let mut sorted_report = report.clone();
    sorted_report.sort();
    let mut reverse_sorted_report = sorted_report.clone();
    reverse_sorted_report.reverse();
    if sorted_report != *report && reverse_sorted_report != *report {
        return false;
    }

    let safe = report
        .iter()
        .enumerate()
        .filter(|(i, number)| {
            let next_number = report.get(i + 1);

            match next_number {
                Some(prev_number) => {
                    let difference = (*number - prev_number).abs();
                    if difference < 1 || difference > 3 {
                        return false;
                    }
                }
                None => (),
            }

            true
        })
        .count()
        == report.len();
    safe
}

pub fn part_two(input: &str) -> Option<u32> {
    let reports = parse_input(input);
    let result = reports
        .iter()
        .filter(|report| {
            if is_safe(report) {
                return true;
            }
            for (i, _) in report.iter().enumerate() {
                let mut candidate_report: Vec<i32> = report.to_vec();
                candidate_report.remove(i);
                if is_safe(&candidate_report) {
                    return true;
                }
            }
            false
        })
        .count() as u32;
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
