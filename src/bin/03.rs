use regex::Regex;
use std::str::FromStr;
use std::sync::LazyLock;

advent_of_code::solution!(3);

static MUL_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"mul\(\d+,\d+\)").unwrap());
static PARAM_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\d+").unwrap());

pub fn part_one(input: &str) -> Option<u32> {
    Some(calculate_mul(input))
}

pub fn part_two(input: &str) -> Option<u32> {
    let split_iter = input.split("don't()");

    let mut safe_string = String::with_capacity(input.len());

    split_iter.enumerate().for_each(|(index, str)| {
        if index == 0 {
            safe_string += str;
        } else {
            let mut str_iter = str.split("do()");
            str_iter.next();

            str_iter.for_each(|str| safe_string += str);
        }
    });

    Some(calculate_mul(safe_string.as_str()))
}

fn calculate_mul(input: &str) -> u32 {
    MUL_REGEX
        .find_iter(input)
        .map(|m| {
            PARAM_REGEX
                .find_iter(m.as_str())
                .map(|m2| u32::from_str(m2.as_str()).unwrap())
                .reduce(|acc, e| acc * e)
                .unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
