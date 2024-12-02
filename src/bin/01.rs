use itertools::Itertools;
use std::str::FromStr;

advent_of_code::solution!(1);

// Disclaimer - I am making extensive use of improper practices for production code, for example the
// liberal use of `.unwrap()`.

pub fn part_one(input: &str) -> Option<u32> {
    let (mut left, mut right): (Vec<u32>, Vec<u32>) = input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .collect_tuple::<(&str, &str)>()
                .unwrap()
        })
        .map(|(left, right)| (u32::from_str(left).unwrap(), u32::from_str(right).unwrap()))
        .unzip();

    left.sort_unstable();
    right.sort_unstable();

    let result = left
        .iter()
        .zip(right.iter())
        .map(|(left, right)| left.abs_diff(*right))
        .sum::<u32>();

    Some(result)
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
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
