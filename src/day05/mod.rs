type Edge = (i32, i32);
type Path = Vec<i32>;

pub fn parse_input(input: String) -> (Vec<Edge>, Vec<Path>) {
    let (part1, part2) = {
        let parts = input.split("\n\n").collect::<Vec<&str>>();
        (parts[0], parts[1])
    };

    let mut rules = Vec::new();

    for line in part1.lines() {
        let (l, r) = {
            let parts = line.split_once("|").unwrap();
            (
                parts.0.trim().parse::<i32>().unwrap(),
                parts.1.trim().parse::<i32>().unwrap(),
            )
        };

        rules.push((l, r));
    }

    let mut paths = vec![];
    part2.lines().for_each(|line| {
        let path = line
            .split(",")
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        paths.push(path);
    });

    (rules, paths)
}

pub fn valid_path(rules: &Vec<Edge>, path: &Path) -> bool {
    for i in 0..path.len() - 1 {
        let (l, r) = (path[i], path[i + 1]);
        match rules.iter().find(|(a, b)| (*a == l && *b == r)) {
            Some(_) => continue,
            None => return false,
        }
    }
    true
}

// We cannot construct a DAG from the full set of edges,
// however we can from the relevant subset for a path
pub fn pick_edges(rules: &Vec<Edge>, path: &Path) -> Vec<Edge> {
    let mut relevant = vec![];
    for rule in rules.iter() {
        if path.contains(&rule.0) && path.contains(&rule.1) {
            relevant.push(*rule);
        }
    }
    relevant
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::Graph;
    use crate::util::parse as util_parse;

    #[test]
    fn test_example_part1() {
        let (rules, paths) =
            util_parse::<(Vec<Edge>, Vec<Path>)>("day05", "example.txt", parse_input);

        let mut acc = 0;
        for path in paths {
            let valid = valid_path(&rules, &path);
            if valid {
                let mid = path[path.len() / 2];
                acc += mid;
            }
        }

        assert_eq!(acc, 143);
    }

    #[test]
    fn test_part1() {
        let (rules, paths) =
            util_parse::<(Vec<Edge>, Vec<Path>)>("day05", "puzzle.txt", parse_input);

        let mut acc = 0;
        for path in paths {
            let valid = valid_path(&rules, &path);
            if valid {
                let mid = path[path.len() / 2];
                acc += mid;
            }
        }

        assert_eq!(acc, 4281);
    }

    #[test]
    fn test_example_part2() {
        let (rules, paths) =
            util_parse::<(Vec<Edge>, Vec<Path>)>("day05", "example.txt", parse_input);

        let mut acc = 0;
        for path in paths {
            let valid = valid_path(&rules, &path);
            if !valid {
                let relevant = pick_edges(&rules, &path);
                let g = Graph::new(relevant);
                let new_path = g.topological_sort().unwrap();
                let mid = new_path[new_path.len() / 2];
                acc += mid;
            }
        }

        assert_eq!(acc, 123);
    }

    #[test]
    fn test_part2() {
        let (rules, paths) =
            util_parse::<(Vec<Edge>, Vec<Path>)>("day05", "puzzle.txt", parse_input);

        let mut acc = 0;
        for path in paths {
            let valid = valid_path(&rules, &path);
            if !valid {
                let relevant = pick_edges(&rules, &path);
                let g = Graph::new(relevant);
                let new_path = g.topological_sort().unwrap();
                let mid = new_path[new_path.len() / 2];
                acc += mid;
            }
        }

        assert_eq!(acc, 5466);
    }
}
