use std::collections::btree_set::Difference;

advent_of_code::solution!(2);

#[derive(PartialEq)]
enum Direction {
    Increase,
    Decrease,
    None,
}

struct SafeReports(i32);

struct Report {
    direction: Direction,
    last_direction: Direction,
    direction_changes: i32,
    levels: Vec<i32>,
}

impl Report {
    fn is_safe(&mut self) -> bool {
        // constraints:
        // 1. sorted increasingly
        // 2. sorted decreasingly
        // 3. the difference between two levels is at least 1 and a maximum of 3
        // 4. it's sorted in one direction if an element is removed
        let levels = self.levels.clone();

        println!("\nCurrent Report: {:?}", levels);

        for (i, &current_level) in levels.iter().enumerate() {
            if i == 0 { continue };
            let prev_level = levels[i - 1].clone();

            println!("prev: {}, current: {}", prev_level, current_level);

            if current_level > prev_level {
                self.direction = Direction::Increase;

                if self.last_direction == Direction::Decrease {
                    self.direction_changes += 1;
                }

                self.last_direction = Direction::Increase;
            } else if current_level < prev_level {
                self.direction = Direction::Decrease;

                if self.last_direction == Direction::Increase {
                    self.direction_changes += 1;
                }

                self.last_direction = Direction::Decrease;
            }

            let difference = i32::abs(current_level - prev_level);
            println!("Difference {}", difference);

            if difference < 1 || difference > 3 {
                println!("Difference is too small or too big.");
                return false;
            }

            println!("Direction changes: {}", self.direction_changes);

            if self.direction_changes != 0 && self.direction_changes != 2 { return false }
        }

        true
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    if input.len() == 0 {
        println!("Input is empty!");
        return None;
    }

    let mut count_safe = 0;
    let lines: Vec<&str> = input.split('\n').collect();

    for line in lines {
        let levels = line.split(' ').map(|l| l.parse::<i32>().unwrap());

        let mut increase = false;
        let mut decrease = false;
        let mut first_run = true;
        let safe = levels.is_sorted_by(|a, b| {
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
        });

        count_safe += if safe { 1 } else { 0 };
    }

    Some(count_safe)
}

pub fn part_two(input: &str) -> Option<i32> {
    if input.len() == 0 {
        println!("Input is empty!");
        return None;
    }

    let lines: Vec<&str> = input.split('\n').collect();
    let mut safe_reports = SafeReports(0);

    for line in lines {
        let mut report = Report {
            direction: Direction::None,
            last_direction: Direction::None,
            direction_changes: 0,
            levels: line.split(' ').map(|l| l.parse::<i32>().unwrap()).collect(),
        };

        let is_safe_report = report.is_safe();

        if is_safe_report {
            safe_reports.0 += 1;
        }

        println!("Report {:?} is safe? {}", report.levels, is_safe_report);
    }

    Some(safe_reports.0)
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
