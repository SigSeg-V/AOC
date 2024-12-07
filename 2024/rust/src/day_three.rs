// mull ti over
// charlie hutcheson
use regex::Regex;
pub const EXAMPLE_STR: &str = include_str!("../resources/DayThree/example.txt");
pub const EXAMPLE2_STR: &str = include_str!("../resources/DayThree/example2.txt");
pub const INPUT_STR: &str = include_str!("../resources/DayThree/input.txt");

fn multiply_num_in_string(s: &str) -> isize {
    s
        .chars()
        .filter(|&c| c.is_numeric() || c == ',')
        .collect::<String>()
        .split(",")
        .map(|num| num.parse::<isize>().unwrap())
        .fold(1, |acc, num| acc * num)
}

pub(crate) fn part_one(input: &str) -> isize {
    let re = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)").unwrap();
    re.find_iter(input).map(|op|{
        multiply_num_in_string(op.as_str())

    })
        .sum()
}

pub(crate) fn part_two(input: &str) -> isize {
    let re = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)|do\(\)|don't\(\)").unwrap();
    let mut enable = true;
    re.find_iter(input).map(|op|{
        match op.as_str() {
            "do()" => {enable = true; 0},
            "don't()" => {enable = false; 0},
            x => {
                if enable {
                    multiply_num_in_string(x)
                } else {
                    0
                }
            },
        }
    })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(EXAMPLE_STR), 161)
    }
    #[test]
    fn test_part_two() {
        assert_eq!(part_two(EXAMPLE2_STR), 48)
    }
}