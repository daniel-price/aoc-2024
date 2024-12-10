advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let mut disk_map: Vec<String> = input
        .chars()
        .filter(|c| c != &'\n')
        .enumerate()
        .flat_map(|(i, c)| {
            let length = c.to_digit(10).unwrap().try_into().unwrap();
            let is_file = i % 2 == 0;
            if is_file {
                let id = (i + 1) / 2;
                return vec![id.to_string(); length];
            }
            vec![".".to_string(); length]
        })
        .collect();

    let mut start_index = 0;
    let mut end_index = disk_map.len() - 1;

    loop {
        while disk_map[start_index] != ".".to_string() {
            start_index += 1;
        }
        while disk_map[end_index] == ".".to_string() {
            end_index -= 1;
        }

        if start_index >= end_index {
            break;
        }

        disk_map.swap(start_index, end_index);
    }

    let result = disk_map
        .iter()
        .enumerate()
        .map(|(i, c)| match c.parse::<u64>() {
            Ok(c) => c * i as u64,
            Err(_) => return 0,
        })
        .sum::<u64>() as u64;

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut disk_map: Vec<Vec<String>> = input
        .chars()
        .filter(|c| c != &'\n')
        .enumerate()
        .map(|(i, c)| {
            let length = c.to_digit(10).unwrap().try_into().unwrap();
            let is_file = i % 2 == 0;
            if is_file {
                let id = (i + 1) / 2;
                return vec![id.to_string(); length];
            }
            vec![".".to_string(); length]
        })
        .collect();

    disk_map.reverse();

    let mut latest_file_id = disk_map.iter().find(|i| i[0] != ".".to_string()).unwrap()[0]
        .parse::<i64>()
        .ok()
        .unwrap();

    while latest_file_id >= 0 {
        println!("latest_file_id: {}", latest_file_id);
        let (i, file) = disk_map
            .iter()
            .enumerate()
            .find(|(_i, f)| f.len() > 0 && f[0] == latest_file_id.to_string())
            .unwrap();

        latest_file_id -= 1;

        for j in 1..disk_map.len() {
            let index = disk_map.len() - j;

            let memory = disk_map[index].clone();

            if memory.len() < 1 {
                continue;
            }

            if memory[0] != ".".to_string() {
                continue;
            }
            if memory.len() < file.len() {
                continue;
            }

            let diff = memory.len() - file.len();

            if index < i {
                println!("breaking because index ({index}) < i ({i})");
                break;
            }

            let mut new_memory = vec![];
            for _ in 0..diff {
                new_memory.push(".".to_string());
                disk_map[index].pop();
            }

            disk_map.swap(i, index);
            if new_memory.len() > 0 {
                disk_map.insert(index, new_memory);
            }
            break;
        }

        let mut current = disk_map.clone();
        current.reverse();
    }

    disk_map.reverse();

    let mut index = 0;
    let mut count = 0;
    disk_map.iter().for_each(|c| {
        c.iter().for_each(|d| {
            match d.parse::<u64>() {
                Ok(c) => {
                    count += c * index as u64;
                }
                Err(_) => {}
            }
            index += 1;
        })
    });

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
