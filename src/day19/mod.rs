use ahash::HashSet;

pub fn num_ways(design: &str, patterns: &[&str]) -> usize {
    let n = design.len();
    let mut memo = vec![0; n + 1];
    memo[0] = 1;
    let pattern_set: HashSet<&str> = patterns.iter().cloned().collect();

    for i in 1..=n {
        for j in (0..i).rev() {
            let substring = &design[j..i];
            if pattern_set.contains(substring) {
                memo[i] += memo[j];
            }
        }
    }

    memo[n]
}

pub fn parse_input(input: String) -> (Vec<String>, Vec<String>) {
    let mut input = input.split("\n\n");
    let rules = input
        .next()
        .unwrap()
        .split(", ")
        .map(|x| x.to_string())
        .collect();

    let designs = input
        .next()
        .unwrap()
        .lines()
        .map(|x| x.to_string())
        .collect();
    (rules, designs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::parse as util_parse;

    #[test]
    fn test_example_part1() {
        let (rules, designs) =
            util_parse::<(Vec<String>, Vec<String>)>("day19", "example.txt", parse_input);

        let rules = rules.iter().map(|x| x.as_str()).collect::<Vec<&str>>();
        let result = designs.iter().filter(|x| num_ways(x, &rules) > 0).count();
        assert_eq!(result, 6);
    }

    #[test]
    fn test_part1() {
        let (rules, designs) =
            util_parse::<(Vec<String>, Vec<String>)>("day19", "puzzle.txt", parse_input);

        let rules = rules.iter().map(|x| x.as_str()).collect::<Vec<&str>>();
        let result = designs.iter().filter(|x| num_ways(x, &rules) > 0).count();
        assert_eq!(result, 242);
    }

    #[test]
    fn test_example_part2() {
        let (rules, designs) =
            util_parse::<(Vec<String>, Vec<String>)>("day19", "example.txt", parse_input);

        let rules = rules.iter().map(|x| x.as_str()).collect::<Vec<&str>>();
        let result = designs.iter().map(|x| num_ways(x, &rules)).sum::<usize>();
        assert_eq!(result, 16);
    }

    #[test]
    fn test_part2() {
        let (rules, designs) =
            util_parse::<(Vec<String>, Vec<String>)>("day19", "puzzle.txt", parse_input);

        let rules = rules.iter().map(|x| x.as_str()).collect::<Vec<&str>>();
        let result = designs.iter().map(|x| num_ways(x, &rules)).sum::<usize>();
        assert_eq!(result, 595975512785325);
    }
}
