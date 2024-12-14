use crate::{grid::Grid, vec2d::Vec2d};
use regex::Regex;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Robot {
    pos: Vec2d<i64>,
    vel: Vec2d<i64>,
}

#[derive(Debug, Clone)]
pub struct Map {
    robots: Vec<Robot>,
    width: u32,
    height: u32,
}

impl Map {
    pub fn new(robots: Vec<Robot>, width: u32, height: u32) -> Self {
        Map {
            robots,
            width,
            height,
        }
    }

    pub fn step(&mut self) {
        for robot in &mut self.robots {
            robot.pos = {
                let x = ((robot.pos.x + (self.width as i64)) + robot.vel.x) % (self.width as i64);
                let y = ((robot.pos.y + (self.height as i64)) + robot.vel.y) % (self.height as i64);
                Vec2d::new(x, y)
            };
        }
    }

    pub fn step_n_times(&mut self, n: u32) {
        for _ in 0..n {
            self.step();
        }
    }

    fn count_quadrants(&self) -> [i64; 4] {
        let mut quad_counts = [0; 4];
        for robot in &self.robots {
            let left_quadrant = robot.pos.x < self.width as i64 / 2;
            let right_quadrant = robot.pos.x >= self.width.div_ceil(2) as i64;
            let top_quadrant = robot.pos.y < self.height as i64 / 2;
            let bottom_quadrant = robot.pos.y >= self.height.div_ceil(2) as i64;
            match (left_quadrant, right_quadrant, top_quadrant, bottom_quadrant) {
                (true, false, true, false) => quad_counts[0] += 1,
                (false, true, true, false) => quad_counts[1] += 1,
                (true, false, false, true) => quad_counts[2] += 1,
                (false, true, false, true) => quad_counts[3] += 1,
                _ => (),
            }
        }

        quad_counts
    }

    pub fn safety_factor(&self) -> i64 {
        let quad_counts = self.count_quadrants();
        quad_counts.iter().fold(1, |acc, x| acc * x)
    }

    pub fn as_grid(&self) -> Grid<char> {
        let mut grid = vec!['.'; (self.width * self.height) as usize];
        for robot in &self.robots {
            let idx = (robot.pos.y * self.width as i64 + robot.pos.x) as usize;
            grid[idx] = '#';
        }

        Grid::new(grid, self.width as usize)
    }
}

impl Grid<char> {
    // a heuristic to determine how "regular" the grid is by counting how many
    // robots are next to another robot - assuming we're looking for a filled
    // xmas tree(?) this should be a high number
    pub fn regularity_score(&self) -> i64 {
        let mut score = 0;
        let rows = self.rows();
        for row in rows {
            for j in 1..row.len() {
                if row[j] == '#' && row[j - 1] == '#' {
                    score += 1;
                }
            }
        }

        score
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut grid = vec![vec!['.'; self.width as usize]; self.height as usize];
        for robot in &self.robots {
            let curr = grid[robot.pos.y as usize][robot.pos.x as usize];
            match curr {
                '.' => grid[robot.pos.y as usize][robot.pos.x as usize] = '#',
                _ => {}
            }
        }

        for row in grid {
            writeln!(f, "{}", row.iter().collect::<String>())?;
        }

        Ok(())
    }
}

pub fn parse_input(input: String) -> Vec<Robot> {
    let mut lines = input.lines();
    let mut robots = Vec::new();
    let re = Regex::new(r"p\=(?<x>\d+),(?<y>\d+) v\=(?<vx>-?\d+),(?<vy>-?\d+)").unwrap();

    while let Some(line) = lines.next() {
        let caps = re.captures(line).unwrap();
        let x = caps["x"].parse::<i64>().unwrap();
        let y = caps["y"].parse::<i64>().unwrap();
        let vx = caps["vx"].parse::<i64>().unwrap();
        let vy = caps["vy"].parse::<i64>().unwrap();
        let pos = Vec2d::new(x, y);
        let vel = Vec2d::new(vx, vy);
        robots.push(Robot { pos, vel });
    }

    robots
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::parse as util_parse;

    #[test]
    fn test_example_part1() {
        let robots = util_parse::<Vec<Robot>>("day14", "example.txt", parse_input);
        let mut map = Map::new(robots, 11, 7);
        map.step_n_times(100);
        assert_eq!(map.safety_factor(), 12);
    }

    #[test]
    fn test_part1() {
        let robots = util_parse::<Vec<Robot>>("day14", "puzzle.txt", parse_input);
        let mut map = Map::new(robots, 101, 103);
        map.step_n_times(100);
        assert_eq!(map.safety_factor(), 223020000);
    }

    #[test]
    fn test_part2() {
        let robots = util_parse::<Vec<Robot>>("day14", "puzzle.txt", parse_input);
        let mut map = Map::new(robots, 101, 103);

        let mut max = (0, 0);
        for i in 1..10_000 {
            map.step();
            let score = map.as_grid().regularity_score();
            if score > max.1 {
                max = (i, score);
            }
        }
        // comfirmed by visual inspection
        assert_eq!(max.0, 7338);
    }
}
