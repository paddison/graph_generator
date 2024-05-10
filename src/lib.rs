use std::fs::File;
use std::io::Write;

pub mod layered;
pub mod layered_random;
pub mod random;
pub mod util;

/// Write the edges of a graph to a text file.
pub fn write_to_file(filename: &str, edges: &[(u32, u32)]) -> std::io::Result<()> {
    let mut file = File::create(filename)?;
    let buffer = format!("{:?}", edges);
    file.write_all(buffer.as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::layered::LayeredGraph;
    use super::write_to_file;
    use crate::random::RandomGraph;

    #[test]
    fn test_new_from_num_nodes_2_nodes_2_edges() {
        let expected = LayeredGraph::new(1, 2, 2, 2);
        let actual = LayeredGraph::new_from_num_nodes(2, 2);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_new_from_num_nodes_8_nodes_2_edges() {
        let expected = LayeredGraph::new(2, 3, 8, 2);
        let actual = LayeredGraph::new_from_num_nodes(8, 2);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_new_from_num_nodes_22_nodes_2_edges() {
        let expected = LayeredGraph::new(3, 4, 22, 2);
        let actual = LayeredGraph::new_from_num_nodes(22, 2);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_new_from_num_nodes_16_nodes_3_edges() {
        let expected = LayeredGraph::new(2, 3, 16, 3);
        let actual = LayeredGraph::new_from_num_nodes(16, 3);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_print_edges() {
        let layout = LayeredGraph::new_from_num_nodes(766, 2);
        println!("{:?}", layout);
        println!("{:?}", layout.build_edges());
    }

    #[test]
    fn test_write_to_file() {
        let layout = LayeredGraph::new_from_num_nodes(1000, 3);
        let _ = write_to_file("1000_3", &layout.build_edges());
    }

    #[test]
    fn test_random_layout() {
        let layout = RandomGraph::new(5);
        let edges = layout.build_edges();
        println!("{:?}", edges);
    }

    #[test]
    fn test_contains_cycle() {
        assert!(RandomGraph::contains_cycle(&[(0, 1), (0, 5), (0, 2), (2, 5), (5, 0)]));
    }
}
