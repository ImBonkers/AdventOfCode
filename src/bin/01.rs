use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut left_list = Vec::<i32>::new();
    let mut right_list = Vec::<i32>::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split("   ").collect();

        if parts.len() != 2 {
            return None;
        }

        let left_num: i32 = parts[0].parse().unwrap();
        let right_num: i32 = parts[1].parse().unwrap();

        left_list.push(left_num);
        right_list.push(right_num);
    }

    left_list.sort();
    right_list.sort();

    let mut sum: u32 = 0;
    for i in 0..left_list.len() {
        let left = left_list[i];
        let right = right_list[i];

        let diff = (right - left).abs() as u32;
        sum += diff;
    }

    return Some(sum);
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut left_list = Vec::<u32>::new();
    let mut right_list = Vec::<u32>::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split("   ").collect();

        if parts.len() != 2 {
            return None;
        }

        let left_num: u32 = parts[0].parse().unwrap();
        let right_num: u32 = parts[1].parse().unwrap();

        left_list.push(left_num);
        right_list.push(right_num);
    }

    let mut number_count = HashMap::<u32, u32>::new();

    for num in right_list.iter() {
        let count = number_count.entry(*num).or_insert(0);
        *count += 1;
    }

    let mut sum = 0;
    for num in left_list.iter() {
        if number_count.contains_key(num) {
            let count = number_count.get_mut(num).unwrap();
            sum += num * *count;
        }
    }

    return Some(sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
