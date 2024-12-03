use regex::Regex;

#[derive(Debug)]
pub enum Instruction {
    Mul(i32, i32),
    Do,
    Dont,
}

pub struct Machine {
    enabled: bool,
    accumulator: i32,
    evaluate_do_instructions: bool,
}

impl Machine {
    pub fn new(evaluate_do_instructions: bool) -> Self {
        Machine {
            enabled: true,
            accumulator: 0,
            evaluate_do_instructions,
        }
    }

    pub fn execute(&mut self, instr: Instruction) {
        match instr {
            Instruction::Mul(n1, n2) => {
                if self.enabled {
                    self.accumulator += n1 * n2;
                }
            }
            Instruction::Do => {
                if self.evaluate_do_instructions {
                    self.enabled = true;
                }
            }
            Instruction::Dont => {
                if self.evaluate_do_instructions {
                    self.enabled = false;
                }
            }
        }
    }

    pub fn run_program(&mut self, instrs: Vec<Instruction>) -> i32 {
        for instr in instrs {
            self.execute(instr);
        }

        self.accumulator
    }
}

pub fn scan_program(input: String) -> Vec<Instruction> {
    let re = Regex::new(r"(mul\(\d+,\d+\)|do\(\)|don't\(\))").unwrap();
    let instrs = re
        .find_iter(&input)
        .map(|c| c.as_str().into())
        .map(|s: String| {
            if s == "do()" {
                Instruction::Do
            } else if s == "don't()" {
                Instruction::Dont
            } else {
                let mut i = 4;
                let chars = s.chars().collect::<Vec<char>>();

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

                Instruction::Mul(n1, n2)
            }
        })
        .collect();

    instrs
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::parse;

    #[test]
    fn test_example_part1() {
        let instrs = parse("day3", "example.txt", scan_program);
        let result = Machine::new(false).run_program(instrs);
        assert_eq!(result, 161);
    }

    #[test]
    fn test_part1() {
        let instrs = parse("day3", "puzzle.txt", scan_program);
        let result = Machine::new(false).run_program(instrs);
        print!("Part 1: {}", result);
    }

    #[test]
    fn test_example_part2() {
        let instrs = parse("day3", "example2.txt", scan_program);
        let result = Machine::new(true).run_program(instrs);
        assert_eq!(result, 48);
    }

    #[test]
    fn test_part2() {
        let instrs = parse("day3", "puzzle.txt", scan_program);
        let result = Machine::new(true).run_program(instrs);
        print!("Part 2: {}", result);
    }
}
