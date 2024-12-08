advent_of_code::solution!(3);

pub fn get_num(pos: &mut usize, chars_vec: &Vec<char>) -> Option<i32> {
    let mut x = 0;
    let mut found = false;

    while chars_vec[*pos].is_digit(10) {
        x = x * 10 + chars_vec[*pos].to_digit(10).unwrap();
        *pos += 1;
        found = true;
    }

    if found {
        Some(x.try_into().unwrap())
    } else {
        None
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let chars_vec: Vec<char> = input.chars().collect();
    let mut answer = 0;

    for (mut i, c) in chars_vec.iter().enumerate() {
        if *c == 'm' {
            if chars_vec[i + 1] == 'u' && chars_vec[i + 2] == 'l' && chars_vec[i + 3] == '(' {
                i += 4;
                let a = get_num(&mut i, &chars_vec);

                if chars_vec[i] == ',' {
                    i += 1;
                    let b = get_num(&mut i, &chars_vec);

                    if a != None && b != None && chars_vec[i] == ')' {
                        answer += a.unwrap() * b.unwrap();
                    }
                }
            }
        }
    }

    Some(answer)
}

pub fn part_two(input: &str) -> Option<i32> {
    let chars_vec: Vec<char> = input.chars().collect();
    let mut answer = 0;
    let mut multiply = true;
    let chars_length = chars_vec.len();

    for (mut i, c) in chars_vec.iter().enumerate() {
        if i + 4 <= chars_length && input[i..i + 4].to_string() == "do()" {
            multiply = true;
        }

        if i + 7 <= chars_length && input[i..i + 7].to_string() == "don\'t()" {
            multiply = false;
        }

        if *c == 'm' && multiply {
            if chars_vec[i + 1] == 'u' && chars_vec[i + 2] == 'l' && chars_vec[i + 3] == '(' {
                i += 4;
                let a = get_num(&mut i, &chars_vec);

                if chars_vec[i] == ',' {
                    i += 1;
                    let b = get_num(&mut i, &chars_vec);

                    if a != None && b != None && chars_vec[i] == ')' {
                        answer += a.unwrap() * b.unwrap();
                    }
                }
            }
        }
    }

    Some(answer)
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
        assert_eq!(result, Some(161));
    }
}
