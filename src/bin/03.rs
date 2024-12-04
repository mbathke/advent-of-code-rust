use nom::{bytes::{complete::take_while_m_n, streaming::is_a}, combinator::map_res, sequence::tuple, IResult};

advent_of_code::solution!(3);

#[derive(Debug, PartialEq)]
pub struct PuzzleResult(u32);

fn from_dec(input: &str) -> Result<u32, std::num::ParseIntError> {
  u32::from_str_radix(input, 10)
}

fn is_digit(c: char) -> bool {
  c.is_digit(16)
}

fn primary(input: &str) -> IResult<&str, u32> {
  map_res(
    take_while_m_n(2, 2, is_digit),
    from_dec
  )(input)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (input, _) = is_a::<&str, &str, nom::error::Error<&str>>("mul")(input).unwrap();
    let (_input, (mul_1, mul_2)) = tuple((primary, primary))(input).unwrap();
    let multiplicator = mul_1 * mul_2;
    Some(multiplicator)
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
