fn parse_input(input: String) -> Vec<Vec<i32>> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        let input =
            std::fs::read_to_string("bin/day2/example.txt").expect("Failed to read input file");
    }

    #[test]
    fn test_example_part2() {
        let input =
            std::fs::read_to_string("bin/day2/example.txt").expect("Failed to read input file");
    }

    #[test]
    fn test_part1() {
        let input =
            std::fs::read_to_string("bin/day2/puzzle.txt").expect("Failed to read input file");
    }

    #[test]
    fn test_part2() {
        let input =
            std::fs::read_to_string("bin/day2/puzzle.txt").expect("Failed to read input file");
    }
}
