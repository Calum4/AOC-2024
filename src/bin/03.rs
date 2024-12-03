use std::str::FromStr;

advent_of_code::solution!(3);

/*
    # Performance Optimisation 
    
    ## Original
    Part 1: 184122457 (129.5µs @ 1394 samples)
    Part 2: 107862689 (83.3µs @ 9140 samples)
    
    ## Replaced regex with `core::str::split()` in `self::calculate_mul()`
    Part 1: 184122457 (56.6µs @ 10000 samples)
    Part 2: 107862689 (43.5µs @ 10000 samples)
*/

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
    input
        .split("mul(")
        .map(|start| {
            let Some(param_str) = start.split(")").next() else {
                return 0;
            };

            param_str
                .split(",")
                .map(|param| u32::from_str(param).unwrap_or(0))
                .product::<u32>()
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
