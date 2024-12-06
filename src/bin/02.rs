advent_of_code::solution!(2);

fn is_safe(levels: &Vec<i32>) -> bool {
    let mut increase = false;
    let mut decrease = false;
    let mut first_run = true;
    levels.is_sorted_by(|a, b| {
        let increased = increase;
        let decreased = decrease;
        #[allow(unused_assignments)]
        let mut result = false;

        if a < b && b - a < 4 {
            increase = true;
            decrease = false;
            result = if first_run {
                increase
            } else {
                increase == increased
            };
        } else if b < a && a - b < 4 {
            increase = false;
            decrease = true;
            result = if first_run {
                decrease
            } else {
                decrease == decreased
            };
        } else {
            result = false;
        }

        first_run = false;
        return result;
    })
}

pub fn part_one(input: &str) -> Option<i32> {
    if input.len() == 0 {
        println!("Input is empty!");
        return None;
    }

    let mut count_safe = 0;
    let lines: Vec<&str> = input.split('\n').collect();

    for line in lines {
        let levels = line
            .split(' ')
            .map(|l| l.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let safe_report = is_safe(&levels);
        count_safe += if safe_report { 1 } else { 0 };
    }

    Some(count_safe)
}

pub fn part_two(input: &str) -> Option<i32> {
    if input.len() == 0 {
        println!("Input is empty!");
        return None;
    }

    let mut count_safe = 0;
    let lines: Vec<&str> = input.split('\n').collect();

    for line in lines {
        let levels = line
            .split(' ')
            .map(|l| l.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let mut safe_report = is_safe(&levels);

        if !safe_report {
            // for every level in levels call is_safe with a subset of levels
            for (i, _level) in levels.iter().enumerate() {
                safe_report = is_safe(&[&levels[..i], &levels[i + 1..]].concat());
                if safe_report {
                    break;
                }
            }
        }

        count_safe += if safe_report { 1 } else { 0 };
    }

    Some(count_safe)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
