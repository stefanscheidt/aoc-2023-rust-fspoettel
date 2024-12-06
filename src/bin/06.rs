use std::collections::HashSet;
use std::str::FromStr;

advent_of_code::solution!(6);

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
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

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
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

pub fn part_one(input: &str) -> Option<usize> {
    let input = input.lines().collect::<Vec<_>>();
    let mut guard = None;

    'rowloop: for (row, l) in input.iter().enumerate() {
        for column in 0..l.len() {
            let s = &l[column..column + 1];
            let d = Direction::from_str(s);
            if let Ok(direction) = d {
                guard = Some(Guard {
                    row,
                    column,
                    direction,
                });
                break 'rowloop;
            }
        }
    }

    let mut guard = guard.unwrap();
    let mut positions = HashSet::new();
    positions.insert((guard.row, guard.column));

    loop {
        if guard.row == 0
            || guard.row == input.len() - 1
            || guard.column == 0
            || guard.column == input[guard.row].len() - 1
        {
            break;
        }

        let (next_row, next_column) = match guard.direction {
            Direction::Up => (guard.row - 1, guard.column),
            Direction::Down => (guard.row + 1, guard.column),
            Direction::Left => (guard.row, guard.column - 1),
            Direction::Right => (guard.row, guard.column + 1),
        };

        if &input[next_row][next_column..next_column + 1] == "#" {
            guard.direction = guard.direction.turn_right()
        } else {
            guard.row = next_row;
            guard.column = next_column;
            positions.insert((next_row, next_column));
        }
    }

    Some(positions.len())

    /*
    (0,0)  (0,column)
    (1,0)   (1,column)
    ....
    (row,0).....(row,column)
     */
}

pub fn part_two(input: &str) -> Option<usize> {
    let input = input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<_>>>();
    let mut guard = None;

    'rowloop: for (row, l) in input.iter().enumerate() {
        for (column, c) in l.iter().enumerate() {
            let d = Direction::from_str(&c.to_string());
            if let Ok(direction) = d {
                guard = Some(Guard {
                    row,
                    column,
                    direction,
                });
                break 'rowloop;
            }
        }
    }

    let mut guard = guard.unwrap();
    let mut guardTrace = HashSet::new();
    guardTrace.insert(guard);

    loop {
        if guard.row == 0
            || guard.row == input.len() - 1
            || guard.column == 0
            || guard.column == input[guard.row].len() - 1
        {
            guardTrace.remove(&guard);
            break;
        }

        let (next_row, next_column) = match guard.direction {
            Direction::Up => (guard.row - 1, guard.column),
            Direction::Down => (guard.row + 1, guard.column),
            Direction::Left => (guard.row, guard.column - 1),
            Direction::Right => (guard.row, guard.column + 1),
        };

        if input[next_row][next_column] == '#' {
            guard.direction = guard.direction.turn_right()
        } else {
            guard.row = next_row;
            guard.column = next_column;
            guardTrace.insert(guard);
        }
    }

    let mut valid_obstacle_positions = HashSet::new();
    for trace in guardTrace.iter() {
        let mut test_input = input.clone();
        let (next_row, next_column) = match trace.direction {
            Direction::Up => (guard.row - 1, guard.column),
            Direction::Down => (guard.row + 1, guard.column),
            Direction::Left => (guard.row, guard.column - 1),
            Direction::Right => (guard.row, guard.column + 1),
        };
        test_input[next_row][next_column] = '#';

        let test_trace = guardTrace.clone();
        loop {
            if guard.row == 0
                || guard.row == input.len() - 1
                || guard.column == 0
                || guard.column == input[guard.row].len() - 1
            {
                guardTrace.remove(&guard);
                break;
            }

            let (next_row, next_column) = match guard.direction {
                Direction::Up => (guard.row - 1, guard.column),
                Direction::Down => (guard.row + 1, guard.column),
                Direction::Left => (guard.row, guard.column - 1),
                Direction::Right => (guard.row, guard.column + 1),
            };

            if input[next_row][next_column] == '#' {
                guard.direction = guard.direction.turn_right()
            } else {
                guard.row = next_row;
                guard.column = next_column;
                guardTrace.insert(guard);
            }
        }


    }

    Some(guardTrace.len())
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
