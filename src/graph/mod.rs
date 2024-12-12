use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug)]
pub struct Graph {
    adjacency_list: HashMap<i32, Vec<i32>>,
}

impl Graph {
    pub fn new(edges: Vec<(i32, i32)>) -> Self {
        let mut adjacency_list: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut all_nodes = HashSet::new();
        for (from, to) in &edges {
            all_nodes.insert(*from);
            all_nodes.insert(*to);
        }

        for node in &all_nodes {
            adjacency_list.entry(*node).or_default();
        }

        for (from, to) in edges {
            adjacency_list.entry(from).or_default().push(to);
        }

        Graph { adjacency_list }
    }

    // https://en.wikipedia.org/wiki/Topological_sorting#Kahn's_algorithm
    pub fn topological_sort(&self) -> Option<Vec<i32>> {
        let mut in_degree: HashMap<i32, usize> = HashMap::new();
        for node in self.adjacency_list.keys() {
            in_degree.entry(*node).or_default();
        }

        for neighbors in self.adjacency_list.values() {
            for &neighbor in neighbors {
                *in_degree.entry(neighbor).or_default() += 1;
            }
        }

        let mut queue: VecDeque<i32> = in_degree
            .iter()
            .filter_map(|(&node, &degree)| if degree == 0 { Some(node) } else { None })
            .collect();

        let mut sorted = Vec::new();

        while let Some(node) = queue.pop_front() {
            sorted.push(node);
            if let Some(neighbors) = self.adjacency_list.get(&node) {
                for &neighbor in neighbors {
                    if let Some(degree) = in_degree.get_mut(&neighbor) {
                        *degree -= 1;
                        if *degree == 0 {
                            queue.push_back(neighbor);
                        }
                    }
                }
            }
        }

        if sorted.len() == self.adjacency_list.len() {
            Some(sorted)
        } else {
            None
        }
    }
}
