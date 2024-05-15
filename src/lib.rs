use std::fs::File;
use std::io::Write;

pub mod comm;
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

#[test]
fn test_write_to_file() {
    use layered::LayeredGraph;
    let layout = LayeredGraph::new_from_num_nodes(1000, 3);
    let _ = write_to_file("1000_3", &layout.build_edges());
}
