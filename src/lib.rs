use std::fs::File;
use std::io::Write;

use comm::CubeGraph;

pub mod comm;
pub mod layered;
pub mod layered_random;
pub mod random;
pub mod util;

/// Write the edges of a graph to a text file.
pub fn write_to_file(filename: &str, edges: &[(u32, u32)]) -> std::io::Result<()> {
    let mut file = File::create(filename)?;
    
    let buffer = edges.into_iter().map(|(tail, head)| format!("{} -> {}\n", tail, head)).collect::<String>();
    file.write_all(buffer.as_bytes())?;
    Ok(())
}

#[test]
fn test_write_to_file() {
    use layered::LayeredGraph;
    let layout = LayeredGraph::new_from_num_nodes(1000, 3);
    let _ = write_to_file("1000_3", &layout.build_edges());
}

#[test]
fn cube_graph_3_dim_2_ts() {
    let layout = CubeGraph::new(3, 3, 3, 2)
        .build()
        .into_iter()
        .map(|(t, h)| (t as u32, h as u32))
        .collect::<Vec<_>>();
    let _ = write_to_file("cube_d3_ts2.txt", &layout);
}

#[test]
fn cube_graph_6_dim_3_ts() {
    let layout = CubeGraph::new(6, 6, 6, 3)
        .build()
        .into_iter()
        .map(|(t, h)| (t as u32, h as u32))
        .collect::<Vec<_>>();
    let _ = write_to_file("cube_d6_ts3.txt", &layout);
}

#[test]
fn cube_graph_8_dim_3_ts() {
    let layout = CubeGraph::new(8, 8, 8, 3)
        .build()
        .into_iter()
        .map(|(t, h)| (t as u32, h as u32))
        .collect::<Vec<_>>();

    let _ = write_to_file("cube_d8_ts3.txt", &layout);
}
