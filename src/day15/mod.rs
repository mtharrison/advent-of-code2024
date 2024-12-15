use crate::{grid::Grid, vec2d::Vec2d};

type Instructions = Vec<Vec2d<i32>>;

#[derive(Debug, Clone)]
pub enum Box {
    Single,
    LeftHalf,
    RightHalf,
}

#[derive(Debug, Clone)]
pub enum WarehouseCell {
    Wall,
    Vacant,
    Box(Box),
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

    pub fn upscale(&mut self) {
        let mut new_cells = Vec::new();
        for row in 0..self.grid.height() {
            for col in 0..self.grid.width() {
                let cell = self.grid.get(col, row).unwrap();
                match cell {
                    WarehouseCell::Wall => {
                        new_cells.push(WarehouseCell::Wall);
                        new_cells.push(WarehouseCell::Wall);
                    }
                    WarehouseCell::Vacant => {
                        new_cells.push(WarehouseCell::Vacant);
                        new_cells.push(WarehouseCell::Vacant);
                    }
                    WarehouseCell::Box(Box::Single) => {
                        new_cells.push(WarehouseCell::Box(Box::LeftHalf));
                        new_cells.push(WarehouseCell::Box(Box::RightHalf));
                    }
                    WarehouseCell::Robot => {
                        new_cells.push(WarehouseCell::Robot);
                        new_cells.push(WarehouseCell::Vacant);
                    }
                    _ => panic!("Unexpected cell in warehouse"),
                }
            }
        }

        let new_grid = Grid::new(new_cells, self.grid.width() * 2);
        self.grid = new_grid;
        self.robot_pos = Vec2d::new(self.robot_pos.x * 2, self.robot_pos.y);
    }

    pub fn get_pushable_boxes(
        &mut self,
        pos: Vec2d<i32>,
        dir: Vec2d<i32>,
    ) -> Option<Vec<Vec2d<i32>>> {
        // first step is to gather all boxes in the direction of the push recursively
        let mut boxes = Vec::new();
        let mut stack = Vec::new();
        stack.push(pos);

        while let Some(p) = stack.pop() {
            if boxes.contains(&p) {
                continue;
            }
            boxes.push(p);
            let p_l = p + dir;
            let p_r = Vec2d::new(p.x + 1, p.y) + dir;

            let next_l = self.grid.get(p_l.x as usize, p_l.y as usize).unwrap();
            let next_r = self.grid.get(p_r.x as usize, p_r.y as usize).unwrap();

            match next_l {
                WarehouseCell::Box(Box::LeftHalf) => {
                    stack.push(p_l);
                }
                WarehouseCell::Box(Box::RightHalf) => {
                    stack.push(Vec2d::new(p_l.x - 1, p_l.y));
                }
                WarehouseCell::Wall => {
                    return None;
                }
                _ => (),
            }

            match next_r {
                WarehouseCell::Box(Box::RightHalf) => {
                    stack.push(Vec2d::new(p_r.x - 1, p_r.y));
                }
                WarehouseCell::Box(Box::LeftHalf) => {
                    stack.push(p_r);
                }
                WarehouseCell::Wall => {
                    return None;
                }
                _ => (),
            }
        }

        Some(boxes)
    }

    pub fn move_pushable_boxes_up_down(&mut self, pos: Vec2d<i32>, dir: Vec2d<i32>) {
        let boxes = self.get_pushable_boxes(pos, dir);
        if let None = boxes {
            return;
        }

        let boxes = boxes.unwrap();
        for b in boxes.iter().rev() {
            let b_l = b;
            let b_r = Vec2d::new(b.x + 1, b.y);

            // set old box positions to vacant
            self.grid
                .set(b_l.x as usize, b_l.y as usize, WarehouseCell::Vacant);
            self.grid
                .set(b_r.x as usize, b_r.y as usize, WarehouseCell::Vacant);
        }

        for b in boxes.iter().rev() {
            // set new box positions
            let new_b_l = *b + dir;
            let new_b_r = Vec2d::new(b.x + 1, b.y) + dir;
            self.grid.set(
                new_b_l.x as usize,
                new_b_l.y as usize,
                WarehouseCell::Box(Box::LeftHalf),
            );
            self.grid.set(
                new_b_r.x as usize,
                new_b_r.y as usize,
                WarehouseCell::Box(Box::RightHalf),
            );
        }

        // move robot
        self.grid.set(
            self.robot_pos.x as usize,
            self.robot_pos.y as usize,
            WarehouseCell::Vacant,
        );

        let new_robot_pos = self.robot_pos + dir;
        self.grid.set(
            new_robot_pos.x as usize,
            new_robot_pos.y as usize,
            WarehouseCell::Robot,
        );

        self.robot_pos = new_robot_pos;
    }

    pub fn move_pushable_boxes_left_right(&mut self, pos: Vec2d<i32>, dir: Vec2d<i32>) {
        let mut new_pos = pos;
        loop {
            let cell = self
                .grid
                .get(new_pos.x as usize, new_pos.y as usize)
                .unwrap();

            match cell {
                WarehouseCell::Wall => break,
                WarehouseCell::Vacant => {
                    while new_pos != pos {
                        self.grid.set(
                            new_pos.x as usize,
                            new_pos.y as usize,
                            self.grid
                                .get(new_pos.x as usize - dir.x as usize, new_pos.y as usize)
                                .unwrap()
                                .clone(),
                        );
                        new_pos = new_pos - dir;
                    }

                    self.grid
                        .set(pos.x as usize, pos.y as usize, WarehouseCell::Robot);
                    self.grid.set(
                        self.robot_pos.x as usize,
                        self.robot_pos.y as usize,
                        WarehouseCell::Vacant,
                    );
                    self.robot_pos = pos;
                    break;
                }
                WarehouseCell::Box(Box::LeftHalf) => {}
                WarehouseCell::Box(Box::RightHalf) => {}
                _ => (),
            }

            new_pos = new_pos + dir;
        }
    }

    pub fn move_pushable_boxes(&mut self, pos: Vec2d<i32>, dir: Vec2d<i32>) {
        if dir.y != 0 {
            self.move_pushable_boxes_up_down(pos, dir);
        } else {
            self.move_pushable_boxes_left_right(pos, dir);
        }
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
            WarehouseCell::Box(Box::LeftHalf) => {
                self.move_pushable_boxes(new_pos, dir);
            }
            WarehouseCell::Box(Box::RightHalf) => {
                if dir.y != 0 {
                    self.move_pushable_boxes(
                        Vec2d {
                            x: new_pos.x - 1,
                            y: new_pos.y,
                        },
                        dir,
                    );
                } else {
                    self.move_pushable_boxes(new_pos, dir);
                }
            }
            WarehouseCell::Box(Box::Single) => {
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
                                WarehouseCell::Box(Box::Single),
                            );
                            self.robot_pos = new_pos;
                            break;
                        }
                        WarehouseCell::Box(_) => (),
                        WarehouseCell::Robot => unreachable!("Robot should not be in the way"),
                    }
                }
            }
            c => panic!("Unexpected cell in warehouse: {:?}", c),
        }
    }

    pub fn gps_score(&self) -> i32 {
        let mut score = 0;
        for row in 0..self.grid.height() {
            for col in 0..self.grid.width() {
                match self.grid.get(col, row) {
                    Some(WarehouseCell::Box(Box::Single))
                    | Some(WarehouseCell::Box(Box::LeftHalf)) => {
                        score += (100 * row as i32) + col as i32;
                    }
                    _ => (),
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
                    WarehouseCell::Box(Box::Single) => 'O',
                    WarehouseCell::Box(Box::LeftHalf) => '[',
                    WarehouseCell::Box(Box::RightHalf) => ']',
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
                    'O' => WarehouseCell::Box(Box::Single),
                    '[' => WarehouseCell::Box(Box::LeftHalf),
                    ']' => WarehouseCell::Box(Box::RightHalf),
                    '@' => {
                        robot_pos = Vec2d::new(j as i32, i as i32);
                        WarehouseCell::Robot
                    }
                    c => {
                        println!("Unexpected character in input: {}", c);
                        panic!();
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

    #[test]
    fn test_example_part2() {
        let (mut wh, moves) =
            util_parse::<(Warehouse, Instructions)>("day15", "example.txt", parse_input);
        wh.upscale();
        for m in moves {
            wh.step(m);
        }
        assert_eq!(wh.gps_score(), 9021);
    }

    #[test]
    fn test_part2() {
        let (mut wh, moves) =
            util_parse::<(Warehouse, Instructions)>("day15", "puzzle.txt", parse_input);
        wh.upscale();
        for m in moves {
            wh.step(m);
        }
        assert_eq!(wh.gps_score(), 1524905);
    }
}
