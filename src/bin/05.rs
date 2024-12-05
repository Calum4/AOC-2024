use itertools::Itertools;
use std::cmp::Ordering;
use std::str::FromStr;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let (page_ordering_str, page_numbers_str) = input.split_once("\n\n").unwrap();
    let page_ordering = setup_ordering(page_ordering_str);

    let mut middle_page_number_sum: u32 = 0;

    page_numbers_str
        .lines()
        .map(|line| {
            line.split(',')
                .map(|page_number| u8::from_str(page_number).unwrap())
                .collect_vec()
        })
        .for_each(|page_numbers| {
            let mut is_ordered = true;

            if page_numbers.len() > 1 && (page_numbers.len() & 1) != 0 {
                let mut page_numbers_iter = page_numbers.iter();
                let mut previous = page_numbers_iter.next().unwrap();

                for next in page_numbers_iter {
                    if page_ordering[*previous as usize][*next as usize] {
                        previous = next;
                        continue;
                    }

                    is_ordered = false;
                    break;
                }
            } else {
                is_ordered = false;
            }

            if is_ordered {
                middle_page_number_sum += page_numbers[(page_numbers.len() - 1) / 2] as u32;
            }
        });

    Some(middle_page_number_sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (page_ordering_str, page_numbers_str) = input.split_once("\n\n").unwrap();
    let page_ordering = setup_ordering(page_ordering_str);

    let mut middle_page_number_sum: u32 = 0;

    page_numbers_str
        .lines()
        .map(|line| {
            line.split(',')
                .map(|page_number| u8::from_str(page_number).unwrap())
                .collect_vec()
        })
        .for_each(|mut page_numbers| {
            if page_numbers.is_sorted_by(|a, b| page_ordering[*a as usize][*b as usize]) {
                return;
            }

            page_numbers.sort_by(|a: &u8, b: &u8| -> Ordering {
                match page_ordering[*a as usize][*b as usize] {
                    true => Ordering::Greater,
                    false => Ordering::Less,
                }
            });

            middle_page_number_sum += page_numbers[(page_numbers.len() - 1) / 2] as u32;
        });

    Some(middle_page_number_sum)
}

fn setup_ordering(page_ordering_str: &str) -> [[bool; 100]; 100] {
    let mut page_ordering = [[false; 100]; 100];

    page_ordering_str
        .lines()
        .map(|line| line.split('|'))
        .filter_map(|mut a| {
            let left = a.next()?;
            let right = a.next()?;

            #[inline]
            fn convert(str: &str) -> usize {
                usize::from_str(str).unwrap()
            }

            Some((convert(left), convert(right)))
        })
        .for_each(|(left, right)| {
            page_ordering[left][right] = true;
        });
    
    page_ordering
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
