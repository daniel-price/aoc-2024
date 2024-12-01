advent_of_code::solution!(1);

#[derive(Debug)]
struct Lists {
    list1: Vec<i32>,
    list2: Vec<i32>,
}

fn parse_input(input: &str) -> Lists {
    let mut lists = Lists {
        list1: vec![],
        list2: vec![],
    };
    input.lines().for_each(|line| {
        let numbers = line.split_whitespace().collect::<Vec<&str>>();
        lists.list1.push(numbers[0].parse::<i32>().unwrap());
        lists.list2.push(numbers[1].parse::<i32>().unwrap());
    });
    lists
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut lists = parse_input(input);
    lists.list1.sort();
    lists.list2.sort();

    let result = lists
        .list1
        .iter()
        .enumerate()
        .map(|(i, list1number)| {
            let list2number = lists.list2[i];
            let difference = (list1number - list2number).abs();
            difference
        })
        .sum::<i32>();
    Some(result as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lists = parse_input(input);
    let result = lists
        .list1
        .iter()
        .map(|list1number| {
            let count = lists
                .list2
                .iter()
                .filter(|list2number| *list2number == list1number)
                .count();
            list1number * count as i32
        })
        .sum::<i32>();

    Some(result as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
