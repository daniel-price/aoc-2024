advent_of_code::solution!(5);

fn is_valid(update: &Vec<i32>, rules: &Vec<Vec<i32>>) -> bool {
    let invalid_rules = rules
        .iter()
        .filter(|r| {
            let first_number = r[0];
            let second_number = r[1];

            if !update.contains(&first_number) || !update.contains(&second_number) {
                //neither number appears - valid
                return false;
            }

            for i in update.iter() {
                if i == &first_number {
                    //first number comes first - valid
                    return false;
                } else if i == &second_number {
                    //second number comes first - invalid
                    return true;
                }
            }

            //neither number appears - valid
            return false;
        })
        .count();
    invalid_rules == 0
}

pub fn part_one(input: &str) -> Option<u32> {
    let pieces = input.split("\n\n").collect::<Vec<&str>>();
    let rules = pieces[0]
        .split("\n")
        .map(|r| {
            r.split("|")
                .map(|r| r.parse().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();
    let updates = pieces[1]
        .split("\n")
        .map(|u| {
            u.split(',')
                .filter(|r| !r.is_empty())
                .map(|r| r.parse().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let valid_updates = updates
        .iter()
        .filter(|u| is_valid(u, &rules))
        .collect::<Vec<&Vec<i32>>>();

    let result = valid_updates
        .iter()
        .filter(|u| !u.is_empty())
        .map(|u| middle_elem(u))
        .sum::<u32>();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let pieces = input.split("\n\n").collect::<Vec<&str>>();
    let rules = pieces[0]
        .split("\n")
        .map(|r| {
            r.split("|")
                .map(|r| r.parse().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();
    let updates = pieces[1]
        .split("\n")
        .map(|u| {
            u.split(',')
                .filter(|r| !r.is_empty())
                .map(|r| r.parse().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let corrected_updates = updates
        .iter()
        .filter_map(|u| {
            if is_valid(u, &rules) {
                return None;
            }

            let mut u_copy = u.clone();
            u_copy.sort_by(|a, b| {
                let rule = rules.iter().find(|r| r.contains(a) && r.contains(b));

                match rule {
                    Some(r) => {
                        if r[0] == *a {
                            return std::cmp::Ordering::Greater;
                        } else {
                            return std::cmp::Ordering::Less;
                        }
                    }
                    None => {
                        return std::cmp::Ordering::Equal;
                    }
                }
            });
            Some(u_copy)
        })
        .collect::<Vec<Vec<i32>>>();

    let result = corrected_updates
        .iter()
        .map(|u| middle_elem(&u))
        .sum::<u32>();
    Some(result)
}

fn middle_elem(u: &Vec<i32>) -> u32 {
    let middle_index = u.len() / 2;
    u[middle_index] as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
