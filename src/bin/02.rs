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
        .filter(|report| check_safe(report.iter()))
        .count();

    Some(u32::try_from(result).unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    let result = input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|level| u8::from_str(level).unwrap())
                .collect_vec()
        })
        .filter(|report| {
            if check_safe(report.iter()) {
                return true;
            }

            for i in 0..report.len() {
                let report_iter = report
                    .iter()
                    .enumerate()
                    .filter(|(index, _)| *index != i)
                    .map(|(_, report)| report);

                if check_safe(report_iter) {
                    return true;
                }
            }

            false
        })
        .count();

    Some(u32::try_from(result).unwrap())
}

fn check_safe<'a, I>(mut level_iter: I) -> bool
where
    I: Iterator<Item = &'a u8>,
{
    let mut is_increasing: Option<bool> = None;
    let mut previous_value: Option<u8> = None;

    loop {
        let prev = match previous_value {
            None => level_iter
                .next()
                .copied()
                .expect("a report should always have at least one level"),
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
