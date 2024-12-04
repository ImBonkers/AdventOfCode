advent_of_code::solution!(2);

pub fn valid_check_levels(levels: &Vec<i32>) -> bool {
    let diffs = levels.windows(2).map(|x| x[1] - x[0]).collect::<Vec<i32>>();
    let small = diffs.iter().all(|&x| x.abs() <= 3);
    let non_zero = diffs.iter().all(|&x| x != 0);
    let all_positive = diffs.iter().all(|&x| x > 0);
    let all_negative = diffs.iter().all(|&x| x < 0);

    return small && non_zero && (all_positive ^ all_negative);
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut valid_reports: u32 = 0;
    for line in input.lines() {
        let levels = line
            .split_whitespace()
            .filter_map(|x| x.parse::<i32>().ok())
            .collect::<Vec<i32>>();

        let diffs = levels.windows(2).map(|x| x[1] - x[0]).collect::<Vec<i32>>();
        let small = diffs.iter().all(|&x| x.abs() <= 3);
        let non_zero = diffs.iter().all(|&x| x != 0);
        let all_positive = diffs.iter().all(|&x| x > 0);
        let all_negative = diffs.iter().all(|&x| x < 0);

        print!("Is small: {}, Is non-zero: {}\n", small, non_zero);

        if small && non_zero && (all_positive ^ all_negative) {
            valid_reports += 1;
        }
    }

    return Some(valid_reports);
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut valid_reports: u32 = 0;
    let mut valid = false;
    for line in input.lines() {
        let levels = line
            .split_whitespace()
            .filter_map(|x| x.parse::<i32>().ok())
            .collect::<Vec<i32>>();

        if valid_check_levels(&levels) {
            valid = true;
            valid_reports += 1;
            continue;
        }

        for i in 0..levels.len() {
            let mut altered = levels.clone();
            altered.remove(i);

            if valid_check_levels(&altered) {
                valid = true;
                valid_reports += 1;
                break;
            }
        }
    }

    return Some(valid_reports);
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
