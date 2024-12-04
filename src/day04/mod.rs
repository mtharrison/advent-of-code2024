use crate::grid::Grid;

pub fn is_xmas_grid(grid: &Grid<char>) -> bool {
    let diag_indices_1 = vec![(0, 0), (1, 1), (2, 2)];
    let diag_indices_2 = vec![(2, 0), (1, 1), (0, 2)];

    let diag_1 = diag_indices_1
        .iter()
        .map(|(x, y)| grid.get(*x, *y).unwrap())
        .collect::<String>();

    let diag_2 = diag_indices_2
        .iter()
        .map(|(x, y)| grid.get(*x, *y).unwrap())
        .collect::<String>();

    (diag_1 == "MAS" || diag_1 == "SAM") && (diag_2 == "MAS" || diag_2 == "SAM")
}

#[cfg(test)]
mod tests {
    use crate::day04::is_xmas_grid;
    use crate::grid::Grid;
    use crate::util::parse as util_parse;

    #[test]
    fn test_example_part1() {
        let grid = util_parse::<Grid<char>>("day04", "example.txt", |s| Grid::from(s));
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
        let grid = util_parse::<Grid<char>>("day04", "puzzle.txt", |s| Grid::from(s));
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
        let grid = util_parse::<Grid<char>>("day04", "example2.txt", |s| Grid::from(s));
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
        let grid = util_parse::<Grid<char>>("day04", "puzzle.txt", |s| Grid::from(s));
        let mut counter = 0;
        for square in grid.squares(3) {
            if is_xmas_grid(&square) {
                counter += 1;
            }
        }

        assert_eq!(counter, 1877);
    }
}
