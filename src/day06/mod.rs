use ahash::{HashSet, HashSetExt};

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
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

#[derive(PartialEq, Clone)]
pub enum Cell {
    Guard,
    Vacant,
    Obstacle,
    Visited,
}

#[derive(Clone)]
pub struct GuardState {
    position: (usize, usize),
    direction: Direction,
}

#[derive(Clone)]
pub struct World {
    cells: Vec<Vec<Cell>>,
    guard: Option<GuardState>,
    initial_guard: Option<GuardState>,
    initial_cells: Vec<Vec<Cell>>,
}

impl World {
    pub fn step(&mut self) {
        match self.guard {
            Some(ref mut guard) => {
                let (i, j) = guard.position;
                let direction = guard.direction;

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
                    self.guard = None;
                    return;
                }

                let (i_next, j_next) = (i_next as usize, j_next as usize);
                let next_cell = &self.cells[i_next][j_next];

                match next_cell {
                    Cell::Vacant | Cell::Visited => {
                        self.cells[i][j] = Cell::Visited;
                        self.cells[i_next][j_next] = Cell::Guard;
                        self.guard = Some(GuardState {
                            position: (i_next, j_next),
                            direction,
                        });
                    }
                    Cell::Obstacle => {
                        let new_direction = match direction {
                            Direction::Up => Direction::Right,
                            Direction::Right => Direction::Down,
                            Direction::Down => Direction::Left,
                            Direction::Left => Direction::Up,
                        };
                        self.guard = Some(GuardState {
                            position: (i, j),
                            direction: new_direction,
                        });
                        self.cells[i][j] = Cell::Guard;
                    }
                    _ => panic!("Invalid cell"),
                }
            }
            None => {}
        }
    }

    pub fn reset(&mut self) {
        self.cells = self.initial_cells.clone();
        self.guard = self.initial_guard.clone();
    }

    pub fn place_obstacle(&mut self, i: usize, j: usize) {
        self.cells[i][j] = Cell::Obstacle;
    }

    pub fn remove_obstacle(&mut self, i: usize, j: usize) {
        self.cells[i][j] = Cell::Vacant;
    }

    pub fn play(&mut self) -> (Result, HashSet<(usize, usize)>) {
        let max_hashset_size = self.cells.len() * self.cells[0].len();
        let mut visited_with_dir = HashSet::with_capacity(max_hashset_size);
        let mut visited = HashSet::with_capacity(max_hashset_size);

        while let Some(ref guard) = self.guard {
            let state = (guard.position, guard.direction);
            if visited_with_dir.contains(&state) {
                return (Result::Loop, visited);
            }
            visited_with_dir.insert(state);
            visited.insert(guard.position);
            self.step();
        }

        (Result::Escaped, visited)
    }
}

impl From<&str> for World {
    fn from(input: &str) -> Self {
        let mut guard = None;

        let cells: Vec<_> = input
            .lines()
            .enumerate()
            .map(|(i, line)| {
                line.chars()
                    .enumerate()
                    .map(|(j, c)| match c {
                        '^' | 'v' | '<' | '>' => {
                            let dir = match c {
                                '^' => Direction::Up,
                                'v' => Direction::Down,
                                '<' => Direction::Left,
                                '>' => Direction::Right,
                                _ => unreachable!(),
                            };
                            guard = Some(GuardState {
                                position: (i, j),
                                direction: dir,
                            });
                            Cell::Guard
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
            initial_cells: cells.clone(),
            cells,
            initial_guard: guard.clone(),
            guard,
        }
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
        let (_, visited) = world.play();
        assert_eq!(visited.len(), 41);
    }

    #[test]
    fn test_part1() {
        let mut world = util_parse::<World>("day06", "puzzle.txt", parse_input);
        let (_, visited) = world.play();
        assert_eq!(visited.len(), 5551);
    }

    #[test]
    fn test_example_part2() {
        let mut world = util_parse::<World>("day06", "example.txt", parse_input);
        let (_, visited) = world.play();
        let mut obstacle_pos = HashSet::default();
        for (i, j) in visited.iter() {
            world.reset();
            world.place_obstacle(*i, *j);

            let (result, _) = world.play();
            if result == Result::Loop {
                obstacle_pos.insert((*i, *j));
            }
            world.remove_obstacle(*i, *j);
        }

        assert_eq!(obstacle_pos.len(), 6);
    }

    #[test]
    fn test_part2() {
        let mut world = util_parse::<World>("day06", "puzzle.txt", parse_input);
        let (_, visited) = world.play();
        let mut obstacle_pos = HashSet::default();
        for (i, j) in visited.iter() {
            world.reset();
            world.place_obstacle(*i, *j);

            let (result, _) = world.play();
            if result == Result::Loop {
                obstacle_pos.insert((*i, *j));
            }
            world.remove_obstacle(*i, *j);
        }

        assert_eq!(obstacle_pos.len(), 1939);
    }
}
