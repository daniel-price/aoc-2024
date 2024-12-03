use regex::Regex;
advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let result = re
        .find_iter(input)
        .map(|mat| {
            let mul = mat.as_str();
            let re_nums = Regex::new(r"\d+").unwrap();
            let product: u32 = re_nums
                .find_iter(mul)
                .map(|m| m.as_str().parse::<u32>().unwrap())
                .product();

            product
        })
        .sum::<u32>();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"(do\(\))|(don't\(\))|(mul\(\d+,\d+\))").unwrap();
    let mut instructions_enabled = true;
    let result = re
        .find_iter(input)
        .map(|mat| {
            let mul = mat.as_str();
            match mul {
                "do()" => {
                    instructions_enabled = true;
                    return 0;
                }
                "don't()" => {
                    instructions_enabled = false;
                    return 0;
                }
                _ => {
                    if !instructions_enabled {
                        return 0;
                    }
                }
            }
            let re_nums = Regex::new(r"\d+").unwrap();
            let product: u32 = re_nums
                .find_iter(mul)
                .map(|m| m.as_str().parse::<u32>().unwrap())
                .product();

            product
        })
        .sum::<u32>();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
