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
