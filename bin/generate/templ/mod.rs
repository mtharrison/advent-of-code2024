pub fn parse_input(input: String) -> String {
    input
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::parse as util_parse;

    #[test]
    fn test_example_part1() {
        let input = util_parse::<String>("day{day}", "example.txt", parse_input);
        println!("Input: {}", input);
    }

    #[test]
    fn test_part_1() {
        let input = util_parse::<String>("day{day}", "puzzle.txt", parse_input);
        println!("Input: {}", input);
    }
}
