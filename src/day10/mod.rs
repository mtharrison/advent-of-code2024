use crate::grid::Grid;
use std::collections::HashSet;

type ScoreFn = fn(&Grid<char>, (usize, usize)) -> usize;

pub fn find_trails(
    grid: &Grid<char>,
    pos: (usize, usize),
    next_height: usize,
    reachable: &mut HashSet<(usize, usize)>,
) {
    for neighbour in grid.neighbours(pos) {
        let next = grid.get(neighbour.1, neighbour.0).unwrap();
        let expected = next_height.to_string().chars().next().unwrap();
        if next == &expected {
            if next_height == 9 {
                reachable.insert(neighbour);
            } else {
                find_trails(grid, neighbour, next_height + 1, reachable);
            }
        }
    }
}

pub fn distinct_trails(grid: &Grid<char>, pos: (usize, usize), next_height: usize) -> usize {
    let mut count = 0;
    for neighbour in grid.neighbours(pos) {
        let next = grid.get(neighbour.1, neighbour.0).unwrap();
        let expected = next_height.to_string().chars().next().unwrap();
        if next == &expected {
            if next_height == 9 {
                count += 1;
            } else {
                count += distinct_trails(grid, neighbour, next_height + 1);
            }
        }
    }
    count
}

pub fn trailhead_score(grid: &Grid<char>, pos: (usize, usize)) -> usize {
    let (row, col) = pos;
    let value = grid.get(col, row).unwrap();
    if value != &'0' {
        0
    } else {
        let mut reachable_positions: HashSet<(usize, usize)> = HashSet::new();
        find_trails(grid, pos, 1, &mut reachable_positions);
        reachable_positions.len()
    }
}

pub fn trailhead_rating(grid: &Grid<char>, pos: (usize, usize)) -> usize {
    let (row, col) = pos;
    let value = grid.get(col, row).unwrap();
    if value != &'0' {
        0
    } else {
        distinct_trails(grid, pos, 1)
    }
}

pub fn map_score(grid: &Grid<char>, score_fn: ScoreFn) -> usize {
    let mut scores = Vec::new();
    for row in 0..grid.height() {
        for col in 0..grid.width() {
            scores.push(score_fn(grid, (row, col)));
        }
    }
    scores.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::parse as util_parse;

    #[test]
    fn test_example_part1() {
        let grid = util_parse::<Grid<char>>("day10", "example.txt", Grid::from);
        let score = map_score(&grid, trailhead_score);
        assert_eq!(score, 36);
    }

    #[test]
    fn test_part_1() {
        let grid = util_parse::<Grid<char>>("day10", "puzzle.txt", Grid::from);
        let score = map_score(&grid, trailhead_score);
        assert_eq!(score, 430);
    }

    #[test]
    fn test_example_part2() {
        let grid = util_parse::<Grid<char>>("day10", "example.txt", Grid::from);
        let score = map_score(&grid, trailhead_rating);
        assert_eq!(score, 81);
    }

    #[test]
    fn test_part2() {
        let grid = util_parse::<Grid<char>>("day10", "puzzle.txt", Grid::from);
        let score = map_score(&grid, trailhead_rating);
        assert_eq!(score, 928);
    }
}
