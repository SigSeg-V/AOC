/// Historian Hysteria
/// Charlie Hutcheson 1/1/24

pub const INPUT_STR: &str = include_str!("../resources/DayOne/input.txt");

// collect input into two vectors
fn collect_input(input: &str) -> [Vec<usize>; 2]{
    let mut left_list = vec![];
    let mut right_list= vec![];

    input.lines().for_each(|line| {
        // split string by whitespace and extract numbers
        let mut tokens = line.split_ascii_whitespace();
        let left_number = tokens.next().expect("left should have a value");
        let right_number = tokens.next().expect("right should have a value");
        
        left_list.push(left_number.parse::<usize>().expect("left value should be a valid u32"));
        right_list.push(right_number.parse::<usize>().expect("right number should be a valid u32"));
    });

    [left_list, right_list]
}

pub fn part_one(input: &str) -> usize {
    let [mut left_list, mut right_list] = collect_input(input);
    left_list.sort();
    right_list.sort();

    left_list
    .iter()
    .zip(right_list)
    .fold(0, |acc, (left, right)| acc + left.abs_diff(right))
}

pub fn part_two(input: &str) -> usize {
    let [mut left_list, mut right_list] = collect_input(input); 
    // get rid of duplicates
    left_list.sort();
    left_list.dedup();

    left_list
    .iter()
    .fold(0, |acc, left| {acc + *left * right_list.iter().filter(|&right| right == left).count()})
}

#[cfg(test)]
mod tests {
    use super::*;
const EXAMPLE_STR: &str = include_str!("../resources/DayOne/example.txt");

    #[test]
    fn example() {
        assert_eq!(part_one(EXAMPLE_STR), 11);
    }
}