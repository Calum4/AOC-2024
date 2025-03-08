use std::cmp::PartialEq;

advent_of_code::solution!(6);

const MAX_X_LENGTH: usize = if !cfg!(test) { 130 } else { 10 };
const MAX_Y_LENGTH: usize = if !cfg!(test) { 130 } else { 10 };

#[derive(Copy, Clone, Debug)]
enum Heading {
    North,
    East,
    South,
    West,
}

impl Heading {
    fn turn_clockwise_90(self) -> Self {
        match self {
            Heading::North => Heading::East,
            Heading::East => Heading::South,
            Heading::South => Heading::West,
            Heading::West => Heading::North,
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Position {
    x: usize,
    y: usize,
    heading: Heading,
}

impl Position {
    fn new(x: usize, y: usize, heading: Heading) -> Self {
        Self {
            x,
            y,
            heading,
        }
    }

    fn advance(self, obstacles: &[[bool; MAX_Y_LENGTH]; MAX_X_LENGTH]) -> Option<(Self, bool)> {
        let (x, y) = match self.heading {
            Heading::North => (self.x, self.y.checked_sub(1)?),
            Heading::East => (self.x + 1, self.y),
            Heading::South => (self.x, self.y + 1),
            Heading::West => (self.x.checked_sub(1)?, self.y),
        };

        if x >= MAX_X_LENGTH || y >= MAX_Y_LENGTH {
            return None;
        }

        if obstacles[y][x] {
            Some((
                Self {
                    x: self.x,
                    y: self.y,
                    heading: self.heading.turn_clockwise_90(),
                },
                false
            ))
        } else {
            Some((
                Self {
                    x,
                    y,
                    heading: self.heading,
                },
                true
            ))
        }
    }

    fn advance_pt2(self, obstacles: &mut [Vec<Option<Obstacle>>]) -> Option<(Self, bool)> {
        let (x, y) = match self.heading {
            Heading::North => (self.x, self.y.checked_sub(1)?),
            Heading::East => (self.x + 1, self.y),
            Heading::South => (self.x, self.y + 1),
            Heading::West => (self.x.checked_sub(1)?, self.y),
        };

        if x >= MAX_X_LENGTH || y >= MAX_Y_LENGTH {
            return None;
        }

        match &mut obstacles[y][x] {
            None => {
                let position = Self {
                    x,
                    y,
                    heading: self.heading,
                };

                Some((position, false))
            }
            Some(obstacle) => {
                let side_hit = Side::from(self.heading);
                let is_loop = obstacle.register_hit(side_hit);

                let position = Self {
                    x: self.x,
                    y: self.y,
                    heading: self.heading.turn_clockwise_90(),
                };

                Some((position, is_loop))
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut obstacles = [[false; MAX_Y_LENGTH]; MAX_X_LENGTH];
    let mut start_position: Option<Position> = None;

    input.as_bytes().split(|byte| *byte == b'\n').enumerate().for_each(|(y_index, line)| {
        line.iter().enumerate().for_each(|(x_index, byte)| {
            if *byte == b'#' {
                obstacles[y_index][x_index] = true;
            } else if start_position.is_none() && *byte == b'^' {
                start_position = Some(Position::new(x_index, y_index, Heading::North));
            }
        });
    });

    let mut current_position = start_position.unwrap();
    let mut visited_positions = [[false; MAX_Y_LENGTH]; MAX_X_LENGTH];
    let mut visited_positions_sum: u32 = 1;

    visited_positions[current_position.y][current_position.x] = true;

    while let Some((new_position, did_advance)) = current_position.advance(&obstacles) {
        current_position = new_position;

        if did_advance && !visited_positions[new_position.y][new_position.x] {
            visited_positions[new_position.y][new_position.x] = true;
            visited_positions_sum += 1;
        }
    }

    Some(visited_positions_sum)
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Side {
    Top,
    Right,
    Bottom,
    Left,
}

impl From<Heading> for Side {
    fn from(heading: Heading) -> Self {
        match heading {
            Heading::North => Self::Bottom,
            Heading::East => Self::Left,
            Heading::South => Self::Top,
            Heading::West => Self::Right,
        }
    }
}

#[derive(Clone, Debug)]
struct Obstacle {
    hits: [Option<Side>; 4],
}

impl Obstacle {
    fn new() -> Self {
        Obstacle {
            hits: [None; 4],
        }
    }

    fn register_hit(&mut self, side: Side) -> bool {
        for hit in &mut self.hits {
            match hit {
                None => {
                    let _ = hit.insert(side);
                    return false
                }
                Some(prev_hit) => {
                    if *prev_hit == side {
                        return true
                    }
                }
            }
        };

        panic!("This should not be possible!");
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut obstacles = vec![vec![None; MAX_X_LENGTH]; MAX_Y_LENGTH];
    let mut start_position: Option<Position> = None;

    input.as_bytes().split(|byte| *byte == b'\n').enumerate().for_each(|(y_index, line)| {
        line.iter().enumerate().for_each(|(x_index, byte)| {
            if *byte == b'#' {
                obstacles[y_index][x_index] = Some(Obstacle::new());
            } else if start_position.is_none() && *byte == b'^' {
                start_position = Some(Position::new(x_index, y_index, Heading::North));
            }
        });
    });

    let mut loops = 0_u32;

    for y in 0..MAX_Y_LENGTH {
        for x in 0..MAX_X_LENGTH {
            let mut current_position = start_position.unwrap();

            if (y == current_position.y && x == current_position.x) || obstacles[y][x].is_some() {
                continue;
            }

            let mut obstacles = {
                let mut obstacles = obstacles.clone();
                obstacles[y][x] = Some(Obstacle::new());

                obstacles
            };

            while let Some((position, is_loop)) = current_position.advance_pt2(&mut obstacles) {
                current_position = position;

                if is_loop {
                    loops += 1;
                    break;
                }
            }
        }
    }

    Some(loops)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
