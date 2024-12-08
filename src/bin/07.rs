advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let result = input
        .lines()
        .filter_map(|line| {
            let parts = line.split(":").collect::<Vec<&str>>();
            let test_value = parts[0].parse::<u64>().unwrap();
            let mut numbers: Vec<u64> = parts[1]
                .split_whitespace()
                .map(|i| i.parse::<u64>().unwrap())
                .collect();

            numbers.reverse();
            let first_num = numbers.pop().unwrap();
            if passes_part_one(test_value, vec![first_num], numbers) {
                return Some(test_value);
            }

            None
        })
        .sum();
    Some(result)
}

fn passes_part_one(test_value: u64, results: Vec<u64>, numbers: Vec<u64>) -> bool {
    let mut cloned_numbers = numbers.clone();
    let cloned_results = results.clone();
    let num = cloned_numbers.pop();
    if num == None {
        return cloned_results.iter().find(|&x| *x == test_value) != None;
    }

    let new_results: Vec<u64> = cloned_results
        .iter()
        .flat_map(|result| {
            let sum = result + num.unwrap();
            let product = result * num.unwrap();
            vec![sum, product]
        })
        .collect();

    let new_results_smaller = new_results
        .iter()
        .filter_map(|&x| {
            if x <= test_value {
                return Some(x);
            }
            None
        })
        .collect();

    passes_part_one(test_value, new_results_smaller, cloned_numbers)
}

pub fn part_two(input: &str) -> Option<u64> {
    let result = input
        .lines()
        .filter_map(|line| {
            let parts = line.split(":").collect::<Vec<&str>>();
            let test_value = parts[0].parse::<u64>().unwrap();
            let mut numbers: Vec<u64> = parts[1]
                .split_whitespace()
                .map(|i| i.parse::<u64>().unwrap())
                .collect();

            numbers.reverse();
            let first_num = numbers.pop().unwrap();
            if passes_part_two(test_value, vec![first_num], numbers) {
                return Some(test_value);
            }

            None
        })
        .sum();
    Some(result)
}

fn passes_part_two(test_value: u64, results: Vec<u64>, numbers: Vec<u64>) -> bool {
    let mut cloned_numbers = numbers.clone();
    let cloned_results = results.clone();
    let num = cloned_numbers.pop();
    if num == None {
        return cloned_results.iter().find(|&x| *x == test_value) != None;
    }

    let new_results: Vec<u64> = cloned_results
        .iter()
        .flat_map(|result| {
            let sum = result + num.unwrap();
            let product = result * num.unwrap();
            let concat = format!("{}{}", result, num.unwrap())
                .parse::<u64>()
                .unwrap();

            vec![sum, product, concat]
        })
        .collect();

    let new_results_smaller = new_results
        .iter()
        .filter_map(|&x| {
            if x <= test_value {
                return Some(x);
            }
            None
        })
        .collect();

    passes_part_two(test_value, new_results_smaller, cloned_numbers)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
