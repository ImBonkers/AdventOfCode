advent_of_code::solution!(4);

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Letter {
    X,
    M,
    A,
    S,
    NONE,
}

pub fn get_letter_from_coordinates(matrix: &[Vec<Letter>], x: i32, y: i32) -> Letter {
    if let Some(row) = matrix.get(x as usize) {
        if let Some(&letter) = row.get(y as usize) {
            return letter.clone();
        }
    }
    return Letter::NONE;
}

pub fn convert_char_to_enum_letter(c: char) -> Letter {
    match c {
        'X' => return Letter::X,
        'M' => return Letter::M,
        'A' => return Letter::A,
        'S' => return Letter::S,
        _ => return Letter::NONE,
    }
}

pub fn look_for_word_in_direction(
    matrix: &[Vec<Letter>],
    word: &[Letter],
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
) -> bool {
    for step in 0..word.len() {
        let new_x = x + (dx * step as i32);
        let new_y = y + (dy * step as i32);
        let letter = get_letter_from_coordinates(matrix, new_x, new_y);
        if letter != word[step] {
            return false;
        }
    }
    return true;
}

pub fn look_for_words_from_coordinate(matrix: &[Vec<Letter>], x: u32, y: u32) -> u32 {
    let word = [Letter::X, Letter::M, Letter::A, Letter::S];
    let directions: Vec<Vec<i32>> = vec![
        vec![0, 1],
        vec![1, 0],
        vec![1, 1],
        vec![1, -1],
        vec![0, -1],
        vec![-1, 0],
        vec![-1, -1],
        vec![-1, 1],
    ];

    let mut valid_words = 0;
    for direction in directions.iter() {
        let dx = direction[0];
        let dy = direction[1];
        if look_for_word_in_direction(matrix, &word, x as i32, y as i32, dx, dy) {
            valid_words += 1;
        }
    }

    return valid_words;
}

pub fn look_for_crossed_words_from_coordinate(matrix: &[Vec<Letter>], x: i32, y: i32) -> u32 {
    let word = [Letter::M, Letter::A, Letter::S];
    let word_vectors = [
        vec![vec![-1, -1], vec![1, 1]],
        vec![vec![1, -1], vec![-1, 1]],
        vec![vec![-1, 1], vec![1, -1]],
        vec![vec![1, 1], vec![-1, -1]],
    ];

    let upper_left = look_for_word_in_direction(
        matrix,
        &word,
        x + word_vectors[0][0][0],
        y + word_vectors[0][0][1],
        word_vectors[0][1][0],
        word_vectors[0][1][1],
    );

    let upper_right = look_for_word_in_direction(
        matrix,
        &word,
        x + word_vectors[1][0][0],
        y + word_vectors[1][0][1],
        word_vectors[1][1][0],
        word_vectors[1][1][1],
    );

    let lower_left = look_for_word_in_direction(
        matrix,
        &word,
        x + word_vectors[2][0][0],
        y + word_vectors[2][0][1],
        word_vectors[2][1][0],
        word_vectors[2][1][1],
    );

    let lower_right = look_for_word_in_direction(
        matrix,
        &word,
        x + word_vectors[3][0][0],
        y + word_vectors[3][0][1],
        word_vectors[3][1][0],
        word_vectors[3][1][1],
    );

    if (upper_left || lower_right) && (upper_right || lower_left) {
        return 1;
    }

    return 0;
}

pub fn part_one(input: &str) -> Option<u32> {
    let letter_matrix: Vec<Vec<Letter>> = input
        .split("\n")
        .map(|x| x.chars().map(|y| convert_char_to_enum_letter(y)).collect())
        .collect();

    let mut found_words: u32 = 0;
    for x in 0..letter_matrix.len() {
        for y in 0..letter_matrix[x].len() {
            found_words += look_for_words_from_coordinate(&letter_matrix, x as u32, y as u32);
        }
    }

    return Some(found_words);
}

pub fn part_two(input: &str) -> Option<u32> {
    let letter_matrix: Vec<Vec<Letter>> = input
        .split("\n")
        .map(|x| x.chars().map(|y| convert_char_to_enum_letter(y)).collect())
        .collect();

    let mut found_words: u32 = 0;
    for x in 0..letter_matrix.len() {
        for y in 0..letter_matrix[x].len() {
            found_words +=
                look_for_crossed_words_from_coordinate(&letter_matrix, x as i32, y as i32);
        }
    }

    return Some(found_words);
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
