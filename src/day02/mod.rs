pub fn parse_input(input: String) -> Vec<Vec<i32>> {
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

pub fn is_safe(report: Vec<i32>, tolerance: bool) -> bool {
    for i in 0..report.len() - 1 {
        let diff = report[i + 1] - report[i];
        if diff < 1 || diff > 3 {
            if !tolerance {
                return false;
            }

            // make a list without element at index i
            let mut cloned = report.clone();
            cloned.remove(i);

            // make a list without element at index i + 1
            let mut cloned2 = report.clone();
            cloned2.remove(i + 1);

            return is_safe(cloned, false) || is_safe(cloned2, false);
        }
    }
    true
}

pub fn check_safe(report: Vec<i32>, tolerance: bool) -> bool {
    let cloned = report.clone();
    is_safe(report, tolerance) || is_safe(cloned.into_iter().rev().collect(), tolerance)
}

pub fn count_safe(reports: Vec<Vec<i32>>, tolerance: bool) -> i32 {
    let mut count = 0;

    for report in reports {
        count += match check_safe(report, tolerance) {
            true => 1,
            false => 0,
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::parse as util_parse;

    #[test]
    fn test_example_part1() {
        let reports = util_parse::<Vec<Vec<i32>>>("day02", "example.txt", parse_input);
        let count = count_safe(reports, false);
        assert_eq!(count, 2);
    }

    #[test]
    fn test_example_part2() {
        let reports = util_parse::<Vec<Vec<i32>>>("day02", "example.txt", parse_input);
        let count = count_safe(reports, true);
        assert_eq!(count, 4);
    }

    #[test]
    fn test_part1() {
        let reports = util_parse::<Vec<Vec<i32>>>("day02", "puzzle.txt", parse_input);
        let count = count_safe(reports, false);
        println!("Count: {}", count);
    }

    #[test]
    fn test_part2() {
        let reports = util_parse::<Vec<Vec<i32>>>("day02", "puzzle.txt", parse_input);
        let count = count_safe(reports, true);
        println!("Count: {}", count);
    }
}
