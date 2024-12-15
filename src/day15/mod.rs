use crate::{grid::Grid, vec2d::Vec2d};

type Instructions = Vec<Vec2d<i32>>;

pub enum WarehouseCell {
    Wall,
    Vacant,
    Box,
    Robot,
}

pub struct Warehouse {
    grid: Grid<WarehouseCell>,
    robot_pos: Vec2d<i32>,
}

impl Warehouse {
    pub fn new(grid: Grid<WarehouseCell>, robot_pos: Vec2d<i32>) -> Self {
        Warehouse { grid, robot_pos }
    }

    pub fn step(&mut self, dir: Vec2d<i32>) {
        let new_pos = self.robot_pos + dir;
        let cell = self
            .grid
            .get(new_pos.x as usize, new_pos.y as usize)
            .unwrap();
        match cell {
            WarehouseCell::Wall => (),
            WarehouseCell::Vacant => {
                self.grid.set(
                    self.robot_pos.x as usize,
                    self.robot_pos.y as usize,
                    WarehouseCell::Vacant,
                );
                self.grid
                    .set(new_pos.x as usize, new_pos.y as usize, WarehouseCell::Robot);
                self.robot_pos = new_pos;
            }
            WarehouseCell::Box => {
                let mut new_box_pos = new_pos;
                loop {
                    if new_box_pos.x < 0
                        || new_box_pos.y < 0
                        || new_box_pos.x >= self.grid.width() as i32
                        || new_box_pos.y >= self.grid.height() as i32
                    {
                        break;
                    }

                    new_box_pos = new_box_pos + dir;
                    let new_box_cell = self
                        .grid
                        .get(new_box_pos.x as usize, new_box_pos.y as usize)
                        .unwrap();

                    match new_box_cell {
                        WarehouseCell::Wall => break,
                        WarehouseCell::Vacant => {
                            self.grid.set(
                                self.robot_pos.x as usize,
                                self.robot_pos.y as usize,
                                WarehouseCell::Vacant,
                            );
                            self.grid.set(
                                new_pos.x as usize,
                                new_pos.y as usize,
                                WarehouseCell::Robot,
                            );
                            self.grid.set(
                                new_box_pos.x as usize,
                                new_box_pos.y as usize,
                                WarehouseCell::Box,
                            );
                            self.robot_pos = new_pos;
                            break;
                        }
                        WarehouseCell::Box => (),
                        WarehouseCell::Robot => unreachable!("Robot should not be in the way"),
                    }
                }
            }
            _ => panic!("Unexpected cell in warehouse"),
        }
    }

    pub fn gps_score(&self) -> i32 {
        let mut score = 0;
        for row in 0..self.grid.height() {
            for col in 0..self.grid.width() {
                if let Some(WarehouseCell::Box) = self.grid.get(col, row) {
                    score += (100 * row as i32) + col as i32;
                }
            }
        }
        score
    }
}

impl std::fmt::Display for Grid<WarehouseCell> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in 0..self.height() {
            for col in 0..self.width() {
                let c = match self.get(col, row).unwrap() {
                    WarehouseCell::Wall => '#',
                    WarehouseCell::Vacant => '.',
                    WarehouseCell::Box => 'O',
                    WarehouseCell::Robot => '@',
                };
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn parse_input(input: String) -> (Warehouse, Instructions) {
    let (part1, part2) = {
        let parts = input.split("\n\n").collect::<Vec<&str>>();
        (parts[0], parts[1])
    };

    let warehouse_width = part1.lines().next().unwrap().len();
    let mut robot_pos = Vec2d::new(0, 0);
    let grid_data: Vec<WarehouseCell> = part1
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, c)| match c {
                    '#' => WarehouseCell::Wall,
                    '.' => WarehouseCell::Vacant,
                    'O' => WarehouseCell::Box,
                    '@' => {
                        robot_pos = Vec2d::new(j as i32, i as i32);
                        WarehouseCell::Robot
                    }
                    c => {
                        println!("Unexpected character in input: {}", c);
                        panic!("dead");
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect();

    let grid = Grid::new(grid_data, warehouse_width);
    let warehouse = Warehouse::new(grid, robot_pos);

    let instructions = part2
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| match c {
            '^' => Vec2d::new(0, -1),
            'v' => Vec2d::new(0, 1),
            '<' => Vec2d::new(-1, 0),
            '>' => Vec2d::new(1, 0),
            _ => panic!("Unexpected instruction in input"),
        })
        .collect();

    (warehouse, instructions)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::parse as util_parse;

    #[test]
    fn test_example_part1() {
        let (mut wh, moves) =
            util_parse::<(Warehouse, Instructions)>("day15", "example.txt", parse_input);
        for m in moves {
            wh.step(m);
        }
        assert_eq!(wh.gps_score(), 10092);
    }

    #[test]
    fn test_part1() {
        let (mut wh, moves) =
            util_parse::<(Warehouse, Instructions)>("day15", "puzzle.txt", parse_input);
        for m in moves {
            wh.step(m);
        }
        assert_eq!(wh.gps_score(), 1495147);
    }

    // #[test]
    // fn test_part_1() {
    //     let input = util_parse::<String>("day15", "puzzle.txt", parse_input);
    //     println!("Input: {}", input);
    // }
}
