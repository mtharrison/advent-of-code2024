fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
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

fn get_distance(mut lhs: Vec<i32>, mut rhs: Vec<i32>) -> i32 {
    let mut distance = 0;

    lhs.sort();
    rhs.sort();

    for i in 0..lhs.len() {
        let x = lhs[i];
        let y = rhs[i];
        distance += (x - y).abs();
    }

    distance
}

fn get_similarity(lhs: Vec<i32>, rhs: Vec<i32>) -> i32 {
    let mut similarity = 0;

    for num in lhs {
        // count how many times num appears in rhs
        let count: i32 = rhs
            .iter()
            .filter(|&x| *x == num)
            .count()
            .try_into()
            .unwrap();

        similarity += count * num;
    }

    similarity
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input =
            std::fs::read_to_string("bin/day1/example.txt").expect("Failed to read input file");
        let (lhs, rhs) = parse_input(&input);
        let distance = get_distance(lhs, rhs);
        assert_eq!(distance, 11);
    }

    #[test]
    fn test_part_1() {
        let input =
            std::fs::read_to_string("bin/day1/puzzle.txt").expect("Failed to read input file");
        let (lhs, rhs) = parse_input(&input);
        let distance = get_distance(lhs, rhs);
        println!("Distance: {}", distance);
    }

    #[test]
    fn test_part_2() {
        let input =
            std::fs::read_to_string("bin/day1/puzzle.txt").expect("Failed to read input file");
        let (lhs, rhs) = parse_input(&input);
        let similarity = get_similarity(lhs, rhs);
        println!("Similarity: {}", similarity);
    }
}
