pub fn parse_input(input: String) -> String {
    input
}

pub fn parse_mul_instr(string: &[char]) -> (bool, String, usize) {
    let mut instr = String::new();
    let mut i = 0;
    let len = string.len();

    if i < len && string[i] == 'm' {
        instr.push(string[i]);
        i += 1;
    } else {
        return (false, instr, i);
    }

    if i < len && string[i] == 'u' {
        instr.push(string[i]);
        i += 1;
    } else {
        return (false, instr, i);
    }

    if i < len && string[i] == 'l' {
        instr.push(string[i]);
        i += 1;
    } else {
        return (false, instr, i);
    }

    if i < len && string[i] == '(' {
        instr.push(string[i]);
        i += 1;
    } else {
        return (false, instr, i);
    }

    if i < len && string[i].is_numeric() {
        while i < len && string[i].is_numeric() {
            instr.push(string[i]);
            i += 1;
        }
    } else {
        return (false, instr, i);
    }

    if i < len && string[i] == ',' {
        instr.push(string[i]);
        i += 1;
    } else {
        return (false, instr, i);
    }

    if i < len && string[i].is_numeric() {
        while i < len && string[i].is_numeric() {
            instr.push(string[i]);
            i += 1;
        }
    } else {
        return (false, instr, i);
    }

    if i < len && string[i] == ')' {
        instr.push(string[i]);
        i += 1;
    } else {
        return (false, instr, i);
    }

    (true, instr, i)
}

pub fn parse_do_dont_instr(string: &[char]) -> (bool, String, usize) {
    let mut instr = String::new();
    let mut i = 0;
    let len = string.len();

    if i < len && string[i] == 'd' {
        instr.push(string[i]);
        i += 1;
    } else {
        return (false, instr, i);
    }

    if i < len && string[i] == 'o' {
        instr.push(string[i]);
        i += 1;
    } else {
        return (false, instr, i);
    }

    if i < len && string[i] == '(' {
        instr.push(string[i]);
        i += 1;

        if i < len && string[i] == ')' {
            instr.push(string[i]);
            i += 1;
        } else {
            return (false, instr, i);
        }

        return (true, "do".into(), i);
    } else if i < len && string[i] == 'n' {
        instr.push(string[i]);
        i += 1;

        if i < len && string[i] == '\'' {
            instr.push(string[i]);
            i += 1;
        } else {
            return (false, instr, i);
        }

        if i < len && string[i] == 't' {
            instr.push(string[i]);
            i += 1;
        } else {
            return (false, instr, i);
        }

        if i < len && string[i] == '(' {
            instr.push(string[i]);
            i += 1;
        } else {
            return (false, instr, i);
        }

        if i < len && string[i] == ')' {
            instr.push(string[i]);
            i += 1;
        } else {
            return (false, instr, i);
        }

        return (true, "dont".into(), i);
    }

    return (false, instr, i);
}

pub fn eval_mul_instr(instr: String) -> i32 {
    let mut i = 4;
    let chars = instr.chars().collect::<Vec<char>>();

    let mut num1 = String::new();
    while chars[i] != ',' {
        num1.push(chars[i]);
        i += 1;
    }

    i += 1;

    let mut num2 = String::new();
    while chars[i] != ')' {
        num2.push(chars[i]);
        i += 1;
    }

    let n1 = num1.parse::<i32>().unwrap();
    let n2 = num2.parse::<i32>().unwrap();

    let result = n1 * n2;

    result
}

pub fn scan(input: String) -> Vec<String> {
    let mut instrs = Vec::<String>::new();
    let mut i = 0;

    let chars = input.chars().collect::<Vec<char>>();

    while i < chars.len() {
        if chars[i] == 'm' {
            let (parsed, instr, j) = parse_mul_instr(&chars[i..]);
            if parsed {
                instrs.push(instr);
            }
            i += j;
        } else if chars[i] == 'd' {
            let (parsed, instr, j) = parse_do_dont_instr(&chars[i..]);
            if parsed {
                instrs.push(instr);
            }
            i += j;
        } else {
            i += 1;
        }
    }

    instrs
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::parse as util_parse;

    #[test]
    fn test_example_part1() {
        let input = util_parse::<String>("day3", "example.txt", parse_input);
        let instrs = scan(input);
        let mut sum = 0;
        for instr in instrs {
            sum += eval_mul_instr(instr);
        }
        assert_eq!(sum, 161);
    }

    #[test]
    fn test_part1() {
        let input = util_parse::<String>("day3", "puzzle.txt", parse_input);
        let instrs = scan(input);
        let mut sum = 0;
        for instr in instrs {
            if instr == "dont" || instr == "do" {
                continue;
            }
            sum += eval_mul_instr(instr);
        }
        println!("Part 1: {}", sum);
    }

    #[test]
    fn test_example_part2() {
        let input = util_parse::<String>("day3", "example2.txt", parse_input);
        let instrs = scan(input);
        let mut enabled = true;
        let mut sum = 0;
        for instr in instrs {
            if instr == "dont" {
                enabled = false;
            } else if instr == "do" {
                enabled = true;
            } else if enabled {
                sum += eval_mul_instr(instr.clone());
            }
        }

        assert_eq!(sum, 48);
    }

    #[test]
    fn test_part2() {
        let input = util_parse::<String>("day3", "puzzle.txt", parse_input);
        let instrs = scan(input);
        let mut enabled = true;
        let mut sum = 0;
        for instr in instrs {
            if instr == "dont" {
                enabled = false;
            } else if instr == "do" {
                enabled = true;
            } else if enabled {
                sum += eval_mul_instr(instr.clone());
            }
        }

        println!("Part 2: {}", sum);
    }
}
