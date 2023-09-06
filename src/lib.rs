use std::fs::File;
use std::io::Write;

use rand::prelude::*;

/// Creates a graph with `num_nodes` vertices, which have `edges_per_node` edges.
/// The layout of the graph is layered, where it grows from one vertice to a certain maximum with
/// and then starts shrinking again to one vertice at the bottom layer:
/// ```
///             /---v---\
///          /-v-\    /-v-\
///         v    v   v    v 
///          \-v-/   \-v-/   
///            \---v---/
/// ```
/// The number of layers depends on the number of vertices.
/// More specifically, if edges^(n) - 1 + edges^(n - 1) <= num_vertices <= edges^(n+1) - 1 + edges^(n), 
/// we will have at maximum 2n + 1 layers, and at minimum n layers.
#[derive(Debug, Eq, PartialEq)]
pub struct GraphLayout {
    growing_layers: u32,
    shrinking_layers: u32,
    num_nodes: u32,
    edges_per_node: u32,
}

impl GraphLayout {
    fn new(growing_nodes: u32, shrinking_nodes: u32, num_nodes: u32, edges_per_node: u32) -> Self {
        return GraphLayout { growing_layers: growing_nodes, shrinking_layers: shrinking_nodes, num_nodes, edges_per_node }
    }

    /// Not implemented.
    fn new_from_num_edges(_num_edges: usize, _edges_per_node: usize) -> Self {
        unimplemented!()
    }

    /// Creates a new layout which will have `num_nodes` vertices and `edges_per_node` edges per vertice.
    pub fn new_from_num_nodes(num_nodes: u32, edges_per_node: u32) -> Self {
        let calc_num_nodes = |edges_per_node: u32, pow: u32| (edges_per_node.pow(pow) - 1) / (edges_per_node - 1);
        for i in 0..{
            let growing_nodes = calc_num_nodes(edges_per_node, i);
            let shrinking_nodes = calc_num_nodes(edges_per_node, i + 1);
            if growing_nodes + shrinking_nodes >= num_nodes {
                return GraphLayout{ growing_layers: i, shrinking_layers: (i + 1), num_nodes, edges_per_node };
            }
        }
        unreachable!()
    }

    /// Build the edges of the graph.
    pub fn build_edges(&self) -> Vec<(u32, u32)> {
        // start with node = 0
        let mut edges = Vec::new();
        let mut node = 0;
        for layer in 0..self.growing_layers {
            let layer_size = self.edges_per_node.pow(layer as u32);
            for _ in 0..layer_size {
                for edge in 1..=self.edges_per_node {
                    edges.push((node, self.edges_per_node * node + edge));
                    if edges.len() as u32 + 1 == self.num_nodes {
                        return edges;
                    }
                }
                node += 1;
            }
        }

        for layer in (1..self.shrinking_layers).rev() {
            let mut layer_size = self.edges_per_node.pow(layer as u32);
            for _ in 0..(layer_size / self.edges_per_node) {
                for edge in 0..self.edges_per_node {
                    let successor = node + layer_size - edge;
                    if successor >= self.num_nodes {
                        return edges;
                    }
                    edges.push((node, node + layer_size - edge));
                    node += 1;
                }
                layer_size -= self.edges_per_node - 1;
            }
        }
        edges
    }

}

/// Represents a graph with randomly created edges.
pub struct RandomLayout {
    num_edges: u32,
}

impl RandomLayout {
    pub fn new(num_edges: u32) -> Self {
        Self { num_edges }
    }

    /// Creates edges of a graph randomly.
    /// The graph created from the edges will be acyclic.
    pub fn build_edges(&self) -> Vec<(u32, u32)> {
        let mut rng = thread_rng();
        let mut edges = vec![(0, 1)];

        while edges.len() < self.num_edges as usize {
            let current_edge = edges[rng.gen_range(0..edges.len())];

            loop {
                let next_successor = rng.gen_range(0..=self.num_edges);  
                let next_predecessor = [current_edge.0, current_edge.1][rng.gen_range(0..2)];
                if next_successor == next_predecessor {
                    continue;
                }
                let next_edge = (next_predecessor, next_successor);
                if edges.iter().find(|e| e == &&next_edge).is_none() {
                    edges.push(next_edge);
                    if RandomLayout::contains_cycle(&edges) {
                        edges.pop();
                    } else {
                        break;
                    }
                }
            };
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

/// Write the edges of a graph to a text file.
pub fn write_to_file(filename: &str, edges: &[(u32, u32)]) -> std::io::Result<()> {
    let mut file = File::create(filename)?;
    let buffer = format!("{:?}", edges);
    file.write_all(buffer.as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{GraphLayout, RandomLayout, write_to_file};

    #[test]
    fn test_new_from_num_nodes_2_nodes_2_edges() {
        let expected = GraphLayout::new(1, 2, 2, 2);
        let actual = GraphLayout::new_from_num_nodes(2, 2);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_new_from_num_nodes_8_nodes_2_edges() {
        let expected = GraphLayout::new(2, 3, 8, 2);
        let actual = GraphLayout::new_from_num_nodes(8, 2);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_new_from_num_nodes_22_nodes_2_edges() {
        let expected = GraphLayout::new(3, 4, 22, 2);
        let actual = GraphLayout::new_from_num_nodes(22, 2);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_new_from_num_nodes_16_nodes_3_edges() {
        let expected = GraphLayout::new(2, 3, 16, 3);
        let actual = GraphLayout::new_from_num_nodes(16, 3);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_print_edges() {
        let layout = GraphLayout::new_from_num_nodes(766, 2);
        println!("{:?}", layout);
        println!("{:?}", layout.build_edges());
    }

    #[test]
    fn test_write_to_file() {
        let layout = GraphLayout::new_from_num_nodes(1000, 3);
        let _ = write_to_file("1000_3", &layout.build_edges());
    }

    #[test]
    fn test_random_layout() {
        let layout = RandomLayout::new(5);
        let edges = layout.build_edges();
        println!("{:?}", edges);
    }

    #[test]
    fn test_contains_cycle() {
        assert!(RandomLayout::contains_cycle(&[(0, 1), (0, 5), (0, 2), (2, 5), (5, 0)]));
    }
}