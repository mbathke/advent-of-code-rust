use std::ascii::AsciiExt;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<i32> {
    println!("{}", input);

    let chars_vec: Vec<char> = input.chars().collect();

    let get_num = |pos: usize| -> i32 {
        let result = for d in pos..chars_vec.len() {
            let mut result = 0;

            let digit = match chars_vec[pos].is_digit(10) {
                Some(x) => x as i32,
                None => -1
            };

            if digit == -1 { return - 1 }

            result = result * 10 + digit
        };

        result
    };

    let mut answer = 0;

    for (mut i, c) in chars_vec.iter().enumerate() {
        if *c == 'm' {
            if chars_vec[i + 1] == 'u' && chars_vec[i + 2] == 'l' && chars_vec[i + 3] == '(' {
                i += 4;
                let a = get_num(i);

                if chars_vec[i + 1] == ',' {
                    i += 2;
                    let b = get_num(i);

                    println!("a {}, b {}", a, b);

                    if a != -1 && b != -1 && chars_vec[i + 1] == ')' {
                        answer += a * b;
                    }
                }
            }
        }
    }

    Some(answer)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
