#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

pub struct Grid<T> {
    data: Vec<T>,
    width: usize,
}

impl<T> Grid<T> {
    pub fn new(data: Vec<T>, width: usize) -> Self {
        Self { data, width }
    }

    pub fn height(&self) -> usize {
        self.data.len() / self.width
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn get(&self, col: usize, row: usize) -> Option<&T> {
        if col >= self.width || row >= self.height() {
            None
        } else {
            Some(&self.data[row * self.width + col])
        }
    }

    pub fn rows(&self) -> Vec<Vec<T>>
    where
        T: Clone,
    {
        let rows = self.data.chunks(self.width);
        rows.map(|row| row.to_vec()).collect()
    }

    pub fn columns(&self) -> Vec<Vec<T>>
    where
        T: Clone,
    {
        (0..self.width)
            .map(|col| {
                (0..self.height())
                    .map(|row| self.get(col, row).unwrap().clone())
                    .collect()
            })
            .collect()
    }

    pub fn diagonals(&self) -> Vec<Vec<T>>
    where
        T: Clone,
    {
        let mut diagonals = vec![];
        // LR Diagonals from top right to bottom left
        // a 3 x 3 grid has 5 LR diagonals

        let mut x = self.width - 1;
        let mut y = 0;

        while y < self.height() {
            let mut diagonal = vec![];
            let mut x1 = x;
            let mut y1 = y;
            while x1 < self.width && y1 < self.height() {
                diagonal.push(self.get(x1, y1).unwrap().clone());
                x1 += 1;
                y1 += 1;
            }
            diagonals.push(diagonal);
            if x > 0 {
                x -= 1;
            } else {
                y += 1;
            }
        }

        // RL Diagonals from top left to bottom right
        // a 3 x 3 grid has 5 RL diagonals

        let mut x = 0;
        let mut y: isize = 0;

        while x < self.width {
            let mut diagonal = vec![];
            let mut x1 = x;
            let mut y1 = y;
            while x1 < self.width && y1 >= 0 {
                diagonal.push(self.get(x1, y1 as usize).unwrap().clone());
                x1 += 1;
                y1 -= 1;
            }
            diagonals.push(diagonal);
            if (y as usize) < self.height() - 1 {
                y += 1;
            } else {
                x += 1;
            }
        }

        diagonals
    }

    pub fn squares(&self, size: usize) -> Vec<Grid<T>>
    where
        T: Clone,
    {
        let mut grids = vec![];
        for y in 0..self.height() - size + 1 {
            for x in 0..self.width - size + 1 {
                let mut data = vec![];
                for y1 in y..y + size {
                    for x1 in x..x + size {
                        data.push(self.get(x1, y1).unwrap().clone());
                    }
                }
                grids.push(Grid::new(data, size));
            }
        }
        grids
    }

    pub fn neighbours(&self, pos: (usize, usize)) -> Vec<(usize, usize)> {
        let mut neighbours = Vec::new();
        let (col, row) = pos;
        let height = self.height();
        let width = self.width();

        if row > 0 {
            // Up
            neighbours.push((col, row - 1));
        }

        if col < width - 1 {
            // Right
            neighbours.push((col + 1, row));
        }

        if row < height - 1 {
            // Down
            neighbours.push((col, row + 1));
        }

        if col > 0 {
            // Left
            neighbours.push((col - 1, row));
        }

        neighbours
    }

    pub fn neighbours_with_dir(
        &self,
        pos: (usize, usize),
    ) -> Vec<(Option<(usize, usize)>, Direction)> {
        let mut neighbours = Vec::new();
        let (col, row) = pos;
        let height = self.height();
        let width = self.width();

        if row > 0 {
            // Up
            neighbours.push((Some((col, row - 1)), Direction::Up));
        } else {
            neighbours.push((None, Direction::Up));
        }

        if col < width - 1 {
            // Right
            neighbours.push((Some((col + 1, row)), Direction::Right));
        } else {
            neighbours.push((None, Direction::Right));
        }

        if row < height - 1 {
            // Down
            neighbours.push((Some((col, row + 1)), Direction::Down));
        } else {
            neighbours.push((None, Direction::Down));
        }

        if col > 0 {
            // Left
            neighbours.push((Some((col - 1, row)), Direction::Left));
        } else {
            neighbours.push((None, Direction::Left));
        }

        neighbours
    }
}

impl std::fmt::Display for Grid<char> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in 0..self.height() {
            for col in 0..self.width {
                write!(f, "{}", self.get(col, row).unwrap())?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Grid<char> {
    pub fn from(s: String) -> Self {
        let data: Vec<char> = s
            .trim()
            .lines()
            .flat_map(|line| line.trim().chars())
            .collect();
        let width = s.trim().lines().next().unwrap().trim().len();
        Self { data, width }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid() {
        let grid = Grid::new(vec![1, 2, 3, 4, 5, 6], 3);
        assert_eq!(grid.get(0, 0), Some(&1));
        assert_eq!(grid.get(1, 0), Some(&2));
        assert_eq!(grid.get(2, 0), Some(&3));
        assert_eq!(grid.get(0, 1), Some(&4));
        assert_eq!(grid.get(1, 1), Some(&5));
        assert_eq!(grid.get(2, 1), Some(&6));
        assert_eq!(grid.get(3, 0), None);
        assert_eq!(grid.get(0, 2), None);
    }

    #[test]
    fn test_height() {
        let grid = Grid::new(vec![1, 2, 3, 4, 5, 6], 3);
        assert_eq!(grid.height(), 2);
    }

    #[test]
    fn test_len() {
        let grid = Grid::new(vec![1, 2, 3, 4, 5, 6], 3);
        assert_eq!(grid.len(), 6);
    }

    #[test]
    fn test_from_string() {
        let grid = Grid::from("123\n456".to_string());
        assert_eq!(grid.get(0, 0), Some(&'1'));
        assert_eq!(grid.get(1, 0), Some(&'2'));
        assert_eq!(grid.get(2, 0), Some(&'3'));
        assert_eq!(grid.get(0, 1), Some(&'4'));
        assert_eq!(grid.get(1, 1), Some(&'5'));
        assert_eq!(grid.get(2, 1), Some(&'6'));
        assert_eq!(grid.get(3, 0), None);
    }

    #[test]
    fn test_rows() {
        let grid = Grid::new(vec![1, 2, 3, 4, 5, 6], 3);
        let rows: Vec<_> = grid.rows();
        assert_eq!(rows, vec![vec![1, 2, 3], vec![4, 5, 6]]);
    }

    #[test]
    fn test_columns() {
        let grid = Grid::new(vec![1, 2, 3, 4, 5, 6], 3);
        let columns: Vec<_> = grid.columns();
        assert_eq!(columns, vec![vec![1, 4], vec![2, 5], vec![3, 6]]);
    }

    #[test]
    fn test_diagonals() {
        let grid: Grid<char> = Grid::from(
            r"
          123
          456
          789
        "
            .to_string(),
        );

        assert_eq!(grid.data, vec!['1', '2', '3', '4', '5', '6', '7', '8', '9']);
        assert_eq!(grid.width, 3);

        let diagonals: Vec<_> = grid.diagonals();
        assert_eq!(
            diagonals,
            vec![
                vec!['3'],
                vec!['2', '6'],
                vec!['1', '5', '9'],
                vec!['4', '8'],
                vec!['7'],
                vec!['1'],
                vec!['4', '2'],
                vec!['7', '5', '3'],
                vec!['8', '6'],
                vec!['9']
            ]
        );
    }

    #[test]
    fn test_squares() {
        let grid: Grid<char> = Grid::from(
            r"
          123
          456
          789
        "
            .to_string(),
        );
        let grids: Vec<Grid<char>> = grid.squares(2);
        assert_eq!(grids.len(), 4);
        assert_eq!(grids[0].data, vec!['1', '2', '4', '5']);
        assert_eq!(grids[1].data, vec!['2', '3', '5', '6']);
        assert_eq!(grids[2].data, vec!['4', '5', '7', '8']);
        assert_eq!(grids[3].data, vec!['5', '6', '8', '9']);
    }

    #[test]
    fn test_neighbours() {
        let grid: Grid<char> = Grid::from(
            r"
          123
          456
          789
        "
            .to_string(),
        );
        let neighbours: Vec<(usize, usize)> = grid.neighbours((0, 0));
        let neighbour_vals = neighbours
            .iter()
            .map(|(col, row)| ((*col, *row), *grid.get(*col, *row).unwrap()))
            .collect::<Vec<_>>();

        assert_eq!(neighbour_vals, vec![((1, 0), '2'), ((0, 1), '4')]);
    }

    #[test]
    fn test_neighbours_with_dir() {
        let grid: Grid<char> = Grid::from(
            r"
          123
          456
          789
        "
            .to_string(),
        );
        let neighbours: Vec<(Option<(usize, usize)>, Direction)> = grid.neighbours_with_dir((0, 0));
        let neighbour_vals = neighbours
            .iter()
            .map(|(pos, dir)| (pos, dir))
            .collect::<Vec<_>>();

        assert_eq!(
            neighbour_vals,
            vec![
                (&None, &Direction::Up),
                (&Some((1, 0)), &Direction::Right),
                (&Some((0, 1)), &Direction::Down),
                (&None, &Direction::Left),
            ]
        );
    }
}
