use num_traits::FromPrimitive;
use strum::EnumIter;

advent_of_code::solution!(4);

const MAX_X_LENGTH: usize = if !cfg!(test) { 140 } else { 10 };
const MAX_Y_LENGTH: usize = if !cfg!(test) { 140 } else { 10 };

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

    fn unchecked_increment(&self, direction: Direction, increment: usize) -> Self {
        let mut position = *self;

        match direction {
            Direction::North => position.y -= increment,
            Direction::NorthEast => {
                position.x += increment;
                position.y -= increment;
            }
            Direction::East => position.x += increment,
            Direction::SouthEast => {
                position.x += increment;
                position.y += increment;
            }
            Direction::South => position.y += increment,
            Direction::SouthWest => {
                position.x -= increment;
                position.y += increment;
            }
            Direction::West => position.x -= increment,
            Direction::NorthWest => {
                position.x -= increment;
                position.y -= increment;
            }
        }

        position
    }
}

mod part_1 {
    use super::*;
    use std::iter::Iterator;
    use strum::IntoEnumIterator;

    pub(super) const WORD_STR: &str = "XMAS";

    #[derive(Copy, Clone, Debug)]
    pub(super) struct PotentialWord {
        origin: Position,
        direction: Direction,
    }

    impl PotentialWord {
        fn try_new(position: Position, direction: Direction) -> Option<PotentialWord> {
            const OFFSET: usize = WORD_STR.len() - 1;
            let mut position_clone = position;

            match direction {
                Direction::North => position_clone.y = position_clone.y.checked_sub(OFFSET)?,
                Direction::NorthEast => {
                    position_clone.x = position_clone.x.checked_add(OFFSET)?;
                    position_clone.y = position_clone.y.checked_sub(OFFSET)?;
                }
                Direction::East => position_clone.x = position_clone.x.checked_add(OFFSET)?,
                Direction::SouthEast => {
                    position_clone.x = position_clone.x.checked_add(OFFSET)?;
                    position_clone.y = position_clone.y.checked_add(OFFSET)?;
                }
                Direction::South => position_clone.y = position_clone.y.checked_add(OFFSET)?,
                Direction::SouthWest => {
                    position_clone.x = position_clone.x.checked_sub(OFFSET)?;
                    position_clone.y = position_clone.y.checked_add(OFFSET)?;
                }
                Direction::West => position_clone.x = position_clone.x.checked_sub(OFFSET)?,
                Direction::NorthWest => {
                    position_clone.x = position_clone.x.checked_sub(OFFSET)?;
                    position_clone.y = position_clone.y.checked_sub(OFFSET)?;
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

        pub(super) fn valid_positions(origin: Position) -> impl Iterator<Item = Self> {
            Direction::iter().filter_map(move |direction| Self::try_new(origin, direction))
        }

        pub(super) fn is_valid(&self, input: &[[char; MAX_X_LENGTH]; MAX_Y_LENGTH]) -> bool {
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
                    position = position.unchecked_increment(self.direction, 1);
                }
            }

            is_valid
        }
    }
}

pub fn part_one(input_str: &str) -> Option<u32> {
    use part_1::*;

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

mod part_2 {
    use super::*;

    // `.len()` should be odd
    const WORD_STR: &str = "MAS";
    // noinspection RsAssertEqual - `assert_ne!()` is not const
    const _: () = {
        assert!((WORD_STR.len() & 1) != 0);
    };

    const OFFSET: usize = (WORD_STR.len() - 1) / 2;

    pub(super) struct PotentialMas {
        origin: Position,
    }

    impl PotentialMas {
        pub(super) fn try_new(origin: Position) -> Option<PotentialMas> {
            origin.x.checked_sub(OFFSET)?;
            origin.y.checked_sub(OFFSET)?;

            let max_x = origin.x.checked_add(OFFSET)?;
            let max_y = origin.y.checked_add(OFFSET)?;

            if max_x >= MAX_X_LENGTH || max_y >= MAX_Y_LENGTH {
                None
            } else {
                Some(PotentialMas { origin })
            }
        }

        pub(super) fn is_valid(&self, input: &[[char; MAX_X_LENGTH]; MAX_Y_LENGTH]) -> bool {
            let get_char = |direction: Direction| {
                let position = self.origin.unchecked_increment(direction, OFFSET);
                input[position.y][position.x]
            };

            let nw = get_char(Direction::NorthWest);
            let se = get_char(Direction::SouthEast);
            if !matches!([nw, se], ['M', 'S'] | ['S', 'M']) {
                return false;
            }

            let ne = get_char(Direction::NorthEast);
            let sw = get_char(Direction::SouthWest);
            matches!([ne, sw], ['M', 'S'] | ['S', 'M'])
        }
    }
}

pub fn part_two(input_str: &str) -> Option<u32> {
    use part_2::*;

    let input = construct_input_array(input_str);
    let mut occurances = 0u32;

    for (y_index, line) in input.iter().enumerate() {
        for (x_index, char) in line.iter().enumerate() {
            if *char != 'A' {
                continue;
            }

            let origin = Position::new(x_index, y_index);
            let Some(potential_mas) = PotentialMas::try_new(origin) else {
                continue;
            };

            if potential_mas.is_valid(&input) {
                occurances += 1;
            }
        }
    }

    Some(occurances)
}

fn construct_input_array(input: &str) -> [[char; MAX_X_LENGTH]; MAX_Y_LENGTH] {
    let mut chars = [[char::default(); MAX_X_LENGTH]; MAX_Y_LENGTH];

    // TODO - Swap to bytes instead of chars?
    input.lines().enumerate().for_each(|(y_index, line)| {
        line.chars().enumerate().for_each(|(x_index, char)| {
            chars[y_index][x_index] = char.to_ascii_uppercase();
        });
    });

    chars
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
        assert_eq!(result, Some(9));
    }
}
