advent_of_code::solution!(1);

fn find_factor(num: i32, numbers: &Vec<i32>) -> i32 {
    numbers.iter().filter(|&n| *n == num).collect::<Vec<_>>().len() as i32
}

pub fn part_one(input: &str) -> Option<i32> {
    if input.len() == 0 {
        println!("Input is empty");
        return None
    }

    // split the input into lines
    let lines: Vec<&str> = input.split('\n').collect();
    let mut left_numbers: Vec<i32> = Vec::new();
    let mut right_numbers: Vec<i32> = Vec::new();

    // read the numbers from left and right and store them into different vectors
    for line in lines {
        let split: Vec<&str> = line.split("   ").collect();
        let left_element = split.first().unwrap().parse::<i32>().unwrap();
        let right_element = split.last().unwrap().parse::<i32>().unwrap();

        left_numbers.push(left_element);
        right_numbers.push(right_element);
    }

    // sort the numbers
    left_numbers.sort();
    right_numbers.sort();
    let mut total_distance = 0;

    for (pos, _number) in left_numbers.iter().enumerate() {
        let distance = i32::abs(left_numbers[pos] - right_numbers[pos]);
        total_distance += distance;
    }

    Some(total_distance)
}

pub fn part_two(input: &str) -> Option<i32> {
    if input.len() == 0 {
        println!("Input is empty");
        return None
    }

    // split the input into lines
    let lines: Vec<&str> = input.split('\n').collect();
    let mut left_numbers: Vec<i32> = Vec::new();
    let mut right_numbers: Vec<i32> = Vec::new();

    // read the numbers from left and right and store them into different vectors
    for line in lines {
        let split: Vec<&str> = line.split("   ").collect();
        let left_element = split.first().unwrap().parse::<i32>().unwrap();
        let right_element = split.last().unwrap().parse::<i32>().unwrap();

        left_numbers.push(left_element);
        right_numbers.push(right_element);
    }

    // sort the numbers
    left_numbers.sort();
    right_numbers.sort();
    let mut total_sim_score = 0;

    for (pos, number) in left_numbers.iter().enumerate() {
        let factor = find_factor(*number, &right_numbers);
        let sim_score = left_numbers[pos] * factor;
        total_sim_score += sim_score;
    }

    Some(total_sim_score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
