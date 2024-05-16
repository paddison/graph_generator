use super::util::Lcg;
/// Represents a graph with randomly created edges.
pub struct RandomGraph {
    num_edges: u32,
}

impl RandomGraph {
    pub fn new(num_edges: u32) -> Self {
        Self { num_edges }
    }

    /// Creates edges of a graph randomly.
    /// The graph created from the edges will be acyclic.
    pub fn build_edges(&self) -> Vec<(u32, u32)> {
        let mut rng = Lcg::new();
        let mut edges = vec![(0, 1)];

        while edges.len() < self.num_edges as usize {
            let current_edge = edges[rng.generate_range(edges.len())];

            loop {
                let next_successor = rng.generate_range(self.num_edges as usize + 1) as u32;
                let next_predecessor = [current_edge.0, current_edge.1][rng.generate_range(2)];
                if next_successor == next_predecessor {
                    continue;
                }
                let next_edge = (next_predecessor, next_successor);
                if edges.iter().find(|e| e == &&next_edge).is_none() {
                    edges.push(next_edge);
                    if RandomGraph::contains_cycle(&edges) {
                        edges.pop();
                    } else {
                        break;
                    }
                }
            }
        }

        edges
    }

    /// Checks if the edges of the graph contain a cycle.
    fn contains_cycle(edges: &[(u32, u32)]) -> bool {
        let mut visited = std::collections::HashSet::new();
        for edge in edges {
            if visited.contains(edge) {
                continue;
            }
            let mut frontier = vec![edge];
            while let Some(edge) = frontier.pop() {
                for successor in edges.iter().filter(|(p, _)| edge.1 == *p) {
                    if visited.contains(successor) {
                        return true;
                    }
                    frontier.push(successor);
                }
                visited.insert(edge);
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::RandomGraph;

    #[test]
    fn test_random_layout() {
        let layout = RandomGraph::new(5);
        let edges = layout.build_edges();
        println!("{:?}", edges);
    }

    #[test]
    fn test_contains_cycle() {
        assert!(RandomGraph::contains_cycle(&[
            (0, 1),
            (0, 5),
            (0, 2),
            (2, 5),
            (5, 0)
        ]));
    }
}
