pub fn load_file(day: &str, filename: &str) -> String {
    let path = format!("src/{}/{}", day, filename);
    let contents = std::fs::read_to_string(path).expect("Failed to read input file");
    contents
}

pub fn parse<T>(day: &str, filename: &str, parse: impl Fn(String) -> T) -> T {
    let contents = load_file(day, filename);
    parse(contents)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_load_file() {
        let contents = load_file("day1", "example.txt");
        assert_eq!(contents, "3   4\n4   3\n2   5\n1   3\n3   9\n3   3\n");
    }

    #[test]
    fn test_parse_file() {
        let (lhs, rhs) = parse::<(Vec<i32>, Vec<i32>)>("day1", "example.txt", |input| {
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
}
