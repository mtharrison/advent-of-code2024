use regex::Regex;

const BUTTON_A_COST: i64 = 3;
const BUTTON_B_COST: i64 = 1;

#[derive(Debug)]
pub struct ClawMachine {
    ax: i64,
    ay: i64,
    bx: i64,
    by: i64,
    x: i64,
    y: i64,
}

impl From<String> for ClawMachine {
    fn from(input: String) -> Self {
        let re = Regex::new(r"X[\+\=](?<x>\d+), Y[\+\=](?<y>\d+)").unwrap();
        let mut lines = input.lines();

        let mut parse = || -> (i64, i64) {
            let caps = re.captures(lines.next().unwrap()).unwrap();
            let x = caps["x"].parse::<i64>().unwrap();
            let y = caps["y"].parse::<i64>().unwrap();
            (x, y)
        };

        let (ax, ay) = parse();
        let (bx, by) = parse();
        let (x, y) = parse();

        ClawMachine {
            ax,
            ay,
            bx,
            by,
            x,
            y,
        }
    }
}

impl ClawMachine {
    // solve set of linear equations by equating coefficients
    pub fn solve(&self, offset: i64) -> Option<(i64, i64)> {
        let bn = (self.y + offset) * self.ax - (self.x + offset) * self.ay;
        let bd = self.ax * self.by - self.bx * self.ay;
        let an = (self.x + offset) - self.bx * bn / bd;
        let ad = self.ax;
        if ad == 0 || bd == 0 || an % ad != 0 || bn % bd != 0 {
            return None;
        }
        Some((an / ad, bn / bd))
    }

    pub fn cost(&self, offset: i64) -> i64 {
        match self.solve(offset) {
            Some((a, b)) => a * BUTTON_A_COST + b * BUTTON_B_COST,
            None => 0,
        }
    }
}

pub fn parse_input(input: String) -> Vec<ClawMachine> {
    input
        .split("\n\n")
        .map(|x| ClawMachine::from(x.to_string()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::parse as util_parse;

    #[test]
    fn test_example_part1() {
        let machines = util_parse::<Vec<ClawMachine>>("day13", "example.txt", parse_input);
        let cost = machines.iter().map(|x| x.cost(0)).sum::<i64>();
        assert_eq!(cost, 480);
    }

    #[test]
    fn test_part1() {
        let machines = util_parse::<Vec<ClawMachine>>("day13", "puzzle.txt", parse_input);
        let cost = machines.iter().map(|x| x.cost(0)).sum::<i64>();
        assert_eq!(cost, 32041);
    }

    #[test]
    fn test_part2() {
        let machines = util_parse::<Vec<ClawMachine>>("day13", "puzzle.txt", parse_input);
        let cost = machines.iter().map(|x| x.cost(10000000000000)).sum::<i64>();
        assert_eq!(cost, 95843948914827);
    }
}
