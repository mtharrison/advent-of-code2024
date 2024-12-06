use std::fmt::Display;

#[derive(Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

enum Result {
    Escaped,
    Loop,
}

impl Direction {
    fn rotate(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

#[derive(PartialEq, Clone)]
enum Cell {
    Guard(Direction),
    Vacant,
    Obstacle,
    Visited,
}

#[derive(Clone)]
struct World {
    cells: Vec<Vec<Cell>>,
    guard_position: Option<(usize, usize)>,
    guard_direction: Direction,
}

impl World {
    fn visited(&self) -> usize {
        self.cells
            .iter()
            .map(|row| row.iter().filter(|cell| **cell == Cell::Visited).count())
            .sum()
    }

    fn step(&mut self) {
        match self.guard_position {
            Some((i, j)) => {
                let direction = match &self.cells[i][j] {
                    Cell::Guard(dir) => dir.clone(),
                    _ => panic!("Invalid guard position"),
                };

                let (i_next, j_next) = {
                    let i = i as isize;
                    let j = j as isize;
                    match direction {
                        Direction::Up => (i - 1, j),
                        Direction::Down => (i + 1, j),
                        Direction::Left => (i, j - 1),
                        Direction::Right => (i, j + 1),
                    }
                };

                if i_next < 0
                    || i_next >= self.cells.len() as isize
                    || j_next < 0
                    || j_next >= self.cells[0].len() as isize
                {
                    self.cells[i][j] = Cell::Visited;
                    self.guard_position = None;
                    return;
                }

                let (i_next, j_next) = (i_next as usize, j_next as usize);
                let next_cell = &self.cells[i_next][j_next];

                match next_cell {
                    Cell::Vacant | Cell::Visited => {
                        self.cells[i][j] = Cell::Visited;
                        self.cells[i_next][j_next] = Cell::Guard(direction.clone());
                        self.guard_position = Some((i_next, j_next));
                    }
                    Cell::Obstacle => {
                        let new_direction = direction.rotate();
                        self.guard_direction = new_direction.clone();
                        self.cells[i][j] = Cell::Guard(new_direction.clone());
                    }
                    _ => panic!("Invalid cell"),
                }
            }
            None => {}
        }
    }

    fn play(&mut self) -> Result {
        let mut previous_guard_states = Vec::new();
        previous_guard_states.push((self.guard_position, self.guard_direction.clone()));
        while self.guard_position.is_some() {
            self.step();
            if let Some(guard_position) = self.guard_position {
                if previous_guard_states
                    .contains(&(Some(guard_position), self.guard_direction.clone()))
                {
                    return Result::Loop;
                }
                previous_guard_states.push((Some(guard_position), self.guard_direction.clone()));
            }
        }

        Result::Escaped
    }
}

impl From<&str> for World {
    fn from(input: &str) -> Self {
        let mut guard_position = None;
        let mut guard_direction = Direction::Up;

        let cells = input
            .lines()
            .enumerate()
            .map(|(i, line)| {
                line.chars()
                    .enumerate()
                    .map(|(j, c)| match c {
                        '^' | 'v' | '<' | '>' => {
                            guard_position = Some((i, j));
                            let dir = match c {
                                '^' => Direction::Up,
                                'v' => Direction::Down,
                                '<' => Direction::Left,
                                '>' => Direction::Right,
                                _ => unreachable!(),
                            };
                            guard_direction = dir.clone();
                            Cell::Guard(dir)
                        }
                        '.' => Cell::Vacant,
                        '#' => Cell::Obstacle,
                        'X' => Cell::Visited,
                        _ => panic!("Invalid character: {}", c),
                    })
                    .collect()
            })
            .collect();

        Self {
            cells,
            guard_position,
            guard_direction,
        }
    }
}

impl Display for World {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in &self.cells {
            for cell in row {
                match cell {
                    Cell::Guard(Direction::Up) => write!(f, "^")?,
                    Cell::Guard(Direction::Down) => write!(f, "v")?,
                    Cell::Guard(Direction::Left) => write!(f, "<")?,
                    Cell::Guard(Direction::Right) => write!(f, ">")?,
                    Cell::Vacant => write!(f, ".")?,
                    Cell::Obstacle => write!(f, "#")?,
                    Cell::Visited => write!(f, "X")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn parse_input(input: String) -> World {
    input.as_str().into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::parse as util_parse;

    #[test]
    fn test_example_part1() {
        let mut world = util_parse::<World>("day06", "example.txt", parse_input);
        world.play();
        assert_eq!(world.visited(), 41);
    }

    #[test]
    fn test_part1() {
        let mut world = util_parse::<World>("day06", "puzzle.txt", parse_input);
        world.play();
        assert_eq!(world.visited(), 5551);
    }

    #[test]
    fn test_example_part2() {
        let mut possible_obstacle_positions = 0;
        let world = util_parse::<World>("day06", "example.txt", parse_input);
        for i in 0..world.cells.len() {
            for j in 0..world.cells[0].len() {
                let mut world = world.clone();
                if world.cells[i][j] == Cell::Vacant {
                    world.cells[i][j] = Cell::Obstacle;
                    match world.play() {
                        Result::Loop => possible_obstacle_positions += 1,
                        _ => {}
                    }
                }
            }
        }

        assert_eq!(possible_obstacle_positions, 6);
    }

    #[test]
    fn test_part2() {
        // This is so obviously brute force that it's wrong, will revisit later
        let mut possible_obstacle_positions = 0;
        let world = util_parse::<World>("day06", "puzzle.txt", parse_input);
        for i in 0..world.cells.len() {
            for j in 0..world.cells[0].len() {
                let mut world = world.clone();
                if world.cells[i][j] == Cell::Vacant {
                    world.cells[i][j] = Cell::Obstacle;
                    match world.play() {
                        Result::Loop => possible_obstacle_positions += 1,
                        _ => {}
                    }
                }
            }
        }

        assert_eq!(possible_obstacle_positions, 1939);
    }
}
