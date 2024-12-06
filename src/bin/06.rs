use std::str::FromStr;

advent_of_code::solution!(6);

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "^" => Ok(Direction::Up),
            ">" => Ok(Direction::Right),
            "<" => Ok(Direction::Left),
            "v" => Ok(Direction::Down),
            _ => Err(()),
        }
    }
}

struct Guard {
    row: usize,
    column: usize,
    direction: Direction,
}

impl Guard {
    fn new(row: usize, column: usize) -> Self {
        Self {
            row,
            column,
            direction: Direction::Up,
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let input = input.lines().collect::<Vec<_>>();
    let mut guard_postion = None;

    for (row, l) in input.iter().enumerate() {
        for column in 0..l.len() {
            let s = dbg!(&l[column..column + 1]);
            let d = Direction::from_str(s);
            if let Ok(direction) = d {
                guard_postion = Some(Guard {
                    row,
                    column,
                    direction,
                });
                break;
            }
        }
    }

    None

    /*
    (0,0)  (0,column)
    (1,0)   (1,column)
    ....
    (row,0).....(row,column)
     */
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
