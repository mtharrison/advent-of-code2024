use std::collections::HashMap;

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

pub fn apply_rules(input: &i64) -> Vec<i64> {
    if let Some(result) = rule1(input) {
        vec![result]
    } else if let Some((lhs, rhs)) = rule2(input) {
        vec![lhs, rhs]
    } else if let Some(result) = rule3(input) {
        vec![result]
    } else {
        unreachable!("No rule applied");
    }
}

pub fn blink_recursive_count(
    mut input: Vec<i64>,
    n: usize,
    hashmap: &mut HashMap<(i64, usize), usize>,
) -> usize {
    if n == 0 || input.is_empty() {
        return input.len();
    }

    let rest = input.split_off(1);

    let lhs = if hashmap.contains_key(&(input[0], n)) {
        *hashmap.get(&(input[0], n)).unwrap()
    } else {
        let lhs = blink_recursive_count(apply_rules(&input[0]), n - 1, hashmap);
        hashmap.insert((input[0], n), lhs);
        lhs
    };

    lhs + blink_recursive_count(rest, n, hashmap)
}

pub fn blink_count(input: Vec<i64>, n: usize) -> usize {
    let mut hashmap: HashMap<(i64, usize), usize> = HashMap::new();
    blink_recursive_count(input, n, &mut hashmap)
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
    fn test_example_part1() {
        let input = util_parse::<Vec<i64>>("day11", "example.txt", parse_input);
        let count = blink_count(input, 25);
        assert_eq!(count, 55312);
    }

    #[test]
    fn test_part1() {
        let input = util_parse::<Vec<i64>>("day11", "puzzle.txt", parse_input);
        let count = blink_count(input, 25);
        assert_eq!(count, 233050);
    }

    #[test]
    fn test_part2() {
        let input = util_parse::<Vec<i64>>("day11", "puzzle.txt", parse_input);
        let count = blink_count(input, 75);
        assert_eq!(count, 276661131175807);
    }
}
