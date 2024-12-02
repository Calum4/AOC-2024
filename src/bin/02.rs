use itertools::Itertools;
use std::str::FromStr;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let result = input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|level| u8::from_str(level).unwrap())
                .collect_vec()
        })
        .filter(|report| {
            let mut is_increasing: Option<bool> = None;
            let mut previous_value: Option<u8> = None;
            let mut level_iter = report.iter();

            loop {
                let prev = match previous_value {
                    None => {
                        let Some(next) = level_iter.next().copied() else {
                            break;
                        };

                        next
                    }
                    Some(prev) => prev,
                };

                let Some(next) = level_iter.next().copied() else {
                    return true;
                };

                if !(1..=3).contains(&prev.abs_diff(next)) {
                    break;
                }

                let increasing = match is_increasing {
                    None => {
                        let increasing = next > prev;
                        is_increasing = Some(increasing);
                        increasing
                    }
                    Some(increasing) => increasing,
                };

                if increasing != (next > prev) {
                    break;
                }

                previous_value = Some(next);
            }

            false
        })
        .count();

    Some(u32::try_from(result).unwrap())
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
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}