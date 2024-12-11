// 0 -> 1
pub fn rule1(input: &i64) -> Option<i64> {
    if input == &0 {
        Some(1)
    } else {
        None
    }
}

// AB -> A B
pub fn rule2(input: &i64) -> Option<(i64, i64)> {
    let log = input.ilog10();
    if log % 2 == 0 {
        return None;
    }
    let pow = (log + 1) / 2;
    let lhs = input / i64::pow(10, pow);
    let rhs = input - lhs * i64::pow(10, pow);
    Some((lhs, rhs))
}

// n -> n * 2024
pub fn rule3(input: &i64) -> Option<i64> {
    Some(*input * 2024)
}

pub fn apply_rules(input: &Vec<i64>) -> Vec<i64> {
    let mut output = Vec::with_capacity(input.len() * 2);
    for i in input {
        if let Some(result) = rule1(i) {
            output.push(result);
        } else if let Some((lhs, rhs)) = rule2(i) {
            output.push(lhs);
            output.push(rhs);
        } else if let Some(result) = rule3(i) {
            output.push(result);
        }
    }
    output
}

pub fn blink(input: &Vec<i64>, n: usize) -> Vec<i64> {
    let mut output = input.clone();
    for i in 0..n {
        output = apply_rules(&output);
    }
    output
}

pub fn parse_input(input: String) -> Vec<i64> {
    input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::parse as util_parse;

    #[test]
    fn test_rule1() {
        assert_eq!(rule1(&0), Some(1));
        assert_eq!(rule1(&1), None);
    }

    #[test]
    fn test_rule2() {
        assert_eq!(rule2(&34), Some((3, 4)));
        assert_eq!(rule2(&1234), Some((12, 34)));
        assert_eq!(rule2(&678906), Some((678, 906)));
        assert_eq!(rule2(&8704), Some((87, 4)));
    }

    #[test]
    fn test_rule3() {
        assert_eq!(rule3(&1), Some(2024));
        assert_eq!(rule3(&2), Some(4048));
    }

    #[test]
    fn test_example_part1() {
        let input = util_parse::<Vec<i64>>("day11", "example.txt", parse_input);
        let output = blink(&input, 25);
        assert_eq!(output.len(), 55312);
    }

    #[test]
    fn test_part1() {
        let input = util_parse::<Vec<i64>>("day11", "puzzle.txt", parse_input);
        let output = blink(&input, 25);
        assert_eq!(output.len(), 233050);
    }

    #[test]
    fn test_part2() {
        let input = util_parse::<Vec<i64>>("day11", "puzzle.txt", parse_input);
        let output = blink(&input, 75);
        assert_eq!(output.len(), 233050);
    }
}
