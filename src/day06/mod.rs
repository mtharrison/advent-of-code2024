use std::{collections::HashSet, fmt::Display};

#[derive(Clone, PartialEq, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, PartialEq, Debug)]
pub enum Result {
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
pub enum Cell {
    Guard(Direction),
    Vacant,
    Obstacle,
    Visited,
}

#[derive(Clone)]
pub struct World {
    cells: Vec<Vec<Cell>>,
    guard_position: Option<(usize, usize)>,
    guard_direction: Direction,
}

impl World {
    pub fn step(&mut self) {
        match self.guard_position {
            Some((i, j)) => {
                let direction = self.guard_direction.clone();

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

    pub fn place_obstacle(&mut self, i: usize, j: usize) {
        self.cells[i][j] = Cell::Obstacle;
    }

    pub fn play(
        &mut self,
    ) -> (
        Result,
        Vec<(usize, usize, Direction)>,
        HashSet<(usize, usize)>,
    ) {
        let mut visited_with_dir = Vec::new();
        let mut visited = HashSet::new();

        while let Some((i, j)) = self.guard_position {
            let state = (i, j, self.guard_direction.clone());
            if visited_with_dir.contains(&state) {
                return (Result::Loop, visited_with_dir, visited);
            }
            visited_with_dir.push(state);
            visited.insert((i, j));
            self.step();
        }

        (Result::Escaped, visited_with_dir, visited)
    }

    pub fn place_guard(&mut self, i: usize, j: usize, dir: Direction) {
        if let Some((i, j)) = self.guard_position {
            self.cells[i][j] = Cell::Vacant;
        }
        self.cells[i][j] = Cell::Guard(dir.clone());
        self.guard_position = Some((i, j));
        self.guard_direction = dir.clone();
    }

    pub fn remove_obstacle(&mut self, i: usize, j: usize) {
        self.cells[i][j] = Cell::Vacant;
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
    use core::num;

    use super::*;
    use crate::util::parse as util_parse;

    #[test]
    fn test_example_part1() {
        let mut world = util_parse::<World>("day06", "example.txt", parse_input);
        let (_, _, visited) = world.play();
        assert_eq!(visited.len(), 41);
    }

    #[test]
    fn test_part1() {
        let mut world = util_parse::<World>("day06", "puzzle.txt", parse_input);
        let (_, _, visited) = world.play();
        assert_eq!(visited.len(), 5551);
    }

    #[test]
    fn test_example_part2() {
        let mut world = util_parse::<World>("day06", "example.txt", parse_input);
        let clean_world = world.clone();
        let (_, visited, _) = world.play();
        let mut obstacle_pos = HashSet::new();
        for i in 1..visited.len() {
            let mut new_world = clean_world.clone();
            let (i_obj, j_obj, _) = &visited[i];
            new_world.place_obstacle(*i_obj, *j_obj);

            let (result, _, _) = new_world.play();
            if result == Result::Loop {
                obstacle_pos.insert((*i_obj, *j_obj));
            }
        }

        assert_eq!(obstacle_pos.len(), 6);
    }

    #[test]
    fn test_part2() {
        let mut world = util_parse::<World>("day06", "puzzle.txt", parse_input);
        let clean_world = world.clone();
        let (_, visited, _) = world.play();
        let mut obstacle_pos = HashSet::new();
        for i in 1..visited.len() {
            let mut new_world = clean_world.clone();
            let (i_obj, j_obj, _) = &visited[i];
            new_world.place_obstacle(*i_obj, *j_obj);

            let (result, _, _) = new_world.play();
            if result == Result::Loop {
                obstacle_pos.insert((*i_obj, *j_obj));
            }
        }

        assert_eq!(obstacle_pos.len(), 1939);
    }
}
