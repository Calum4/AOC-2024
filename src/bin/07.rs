use std::str::FromStr;
use itertools::Itertools;
use rayon::prelude::*;

advent_of_code::solution!(7);

fn is_equation_valid_pt1((result, values): (u64, Vec<u64>)) -> Option<u64> {
    for mut i in 0..=((u64::MAX << (values.len() - 1)) ^ u64::MAX) {
        let calculated_result = values.iter().copied().reduce(|acc, value| {
            let acc = match i & 1 {
                0 => acc + value,
                1 => acc * value,
                _ => panic!("This should not be possible")
            };

            i >>= 1;
            acc
        }).unwrap();

        if result == calculated_result {
            return Some(result)
        }
    }

    None
}

pub fn part_one(input: &str) -> Option<u64> {
    let sum = input
        .par_lines()
        .map(|line| {
            let mut split_line = line
                .split(":");

            let result = split_line.next().map(u64::from_str).unwrap().unwrap();
            let values = split_line.next().unwrap().split_ascii_whitespace().flat_map(u64::from_str).collect_vec();

            (result, values)
        })
        .filter_map(is_equation_valid_pt1)
        .sum();

    Some(sum)
}

fn is_equation_valid_pt2((result, values): (u64, Vec<u64>)) -> Option<u64> {
    let num_ops = values.len() - 1;

    let mut result_vec = vec![0; num_ops];

    for i in 0..3u32.pow(num_ops as u32) {
        if i > 0 {
            result_vec.fill(0);
        }

        let mut value = i;

        for result in result_vec.iter_mut().rev() {
            if value == 0 {
                break;
            }

            *result = (value % 3) as u8;
            value /= 3;
        }

        let mut result_vec_iter = result_vec.iter();

        let calculated_result = values.iter().copied().reduce(|acc, value| {
            match result_vec_iter.next().unwrap() {
                0 => acc + value,
                1 => acc * value,
                2 => {
                    let concat = format!("{acc}{value}");
                    concat.parse().unwrap()
                }
                _ => panic!("This should not be possible")
            }
        }).unwrap();

        if result == calculated_result {
            return Some(result)
        }
    }

    None
}

pub fn part_two(input: &str) -> Option<u64> {
    let sum = input
        .par_lines()
        .map(|line| {
            let mut split_line = line
                .split(":");

            let result = split_line.next().map(u64::from_str).unwrap().unwrap();
            let values = split_line.next().unwrap().split_ascii_whitespace().flat_map(u64::from_str).collect_vec();

            (result, values)
        })
        .filter_map(is_equation_valid_pt2)
        .sum();

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
