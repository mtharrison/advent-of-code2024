pub fn parse_input(input: String) -> (Vec<i32>, Vec<i32>) {
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

pub fn get_distance(mut lhs: Vec<i32>, mut rhs: Vec<i32>) -> i32 {
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

pub fn get_similarity(lhs: Vec<i32>, rhs: Vec<i32>) -> i32 {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::parse as util_parse;

    #[test]
    fn test_example_part1() {
        let (lhs, rhs) = util_parse::<(Vec<i32>, Vec<i32>)>("day01", "example.txt", parse_input);
        let distance = get_distance(lhs, rhs);
        assert_eq!(distance, 11);
    }

    #[test]
    fn test_example_part2() {
        let (lhs, rhs) = util_parse::<(Vec<i32>, Vec<i32>)>("day01", "example.txt", parse_input);
        let distance = get_similarity(lhs, rhs);
        assert_eq!(distance, 31);
    }

    #[test]
    fn test_part_1() {
        let (lhs, rhs) = util_parse::<(Vec<i32>, Vec<i32>)>("day01", "puzzle.txt", parse_input);
        let distance = get_distance(lhs, rhs);
        println!("Distance: {}", distance);
    }

    #[test]
    fn test_part_2() {
        let (lhs, rhs) = util_parse::<(Vec<i32>, Vec<i32>)>("day01", "puzzle.txt", parse_input);
        let similarity = get_similarity(lhs, rhs);
        println!("Similarity: {}", similarity);
    }
}
