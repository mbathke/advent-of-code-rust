use std::usize;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<i32> {
    let rules_and_pages: Vec<&str> = input.split("\n\n").collect();
    let rules: Vec<&str> = rules_and_pages[0].lines().collect();
    let manual: Vec<&str> = rules_and_pages[1].lines().collect();

    // [(a, b), (a, b), ..]
    let rules: Vec<(i32, i32)> = rules
        .iter()
        .map(|r| {
            let rule: Vec<i32> = r.split('|').map(|s| s.parse::<i32>().unwrap()).collect();
            (rule[0], rule[1])
        })
        .collect();

    // [[a, b, c, d], [a, b, c, d], ..]
    let manual: Vec<Vec<i32>> = manual
        .iter()
        .map(|p| p.split(',').map(|n| n.parse::<i32>().unwrap()).collect())
        .collect();

    let mut answer = 0;
    for pages in manual.iter() {
        let sorted_pages = pages.clone();
        let mut is_sorted = true;

        for rule in rules.iter() {
            let left = rule.0;
            let right = rule.1;

            let left_pos = match sorted_pages.iter().position(|&p| p == left) {
                Some(pos) => pos,
                None => continue,
            };

            let right_pos = match sorted_pages.iter().position(|&p| p == right) {
                Some(pos) => pos,
                None => continue,
            };

            if left_pos > right_pos {
                is_sorted = false;
                break;
            }
        }

        if is_sorted {
            let middle_index = ((sorted_pages.len() / 2) as f32).floor() as i32;
            answer += sorted_pages.get(middle_index as usize).unwrap();
        }
    }

    Some(answer)
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
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
