pub fn load_file(day: &str, filename: &str) -> String {
    let path = format!("src/{}/inputs/{}", day, filename);
    
    std::fs::read_to_string(path).expect("Failed to read input file")
}

pub fn parse<T>(day: &str, filename: &str, parse: impl Fn(String) -> T) -> T {
    let contents = load_file(day, filename);
    parse(contents)
}

pub fn parse_input<T: From<String>>(day: &str, filename: &str) -> T {
    let contents = load_file(day, filename);
    contents.into()
}

pub fn parse_input_day01(input: String) -> (Vec<i32>, Vec<i32>) {
    let mut lhs = Vec::new();
    let mut rhs = Vec::new();

    for line in input.lines() {
        let parts: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        lhs.push(parts[0]);
        rhs.push(parts[1]);
    }

    (lhs, rhs)
}

pub fn parse_input_day02(input: String) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    for line in input.lines() {
        let parts: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        result.push(parts);
    }
    result
}

pub fn parse_input_day05(input: String) -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    let (part1, part2) = {
        let parts = input.split("\n\n").collect::<Vec<&str>>();
        (parts[0], parts[1])
    };

    let mut rules = Vec::new();

    for line in part1.lines() {
        let (l, r) = {
            let parts = line.split_once("|").unwrap();
            (
                parts.0.trim().parse::<i32>().unwrap(),
                parts.1.trim().parse::<i32>().unwrap(),
            )
        };

        rules.push((l, r));
    }

    let mut paths = vec![];
    part2.lines().for_each(|line| {
        let path = line
            .split(",")
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        paths.push(path);
    });

    (rules, paths)
}

pub fn parse_input_day07(input: String) -> Vec<(i64, Vec<i64>)> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(":").map(|x| x.trim());
            let result = parts.next().unwrap().parse().unwrap();
            let operands: Vec<i64> = parts
                .next()
                .unwrap()
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();

            (result, operands)
        })
        .collect()
}

pub fn parse_input_day08(input: String) -> (Vec<crate::day08::CellTower>, (usize, usize)) {
    let mut width = 0;
    let mut height = 0;

    let towers = input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            height = i;
            line.chars()
                .enumerate()
                .filter_map(|(j, c)| {
                    width = j;
                    match c {
                        '.' | '#' => None,
                        _ => Some(crate::day08::CellTower {
                            pos: crate::vec2d::Vec2d::new(j as i32, i as i32),
                            freq: c,
                        }),
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect();

    (towers, (width + 1, height + 1))
}

#[derive(Debug, PartialEq)]
pub struct Point {
    x: i32,
    y: i32,
}

impl From<String> for Point {
    fn from(s: String) -> Self {
        let parts: Vec<i32> = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
        Point {
            x: parts[0],
            y: parts[1],
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_load_file() {
        let contents = load_file("util", "example.txt");
        assert_eq!(contents, "3   4\n4   3\n2   5\n1   3\n3   9\n3   3\n");
    }

    #[test]
    fn test_parse_file() {
        let (lhs, rhs) = parse::<(Vec<i32>, Vec<i32>)>("util", "example.txt", |input| {
            let mut result = Vec::new();
            for line in input.lines() {
                let parts: Vec<i32> = line
                    .split_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect();
                result.push(parts);
            }
            (result[0].clone(), result[1].clone())
        });
        assert_eq!(lhs, vec![3, 4]);
        assert_eq!(rhs, vec![4, 3]);
    }

    #[test]
    fn test_parse_input() {
        let point = parse_input::<Point>("util", "point.txt");
        assert_eq!(point, Point { x: 1, y: 2 });
    }
}
