use crate::vec2d::Vec2d;
use itertools::Itertools;
use crate::util::parse_input_day08 as parse_input;

pub struct CellTower {
    pos: Vec2d,
    freq: char,
}

pub fn out_of_bounds(pos: &Vec2d, dims: &(usize, usize)) -> bool {
    pos.x < 0 || pos.y < 0 || pos.x >= dims.0 as i32 || pos.y >= dims.1 as i32
}

pub fn antinodes_unique(antinodes: Vec<Vec2d>) -> usize {
    antinodes.iter().unique().count()
}

pub fn find_antinodes_pt1(towers: Vec<CellTower>, dims: (usize, usize)) -> Vec<Vec2d> {
    towers
        .iter()
        .combinations(2)
        .flat_map(|x| {
            let (a, b) = (x[0], x[1]);
            if a.freq != b.freq {
                return None;
            }

            let p1 = a.pos;
            let p2 = b.pos;

            let d1 = p1 - p2;
            let d2 = p2 - p1;

            Some(vec![p1 + d1, p2 + d2])
        })
        .flatten()
        .filter(|x| !out_of_bounds(x, &dims))
        .collect::<Vec<_>>()
}

pub fn find_antinodes_pt2(towers: Vec<CellTower>, dims: (usize, usize)) -> Vec<Vec2d> {
    towers
        .iter()
        .combinations(2)
        .flat_map(|x| {
            let mut anti = Vec::new();
            let (a, b) = (x[0], x[1]);
            if a.freq != b.freq {
                return None;
            }

            let mut p1 = a.pos;
            let mut p2 = b.pos;

            let d1 = p1 - p2;
            let d2 = p2 - p1;

            while !out_of_bounds(&p1, &dims) {
                anti.push(p1);
                p1 = p1 + d1;
            }

            while !out_of_bounds(&p2, &dims) {
                anti.push(p2);
                p2 = p2 + d2;
            }

            Some(anti)
        })
        .flatten()
        .filter(|x| !out_of_bounds(x, &dims))
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::parse as util_parse;

    #[test]
    fn test_example_part1() {
        let (towers, dims) =
            util_parse::<(Vec<CellTower>, (usize, usize))>("day08", "example.txt", parse_input);
        let antinodes = find_antinodes_pt1(towers, dims);
        assert_eq!(antinodes_unique(antinodes), 14);
    }

    #[test]
    fn test_part1() {
        let (towers, dims) =
            util_parse::<(Vec<CellTower>, (usize, usize))>("day08", "puzzle.txt", parse_input);
        let antinodes = find_antinodes_pt1(towers, dims);
        assert_eq!(antinodes_unique(antinodes), 426);
    }

    #[test]
    fn test_example_part2() {
        let (towers, dims) =
            util_parse::<(Vec<CellTower>, (usize, usize))>("day08", "example2.txt", parse_input);
        let antinodes = find_antinodes_pt2(towers, dims);
        assert_eq!(antinodes_unique(antinodes), 9);

        let (towers, dims) =
            util_parse::<(Vec<CellTower>, (usize, usize))>("day08", "example.txt", parse_input);
        let antinodes = find_antinodes_pt2(towers, dims);
        assert_eq!(antinodes_unique(antinodes), 34);
    }

    #[test]
    fn test_part2() {
        let (towers, dims) =
            util_parse::<(Vec<CellTower>, (usize, usize))>("day08", "puzzle.txt", parse_input);
        let antinodes = find_antinodes_pt2(towers, dims);
        assert_eq!(antinodes_unique(antinodes), 1359);
    }
}
