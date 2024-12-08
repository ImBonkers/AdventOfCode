use std::collections::HashMap;

advent_of_code::solution!(5);

pub fn update_numbers_are_out_of_order(
    start_number: u32,
    goal_number: u32,
    number_ordering: &HashMap<u32, Vec<u32>>,
) -> bool {
    let valid_numbers = number_ordering.get(&start_number).unwrap();
    if valid_numbers.contains(&goal_number) {
        return false;
    }
    return true;
}

pub fn update_order_is_valid(
    update_order: &Vec<u32>,
    number_ordering: &HashMap<u32, Vec<u32>>,
) -> bool {
    for update_number_idx in 0..update_order.len() {
        for next_number_idx in (update_number_idx + 1)..update_order.len() {
            let update_number = update_order[update_number_idx];
            let next_number = update_order[next_number_idx];
            if update_numbers_are_out_of_order(
                update_number as u32,
                next_number as u32,
                number_ordering,
            ) {
                return false;
            }
        }
    }
    return true;
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut number_ordering: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut update_ordering: Vec<Vec<u32>> = Vec::new();

    let sections = input.split("\n\n").collect::<Vec<&str>>();
    let rules_lines = sections[0].split("\n").collect::<Vec<&str>>();
    let order_lines = sections[1].split("\n").collect::<Vec<&str>>();

    for line in rules_lines {
        let parts = line.split("|").collect::<Vec<&str>>();
        let first_rule = parts[0].parse::<u32>().unwrap();
        let second_rule = parts[1].parse::<u32>().unwrap();

        if !number_ordering.contains_key(&first_rule) {
            number_ordering.insert(first_rule, Vec::new());
        }

        number_ordering
            .get_mut(&first_rule)
            .unwrap()
            .push(second_rule);
    }

    for line in order_lines {
        if line.len() == 0 {
            continue;
        }
        let parts = line.split(",").collect::<Vec<&str>>();
        let mut order: Vec<u32> = Vec::new();
        for part in parts {
            order.push(part.parse::<u32>().unwrap());
        }
        update_ordering.push(order);
    }

    let mut valid_update_orderings = 0;
    for update_order in update_ordering {
        if update_order_is_valid(&update_order, &number_ordering) {
            let middle_number = update_order[update_order.len() / 2];
            valid_update_orderings += middle_number;
        }
    }

    return Some(valid_update_orderings);
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut number_ordering: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut update_ordering: Vec<Vec<u32>> = Vec::new();

    let sections = input.split("\n\n").collect::<Vec<&str>>();
    let rules_lines = sections[0].split("\n").collect::<Vec<&str>>();
    let order_lines = sections[1].split("\n").collect::<Vec<&str>>();

    for line in rules_lines {
        let parts = line.split("|").collect::<Vec<&str>>();
        let first_rule = parts[0].parse::<u32>().unwrap();
        let second_rule = parts[1].parse::<u32>().unwrap();

        if !number_ordering.contains_key(&first_rule) {
            number_ordering.insert(first_rule, Vec::new());
        }

        number_ordering
            .get_mut(&first_rule)
            .unwrap()
            .push(second_rule);
    }

    for line in order_lines {
        if line.len() == 0 {
            continue;
        }
        let parts = line.split(",").collect::<Vec<&str>>();
        let mut order: Vec<u32> = Vec::new();
        for part in parts {
            order.push(part.parse::<u32>().unwrap());
        }
        update_ordering.push(order);
    }

    let mut valid_update_orderings = 0;
    for update_order in update_ordering {
        if update_order_is_valid(&update_order, &number_ordering) {
        } else {
            let mut sorted_order = update_order.clone();
            sorted_order.sort_by(|a, b| {
                if update_numbers_are_out_of_order(*a, *b, &number_ordering) {
                    return std::cmp::Ordering::Less;
                } else {
                    return std::cmp::Ordering::Greater;
                }
            });

            let middle_number = sorted_order[sorted_order.len() / 2];
            valid_update_orderings += middle_number;
        }
    }

    return Some(valid_update_orderings);
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
