advent_of_code::solution!(4);

const SEARCH_WORD_LEN: usize = 4;

pub fn check_for_xmas(x: usize, y: usize, matrix: &Vec<Vec<char>>) -> bool {
    match matrix[y][x] == 'X'
        && matrix[y][x + 1] == 'M'
        && matrix[y][x + 2] == 'A'
        && matrix[y][x + 3] == 'S'
    {
        true => true,
        false => false,
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let mut matrix: Vec<Vec<char>> = Vec::new();

    println!("[");
    for line in input.lines() {
        let line_chars = line.chars().collect();
        println!("{:?}", line_chars);
        matrix.push(line_chars);
    }
    println!("]");

    let mut found_xmas = 0;

    for mut y in 0..matrix.len() {
        for mut x in 0..matrix[y].len() {
            if matrix[y][x] == 'X' {
                let ly = matrix[y].len();
                match ly {
                    ly x + SEARCH_WORD_LEN => found_xmas += if check_for_xmas(x, y, &matrix) { 1 } else { 0 },
                }


                if x + SEARCH_WORD_LEN < matrix[y].len()
                    && check_for_xmas(x, y, &matrix)
                {
                    found_xmas += 1;
                    x += SEARCH_WORD_LEN;
                }
            }
        }
    }

    Some(found_xmas)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
