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

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}
