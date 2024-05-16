use std::collections::HashSet;

pub fn comp_graph(inside: usize, outside: usize, n_layers: usize) -> Vec<(usize, usize)> {
    if n_layers <= 1 || inside + outside == 0 {
        return Vec::new();
    }

    let layers = create_layers(inside + outside, n_layers);
    let mut edges = Vec::new();

    // add neighbor edges
    for (upper, lower) in layers.iter().zip(&layers[1..]) {
        for i in 0..inside + outside {
            let vertex = upper[i];
            // add left neighbor
            if let Some(left) = lower.get(i.wrapping_sub(1)) {
                edges.push((vertex, *left));
            }
            // add lower neighbor
            edges.push((vertex, lower[i]));
            // add right neibhbor
            if let Some(right) = lower.get(i + 1) {
                edges.push((vertex, *right));
            }
        }
    }

    if outside == 0 {
        return edges;
    }

    // add comm edges
    let mut comm = (inside + outside) * n_layers;

    for (upper, lower) in layers.iter().zip(&layers[1..]) {
        for (vertex_upper, vertex_lower) in upper[inside..].iter().zip(&lower[inside..]) {
            edges.push((*vertex_upper, comm));
            edges.push((comm, *vertex_lower));
        }
        comm += 1;
    }

    edges
}

#[inline(always)]
fn create_layers(nodes_per_layer: usize, n_layers: usize) -> Vec<Vec<usize>> {
    (0..n_layers)
        .map(|i| (i * nodes_per_layer..(i + 1) * nodes_per_layer).collect())
        .collect()
}

/// Builds a cube graph:
/// consists of a series of cubes, with a cube being some state at a given timestep
///
///```
///     2  5  8     t = 0
///   1  4  7
/// 0  3  6  17
///        16
/// 9 12 15  26
///        25
///18 21 24
///    |
///    |
///    v
///     2  5  8     t = 1
///   1  4  7
/// 0  3  6  17
///        16
/// 9 12 15  26
///        25
///18 21 24
///    |
///    |
///    ...          t = 2
///```
///
/// Edges are between surrounding blocks, diagonals included (a block is a vertex)
/// Vertices on one of the sides of the cube are called 'outer vertices' (0 - 9 are
/// outer vertices for example)
/// outer vertices are connected to an additional 'comm vertex' between each timestep
pub fn cube_graph(width: usize, height: usize, depth: usize, timesteps: usize) -> Vec<(usize, usize)> {
    // has to be at least 3 by 3
    if height < 3 || width < 3 || depth < 3 {
        return Vec::new();
    }

    let number_of_vertices = width * height * depth;
    let mut edges = Vec::new();

    // vertices are ordered from depth to width to height (/>v)
    for step in 0..(timesteps - 1) {
        let comm = step + number_of_vertices * timesteps;
        for id in 0..(number_of_vertices) {
            // get neihbor indices
            let mut neighbor_edges = get_neighbor_indices(id as isize, width as isize, height as isize, depth as isize)
                .into_iter()
                .map(|n| (id, n + (step + 1) * number_of_vertices))
                .collect();

            edges.append(&mut neighbor_edges);
            // add comm vertice between outer edge
            if is_outer_vertice(id, width, height, depth) {
                edges.push((id, comm));
                edges.push((comm, id + (step + 1) * number_of_vertices));
            }
        }
    }

    edges.into_iter().collect::<HashSet<_>>().into_iter().collect()
}

fn is_outer_vertice(id: usize, width: usize, height: usize, depth: usize) -> bool {
    // outer vertices can be determined in the following way:
    // top:
    id < depth * width ||
    // bottom: 
    id >= depth * width * (height - 1) ||
    // front:
    id % depth == 0 ||
    // back:
    id % depth == depth - 1 ||
    // left:
    id % (depth * width) < depth ||
    // right:
    id % (depth * width) > (depth - 1) * width
}

fn get_neighbor_indices(id: isize, width: isize, height: isize, depth: isize) -> Vec<usize> {
    let ops = [0, 1, -1];
    let z = id / (width * depth);
    let y = (id / depth) % width;
    let x = id % depth;

    // nodes always have to be on adjacent points

    (0..3)
        .flat_map(|i| {
            (0..3).flat_map(move |j| {
                (0..3).map(move |k| id + depth * width * ops[i] + 1 * ops[j] + depth * ops[k])
            })
        })
        .filter(|n| 
                // top
                *n >= 0 && 
                // bottom
                *n < width * height * depth && 
                // left && right
                y.abs_diff((*n / depth) % width) <= 1 && 
                // front && back
                x.abs_diff(*n % depth) <= 1)
        .map(|n| n as usize)
        .collect()
}

fn calc_neighbor(id: isize, width: isize, height: isize, depth: isize) -> isize {
    1 //id as isize + width as isize * ops[i] + height as isize * ops[j] + depth as isize * ops[k]
}

#[test]
fn test_create_layers() {
    let actual = create_layers(7, 3);
    let expected = vec![
        vec![0, 1, 2, 3, 4, 5, 6],
        vec![7, 8, 9, 10, 11, 12, 13],
        vec![14, 15, 16, 17, 18, 19, 20],
    ];

    assert_eq!(actual, expected);
}

#[test]
fn test_create_comp_graph_empty() {
    assert!(comp_graph(123, 123, 1).is_empty());
    assert!(comp_graph(123, 123, 0).is_empty());
    assert!(comp_graph(0, 0, 1999).is_empty());
}

#[test]
fn test_create_comp_graph_no_outside() {
    let actual = comp_graph(3, 0, 2);
    let expected = vec![(0, 3), (0, 4), (1, 3), (1, 4), (1, 5), (2, 4), (2, 5)];
    assert_eq!(actual, expected);
}

#[test]
fn test_create_comp_graph_no_inside() {
    use std::collections::HashSet;
    let actual = comp_graph(0, 3, 2).into_iter().collect::<HashSet<_>>();
    let expected = HashSet::from([
        (0, 3),
        (0, 4),
        (1, 3),
        (1, 4),
        (1, 5),
        (2, 4),
        (2, 5),
        (0, 6),
        (1, 6),
        (2, 6),
        (6, 3),
        (6, 4),
        (6, 5),
    ]);
    assert_eq!(actual, expected);
}

#[test]
fn test_create_comp_graph() {
    use std::collections::HashSet;
    let actual = comp_graph(3, 1, 2).into_iter().collect::<HashSet<_>>();
    let expected = HashSet::from([
        (0, 4),
        (0, 5),
        (1, 4),
        (1, 5),
        (1, 6),
        (2, 5),
        (2, 6),
        (2, 7),
        (3, 6),
        (3, 7),
        (3, 8),
        (8, 7),
    ]);
    assert_eq!(actual, expected);
}

#[test]
fn test_create_comp_graph_larg() {
    let edges = comp_graph(10, 5, 10)
        .into_iter()
        .map(|(t, h)| (t as u32, h as u32))
        .collect::<Vec<(u32, u32)>>();
    println!("{:?}", edges);
}

#[test]
fn test_cube_graph() {
    let mut edges = cube_graph(8, 8, 8, 4);
    edges.sort_by(|a, b| a.0.cmp(&b.0));
    println!("{edges:?}\n{}", edges.len());
}

#[test]
fn test_neighbor_indices() {
    let neighbors = get_neighbor_indices(13, 3, 3, 3);
    println!("{neighbors:?}");
}
