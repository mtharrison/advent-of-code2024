use itertools::Itertools;
use rayon::prelude::*;

type Equation = (i64, Vec<i64>);

pub fn parse_input(input: String) -> Vec<Equation> {
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

pub fn sum_valid(operations: Vec<char>, input: &Vec<Equation>) -> i64 {
    let total_sum = input
        .par_iter()
        .map(|(result, operands)| {
            for operators in vec![operations.iter(); operands.len() - 1]
                .into_iter()
                .multi_cartesian_product()
            {
                let mut acc = 0;
                for (i, &operand) in operands.iter().enumerate() {
                    if i == 0 {
                        acc = operand;
                    } else {
                        let operator = operators[i - 1];
                        match operator {
                            '+' => acc += operand,
                            '*' => acc *= operand,
                            '|' => acc = acc * i64::pow(10, operand.ilog10() + 1) + operand,
                            _ => panic!("Invalid operator"),
                        }
                    }
                }
                if acc == *result {
                    // println!("Found: {:?} = {:?} {:?}", acc, operators, operands);
                    return *result;
                }
            }

            0
        })
        .sum();

    total_sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::parse as util_parse;

    #[test]
    fn test_example_part1() {
        let input = util_parse::<Vec<Equation>>("day07", "example.txt", parse_input);
        let operations = vec!['+', '*'];
        assert_eq!(sum_valid(operations, &input), 3749);
    }

    #[test]
    fn test_part1() {
        let input = util_parse::<Vec<Equation>>("day07", "puzzle.txt", parse_input);
        let operations = vec!['+', '*'];
        assert_eq!(sum_valid(operations, &input), 5837374519342);
    }

    #[test]
    fn test_example_part2() {
        let input = util_parse::<Vec<Equation>>("day07", "example.txt", parse_input);
        let operations = vec!['+', '*', '|'];
        assert_eq!(sum_valid(operations, &input), 11387);
    }

    #[test]
    fn test_part2() {
        let input = util_parse::<Vec<Equation>>("day07", "puzzle.txt", parse_input);
        let operations = vec!['+', '*', '|'];
        assert_eq!(sum_valid(operations, &input), 492383931650959);
    }
}
