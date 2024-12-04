use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let mul_re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let num_re = Regex::new(r"\d{1,3}").unwrap();

    let mut sum = 0;
    for mat in mul_re.find_iter(input) {
        let mul_str = mat.as_str();
        let nums = num_re
            .find_iter(mul_str)
            .map(|x| x.as_str().parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        sum += nums[0] * nums[1];
    }

    return Some(sum);
}

pub fn part_two(input: &str) -> Option<u32> {
    let mul_re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let num_re = Regex::new(r"\d{1,3}").unwrap();

    let do_split = input.split("do()");
    let mut sections = Vec::<&str>::new();

    for split in do_split {
        if split.contains("don't()") {
            let do_part = split.split("don't()").collect::<Vec<&str>>()[0];
            sections.push(do_part);
        } else {
            sections.push(split);
        }
    }

    let mut sum = 0;
    for substring in sections {
        for mat in mul_re.find_iter(&substring) {
            let mul_str = mat.as_str();
            let nums = num_re
                .find_iter(mul_str)
                .map(|x| x.as_str().parse::<u32>().unwrap())
                .collect::<Vec<u32>>();

            sum += nums[0] * nums[1];
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
