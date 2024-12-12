use std::collections::HashSet;

use crate::grid::{Direction, Grid};

type Edge = (isize, isize, Direction); // (x, y, side)
type Region = (usize, HashSet<Edge>); // (plots, edges)

pub fn parse_input(input: String) -> Grid<char> {
    Grid::from(input)
}

pub fn find_regions(grid: &Grid<char>) -> Vec<Region> {
    let mut regions = Vec::new();
    let mut visited = HashSet::new();

    for row in 0..grid.height() {
        for col in 0..grid.width() {
            if visited.contains(&(col, row)) {
                continue;
            }

            let mut plots = 0;
            let mut edges = HashSet::new();
            let plant_type = grid.get(col, row).unwrap();

            let mut stack = vec![(col, row)];
            while let Some((col, row)) = stack.pop() {
                if visited.contains(&(col, row)) {
                    continue;
                }

                visited.insert((col, row));
                plots += 1;

                let neighbours = grid.neighbours_with_dir((col, row));
                let col = col as isize;
                let row = row as isize;
                for neighbour in neighbours {
                    let (neighbour, dir) = neighbour;
                    let mut add_edge = false;
                    if let Some((x, y)) = neighbour {
                        let neighbour_type = grid.get(x, y).unwrap();
                        if neighbour_type == plant_type {
                            if !visited.contains(&(x, y)) {
                                stack.push((x, y));
                            }
                        } else {
                            add_edge = true;
                        }
                    } else {
                        add_edge = true;
                    }

                    if add_edge {
                        // dedupe edges
                        edges.insert(match dir {
                            Direction::Up => (col, row, Direction::Up),
                            Direction::Down => (col, row + 1, Direction::Up),
                            Direction::Left => (col - 1, row, Direction::Right),
                            Direction::Right => (col, row, Direction::Right),
                        });
                    }
                }
            }

            regions.push((plots, edges));
        }
    }

    regions
}

pub fn price_regions_by_edges(regions: &[Region]) -> usize {
    regions
        .iter()
        .map(|(plots, edges)| plots * edges.len())
        .sum()
}

// Insight here is that the number of sides is equal to the number of vertices
// and the number of vertices is equal to the number of right angles,
// which in turn is equal to the number of edges which have an edge at right angles
pub fn price_regions_by_sides(regions: &[Region]) -> usize {
    regions
        .iter()
        .map(|(plots, edges)| {
            edges
                .iter()
                .map(|(col, row, dir)| match dir {
                    Direction::Up => {
                        if edges.contains(&(*col, *row - 1, Direction::Right))
                            || edges.contains(&(*col, *row, Direction::Right))
                        {
                            1
                        } else {
                            0
                        }
                    }
                    Direction::Right => {
                        if edges.contains(&(*col, *row, Direction::Up))
                            || edges.contains(&(col + 1, *row, Direction::Up))
                        {
                            1
                        } else {
                            0
                        }
                    }
                    _ => {
                        unreachable!("Only up and right edges should be present");
                    }
                })
                .sum::<usize>()
                * plots
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::parse as util_parse;

    #[test]
    fn test_example_part1() {
        let input = util_parse::<Grid<char>>("day12", "example.txt", parse_input);
        let regions = find_regions(&input);
        let price = price_regions_by_edges(&regions);
        assert_eq!(price, 1930);
    }

    #[test]
    fn test_part_1() {
        let input = util_parse::<Grid<char>>("day12", "puzzle.txt", parse_input);
        let regions = find_regions(&input);
        let price = price_regions_by_edges(&regions);
        assert_eq!(price, 1549354);
    }

    #[test]
    fn test_example_part2_2() {
        let input = util_parse::<Grid<char>>("day12", "example2.txt", parse_input);
        let regions = find_regions(&input);
        let price = price_regions_by_sides(&regions);
        assert_eq!(price, 80);
    }

    #[test]
    fn test_example_part2_3() {
        let input = util_parse::<Grid<char>>("day12", "example3.txt", parse_input);
        let regions = find_regions(&input);
        let price = price_regions_by_sides(&regions);
        assert_eq!(price, 436);
    }

    #[test]
    fn test_example_part2_4() {
        let input = util_parse::<Grid<char>>("day12", "example4.txt", parse_input);
        let regions = find_regions(&input);
        let price = price_regions_by_sides(&regions);
        assert_eq!(price, 236);
    }

    #[test]
    fn test_example_part2_5() {
        let input = util_parse::<Grid<char>>("day12", "example5.txt", parse_input);
        let regions = find_regions(&input);
        let price = price_regions_by_sides(&regions);
        assert_eq!(price, 368);
    }

    #[test]
    fn test_example_part2_6() {
        let input = util_parse::<Grid<char>>("day12", "example6.txt", parse_input);
        let regions = find_regions(&input);
        let price = price_regions_by_sides(&regions);
        assert_eq!(price, 1206);
    }

    #[test]
    fn test_part2() {
        let input = util_parse::<Grid<char>>("day12", "puzzle.txt", parse_input);
        let regions = find_regions(&input);
        let price = price_regions_by_sides(&regions);
        assert_eq!(price, 937032);
    }
}
