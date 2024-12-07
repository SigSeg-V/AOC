// Red-nosed Reports
// Charlie Hutcheson - 5/12/24
use std::ops::{Deref, RangeBounds, RangeInclusive};

pub const INPUT_STR: &str = include_str!("../resources/DayTwo/input.txt");
pub const EXAMPLE_STR: &str = include_str!("../resources/DayTwo/example.txt");

pub fn part_one(input: &str) -> usize {
    // split into lines
    input.lines().filter(|line|{
        // get first two elements for direction and distance
        let line_nums = line
        .split_ascii_whitespace()
        .map(|n| {
            n.parse::<isize>().unwrap()
        })
        .collect::<Vec<_>>();

        // get direction
        let diff = line_nums.get(1).unwrap() - line_nums.first().unwrap();
        let is_ascending = match diff {
            x if x > 0 => true,
            x if x < 0 => false,
            _ => return false, // no difference is a failed test
        };
        let windows = line_nums.windows(2);
        for wnd in windows {
            let first = wnd.first().unwrap();
            let second = wnd.get(1).unwrap();
            let diff = second - first;
            if !(1..=3).contains(&diff.abs()) || (diff > 0) != is_ascending {
                return false;
            }
        }
        true
    })
    .count()
}

// trying a smarter solution than above - might need to calc up to 4 times though ;-;
fn is_safe_record(record: &[isize], range: &impl RangeBounds<isize>, retry: bool) -> bool {
    let mut prev = record.first().unwrap();
    let mut retry = retry;
    for curr in &record[1..] {
        match range.contains(&(curr - prev)) {
            true => prev = curr,
            false if !retry => return false,
            _ => retry = false,
        }
    }
    true
}

pub fn part_two(input: &str) -> usize {
    input.lines().filter(|line|{
        // convert line to integers
        let vec = line
            .split_ascii_whitespace()
            .map(|n| n.parse::<isize>().unwrap())
            .collect::<Vec<_>>();

        static INCREASING: RangeInclusive<isize> = 1..=3;
        static DECREASING: RangeInclusive<isize> = -3..=-1;
        [
            is_safe_record(&vec, &INCREASING, true),
            is_safe_record(&vec, &DECREASING, true),
            is_safe_record(&vec[1..], &INCREASING, false),
            is_safe_record(&vec[1..], &DECREASING, false),
        ].iter().any(|&x| x)
    }).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        assert_eq!(part_one(EXAMPLE_STR), 2)
    }

    #[test]
    fn part_two_example() {

        assert_eq!(part_two(EXAMPLE_STR), 4)
    }
}