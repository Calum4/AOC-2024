use num_traits::FromPrimitive;
use std::iter::Iterator;
use strum::{EnumIter, IntoEnumIterator};

advent_of_code::solution!(4);

const MAX_X_LENGTH: usize = if !cfg!(test) { 140 } else { 10 };
const MAX_Y_LENGTH: usize = if !cfg!(test) { 140 } else { 10 };

const WORD_STR: &str = "XMAS";

#[derive(Copy, Clone, EnumIter, Debug)]
enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

#[derive(Copy, Clone, Debug)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn unchecked_increment(&self, direction: Direction) -> Self {
        let mut position = *self;

        match direction {
            Direction::North => position.y -= 1,
            Direction::NorthEast => {
                position.x += 1;
                position.y -= 1;
            }
            Direction::East => position.x += 1,
            Direction::SouthEast => {
                position.x += 1;
                position.y += 1;
            }
            Direction::South => position.y += 1,
            Direction::SouthWest => {
                position.x -= 1;
                position.y += 1;
            }
            Direction::West => position.x -= 1,
            Direction::NorthWest => {
                position.x -= 1;
                position.y -= 1;
            }
        }

        position
    }
}

#[derive(Copy, Clone, Debug)]
struct PotentialWord {
    origin: Position,
    direction: Direction,
}

impl PotentialWord {
    fn try_new(position: Position, direction: Direction) -> Option<PotentialWord> {
        let offset = WORD_STR.len() - 1;
        let mut position_clone = position;

        match direction {
            Direction::North => position_clone.y = position_clone.y.checked_sub(offset)?,
            Direction::NorthEast => {
                position_clone.x = position_clone.x.checked_add(offset)?;
                position_clone.y = position_clone.y.checked_sub(offset)?;
            }
            Direction::East => position_clone.x = position_clone.x.checked_add(offset)?,
            Direction::SouthEast => {
                position_clone.x = position_clone.x.checked_add(offset)?;
                position_clone.y = position_clone.y.checked_add(offset)?;
            }
            Direction::South => position_clone.y = position_clone.y.checked_add(offset)?,
            Direction::SouthWest => {
                position_clone.x = position_clone.x.checked_sub(offset)?;
                position_clone.y = position_clone.y.checked_add(offset)?;
            }
            Direction::West => position_clone.x = position_clone.x.checked_sub(offset)?,
            Direction::NorthWest => {
                position_clone.x = position_clone.x.checked_sub(offset)?;
                position_clone.y = position_clone.y.checked_sub(offset)?;
            }
        }

        if position_clone.x >= MAX_X_LENGTH || position_clone.y >= MAX_Y_LENGTH {
            None
        } else {
            Some(Self {
                origin: position,
                direction,
            })
        }
    }

    fn valid_positions(origin: Position) -> impl Iterator<Item = Self> {
        Direction::iter().filter_map(move |direction| Self::try_new(origin, direction))
    }

    fn is_valid(&self, input: &[[char; MAX_X_LENGTH]; MAX_Y_LENGTH]) -> bool {
        let mut position = self.origin;

        let word_iter = WORD_STR
            .chars()
            .map(|char| char.to_ascii_uppercase())
            .enumerate()
            .map(|(word_index, char)| (word_index == WORD_STR.len() - 1, char));

        let mut is_valid = false;

        for (is_last, char) in word_iter {
            if input[position.y][position.x] != char {
                break;
            }

            if is_last {
                is_valid = true;
                break;
            } else {
                position = position.unchecked_increment(self.direction);
            }
        }

        is_valid
    }
}

fn construct_input_array(input: &str) -> [[char; MAX_X_LENGTH]; MAX_Y_LENGTH] {
    let mut chars = [[char::default(); MAX_X_LENGTH]; MAX_Y_LENGTH];

    input.lines().enumerate().for_each(|(y_index, line)| {
        line.chars().enumerate().for_each(|(x_index, char)| {
            chars[y_index][x_index] = char.to_ascii_uppercase();
        });
    });

    chars
}

pub fn part_one(input_str: &str) -> Option<u32> {
    let input = construct_input_array(input_str);
    let mut occurances = 0u32;

    for (y_index, line) in input.iter().enumerate() {
        for (x_index, char) in line.iter().enumerate() {
            if *char != 'X' {
                continue;
            }

            let origin = Position::new(x_index, y_index);

            let valid_words = PotentialWord::valid_positions(origin)
                .filter(|potential_word| potential_word.is_valid(&input))
                .count();

            occurances += u32::from_usize(valid_words).unwrap_or(0);
        }
    }

    Some(occurances)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
