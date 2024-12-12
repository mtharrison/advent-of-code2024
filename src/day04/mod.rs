use crate::grid::Grid;

pub fn is_xmas_grid(grid: &Grid<char>) -> bool {
    let lines = [vec![(0, 0), (1, 1), (2, 2)], vec![(2, 0), (1, 1), (0, 2)]];

    let strings = lines
        .iter()
        .map(|line| {
            line.iter()
                .map(|(x, y)| grid.get(*x, *y).unwrap())
                .collect::<String>()
        })
        .collect::<Vec<String>>();

    strings.iter().all(|s| s == "MAS" || s == "SAM")
}

#[cfg(test)]
mod tests {
    use crate::day04::is_xmas_grid;
    use crate::grid::Grid;
    use crate::util::parse as util_parse;

    #[test]
    fn test_example_part1() {
        let grid = util_parse::<Grid<char>>("day04", "example.txt", Grid::from);
        let mut counter = 0;
        let searches = Vec::from([grid.rows(), grid.columns(), grid.diagonals()]);
        let searches = searches.iter().flatten();

        for search in searches {
            let string = search.iter().collect::<String>();
            counter += string.matches("XMAS").count();
            counter += string.matches("SAMX").count();
        }

        assert_eq!(counter, 18);
    }

    #[test]
    fn test_part1() {
        let grid = util_parse::<Grid<char>>("day04", "puzzle.txt", Grid::from);
        let mut counter = 0;
        let searches = Vec::from([grid.rows(), grid.columns(), grid.diagonals()]);
        let searches = searches.iter().flatten();

        for search in searches {
            let string = search.iter().collect::<String>();
            counter += string.matches("XMAS").count();
            counter += string.matches("SAMX").count();
        }

        assert_eq!(counter, 2462);
    }

    #[test]
    fn test_example_part2() {
        let grid = util_parse::<Grid<char>>("day04", "example2.txt", Grid::from);
        let mut counter = 0;
        for square in grid.squares(3) {
            if is_xmas_grid(&square) {
                counter += 1;
            }
        }

        assert_eq!(counter, 9);
    }

    #[test]
    fn test_part2() {
        let grid = util_parse::<Grid<char>>("day04", "puzzle.txt", Grid::from);
        let mut counter = 0;
        for square in grid.squares(3) {
            if is_xmas_grid(&square) {
                counter += 1;
            }
        }

        assert_eq!(counter, 1877);
    }
}
