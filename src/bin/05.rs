advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let rules_and_pages: Vec<&str> = input.split("\n\n").collect();
    let rules: Vec<&str> = rules_and_pages[0].lines().collect();
    let pages: Vec<&str> = rules_and_pages[1].lines().collect();

    // [(a, b), (a, b), ..]
    let rules: Vec<(i32, i32)> = rules
        .iter()
        .map(|r| {
            let rule: Vec<i32> = r.split('|').map(|s| s.parse::<i32>().unwrap()).collect();
            (rule[0], rule[1])
        })
        .collect();

    // [[a, b, c, d], [a, b, c, d], ..]
    let pages: Vec<Vec<i32>> = pages
        .iter()
        .map(|p| p.split(',').map(|n| n.parse::<i32>().unwrap()).collect())
        .collect();

    println!("Rules {:?}", rules);
    println!("Pages {:?}", pages);

    None
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
