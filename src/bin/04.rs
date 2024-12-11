use std::usize;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<i32> {
    let mut matrix: Vec<Vec<char>> = Vec::new();
    let search_term: Vec<char> = vec!['X', 'M', 'A', 'S'];

    for line in input.lines() {
        let line_chars = line.chars().collect();
        matrix.push(line_chars);
    }

    let mut found_xmas = 0;

    for y in 0..matrix.len() {
        for x in 0..matrix[y].len() {
            if matrix[y][x] == 'X' {
                for dy in -1..=1 as i32 {
                    for dx in -1..=1 as i32 {
                        if dy == 0 && dx == 0 {
                            continue;
                        }
                        let mut keyword_matches = true;
                        for (i, &val) in search_term.iter().enumerate() {
                            let row = y as i32 + dy * i as i32;
                            let col = x as i32 + dx * i as i32;

                            if 0 > row
                                || (row as usize) >= matrix.len()
                                || 0 > col
                                || (col as usize) >= matrix[y].len()
                                || matrix[row as usize][col as usize] != val
                            {
                                keyword_matches = false;
                                break;
                            }
                        }

                        if keyword_matches {
                            found_xmas += 1
                        }
                    }
                }
            }
        }
    }

    Some(found_xmas)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut matrix: Vec<Vec<char>> = Vec::new();
    let directions: Vec<(i32,i32)> = vec![(-1, -1), (-1, 1), (1, 1), (1, -1)];

    for line in input.lines() {
        let line_chars = line.chars().collect();
        matrix.push(line_chars);
    }

    let mut found_mas = 0;
    for y in 1..matrix.len() - 1 {
        for x in 1..matrix[y].len() - 1 {
            if matrix[y][x] == 'A' {
                let mut s = String::new();

                for dir in directions.iter() {
                    let dy = y as i32 + dir.0;
                    let dx = x as i32 + dir.1;
                    s.push(matrix[dy as usize][dx as usize]);
                }

                if s == "MMSS" || s == "MSSM" || s == "SSMM" || s == "SMMS" {
                    found_mas += 1;
                }
            }
        }
    }

    Some(found_mas)
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
        assert_eq!(result, Some(9));
    }
}
