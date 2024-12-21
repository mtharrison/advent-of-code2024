use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
    i32,
};

use itertools::Position;

use crate::{grid::Grid, vec2d::Vec2d};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

type Pos = Vec2d<i32>;
type State = (Pos, Direction);
#[derive(Debug)]
struct Candidate(Pos, Direction, i32);

impl Ord for Candidate {
    fn cmp(&self, other: &Self) -> Ordering {
        other.2.cmp(&self.2) // Note: BinaryHeap is a max-heap, so we reverse the order
    }
}

impl PartialOrd for Candidate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Candidate {
    fn eq(&self, other: &Self) -> bool {
        self.2 == other.2
    }
}

impl Eq for Candidate {}

#[derive(Debug)]
pub struct Maze {
    grid: Grid<char>,
    source: State,
    target: Pos,
}

impl Maze {
    pub fn from(grid: Grid<char>) -> Self {
        let source = grid.find(&'S').unwrap();
        let target = grid.find(&'E').unwrap();
        Self {
            grid,
            source: (
                Vec2d::new(source.0 as i32, source.1 as i32),
                Direction::East,
            ),
            target: Vec2d::new(target.0 as i32, target.1 as i32),
        }
    }

    // returns the next states from the current state
    pub fn neighbours(&self, state: &State) -> Vec<(State, i32)> {
        let (pos, dir) = state;
        let mut neighbours = vec![];

        // move forward
        let next_pos = match dir {
            Direction::North => *pos + Vec2d::new(0, -1),
            Direction::East => *pos + Vec2d::new(1, 0),
            Direction::South => *pos + Vec2d::new(0, 1),
            Direction::West => *pos + Vec2d::new(-1, 0),
        };

        if self.grid.get(next_pos.x as usize, next_pos.y as usize) != Some(&'#') {
            neighbours.push(((next_pos, dir.clone()), 1));
        }

        // turn clockwise
        let next_dir = match dir {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };
        neighbours.push(((*pos, next_dir), 1000));

        // turn anti-clockwise
        let next_dir = match dir {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        };
        neighbours.push(((*pos, next_dir), 1000));

        neighbours
    }

    // returns the next states from the current state
    pub fn neighbours_rev(&self, state: &State) -> Vec<(State, i32)> {
        let (pos, dir) = state;
        let mut neighbours = vec![];

        // move backward
        let next_pos = match dir {
            Direction::North => *pos - Vec2d::new(0, -1),
            Direction::East => *pos - Vec2d::new(1, 0),
            Direction::South => *pos - Vec2d::new(0, 1),
            Direction::West => *pos - Vec2d::new(-1, 0),
        };

        if self.grid.get(next_pos.x as usize, next_pos.y as usize) != Some(&'#') {
            neighbours.push(((next_pos, dir.clone()), 1));
        }

        // turn clockwise
        let next_dir = match dir {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };
        neighbours.push(((*pos, next_dir), 1000));

        // turn anti-clockwise
        let next_dir = match dir {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        };
        neighbours.push(((*pos, next_dir), 1000));

        neighbours
    }
}

// Dijkstra's algorithm
pub fn dijkstra(maze: &Maze) -> HashMap<(Vec2d<i32>, Direction), i32> {
    let mut unvisited = BinaryHeap::new();
    let mut dist = HashMap::new();
    let mut visited = HashMap::new();

    for i in 0..maze.grid.width() {
        for j in 0..maze.grid.height() {
            if maze.grid.get(i, j).unwrap() == &'#' {
                continue;
            }

            let pos = Vec2d {
                x: i as i32,
                y: j as i32,
            };

            for &dir in &[
                Direction::East,
                Direction::West,
                Direction::North,
                Direction::South,
            ] {
                dist.insert((pos, dir), i32::MAX);
            }
        }
    }

    dist.insert((maze.source.0, maze.source.1), 0);
    unvisited.push(Candidate(maze.source.0, maze.source.1, 0));

    while let Some(Candidate(u_pos, u_dir, u_dist)) = unvisited.pop() {
        if visited.contains_key(&(u_pos, u_dir)) {
            continue;
        }
        visited.insert((u_pos, u_dir), true);

        for ((v_pos, v_dir), weight) in maze.neighbours(&(u_pos, u_dir)) {
            let new_dist = u_dist + weight;

            if new_dist < *dist.get(&(v_pos, v_dir)).unwrap_or(&i32::MAX) {
                dist.insert((v_pos, v_dir), new_dist);
                unvisited.push(Candidate(v_pos, v_dir, new_dist));
            }
        }
    }

    dist
}

pub fn count_best_path_tiles(
    maze: &Maze,
    target: State,
    dist: HashMap<(Vec2d<i32>, Direction), i32>,
) -> usize {
    // Backtrack from the destination to find all best-path tiles
    let mut best_path_tiles = HashSet::new();
    let mut stack = vec![target];

    while let Some(tile) = stack.pop() {
        if best_path_tiles.contains(&tile) {
            continue; // Avoid re-processing
        }

        best_path_tiles.insert(tile);

        // Check neighbors to see which ones contributed to this tile's shortest path
        for (neighbor, weight) in maze.neighbours_rev(&tile) {
            if let Some(&neighbor_dist) = dist.get(&neighbor) {
                if neighbor_dist + weight == *dist.get(&tile).unwrap_or(&i32::MAX) {
                    stack.push(neighbor);
                }
            }
        }
    }

    // dedupe
    let mut tiles_pos_only = HashSet::new();
    for (pos, _) in best_path_tiles.iter() {
        tiles_pos_only.insert(pos);
    }

    tiles_pos_only.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::parse as util_parse;

    #[test]
    fn test_example_part1() {
        let maze = util_parse::<Maze>("day16", "example.txt", |input| {
            Maze::from(Grid::from(input))
        });

        let distances = dijkstra(&maze);

        let mut min_dist = i32::MAX;

        for &dir in &[
            Direction::East,
            Direction::West,
            Direction::North,
            Direction::South,
        ] {
            let d = distances.get(&(maze.target, dir)).unwrap();
            if *d < min_dist {
                min_dist = *d;
            }
        }

        assert_eq!(min_dist, 7036)
    }

    #[test]
    fn test_part1() {
        let maze = util_parse::<Maze>("day16", "puzzle.txt", |input| Maze::from(Grid::from(input)));

        let distances = dijkstra(&maze);

        let mut min_dist = i32::MAX;

        for &dir in &[
            Direction::East,
            Direction::West,
            Direction::North,
            Direction::South,
        ] {
            let d = distances.get(&(maze.target, dir)).unwrap();
            if *d < min_dist {
                min_dist = *d;
            }
        }

        assert_eq!(min_dist, 107512)
    }

    #[test]
    fn test_example_part2() {
        let maze = util_parse::<Maze>("day16", "example.txt", |input| {
            Maze::from(Grid::from(input))
        });

        let distances = dijkstra(&maze);

        let mut min_dist = i32::MAX;
        let mut final_position = (maze.target, Direction::East);

        for &dir in &[
            Direction::East,
            Direction::West,
            Direction::North,
            Direction::South,
        ] {
            let d = distances.get(&(maze.target, dir)).unwrap();
            if *d < min_dist {
                min_dist = *d;
                final_position.1 = dir;
            }
        }

        let best_paths_tiles_count = count_best_path_tiles(&maze, final_position, distances);
        assert_eq!(best_paths_tiles_count, 45);
    }

    #[test]
    fn test_part2() {
        let maze = util_parse::<Maze>("day16", "puzzle.txt", |input| Maze::from(Grid::from(input)));

        let distances = dijkstra(&maze);

        let mut min_dist = i32::MAX;
        let mut final_position = (maze.target, Direction::East);

        for &dir in &[
            Direction::East,
            Direction::West,
            Direction::North,
            Direction::South,
        ] {
            let d = distances.get(&(maze.target, dir)).unwrap();
            if *d < min_dist {
                min_dist = *d;
                final_position.1 = dir;
            }
        }

        let best_paths_tiles_count = count_best_path_tiles(&maze, final_position, distances);
        assert_eq!(best_paths_tiles_count, 561);
    }
}
