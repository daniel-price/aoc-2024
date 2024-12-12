use std::collections::HashMap;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<i64> {
    part_one_and_two(input, 25)
}

pub fn part_two(input: &str) -> Option<i64> {
    part_one_and_two(input, 75)
}

pub fn part_one_and_two(input: &str, blink_count: i32) -> Option<i64> {
    let stones: Vec<i64> = input
        .split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect();

    let mut stones_count: HashMap<i64, i64> = HashMap::new();
    stones.iter().for_each(|stone| {
        stones_count
            .entry(*stone)
            .and_modify(|e| *e += 1)
            .or_insert(1);
    });

    for blink in 0..blink_count {
        stones_count.clone().iter().for_each(|(stone, count)| {
            stones_count.entry(*stone).and_modify(|e| {
                if *e > *count {
                    *e -= count;

                    return;
                }
                *e = 0;
            });
            if stones_count.get(stone) == Some(&0) {
                stones_count.remove(&stone);
            }
            if stone == &0 {
                stones_count
                    .entry(1)
                    .and_modify(|e| *e += *count)
                    .or_insert(*count);
                return;
            }

            let stone_string = stone.to_string();
            let stone_string_len = stone_string.len();

            if stone_string_len % 2 == 0 {
                let first_half = stone_string
                    .chars()
                    .take(stone_string_len / 2)
                    .collect::<String>()
                    .parse::<i64>()
                    .unwrap();

                let second_half = stone_string
                    .chars()
                    .skip(stone_string_len / 2)
                    .collect::<String>()
                    .parse::<i64>()
                    .unwrap();

                stones_count
                    .entry(first_half)
                    .and_modify(|e| *e += *count)
                    .or_insert(*count);

                stones_count
                    .entry(second_half)
                    .and_modify(|e| *e += *count)
                    .or_insert(*count);

                return;
            }

            stones_count
                .entry(stone * 2024)
                .and_modify(|e| *e += count)
                .or_insert(*count);
        })
    }

    let result = stones_count.values().sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }
}
