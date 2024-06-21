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

type Cube = Vec<Vec<Vec<usize>>>;

pub struct CubeGraph {
    cubes: Vec<Cube>,
    width: usize,
    height: usize,
    depth: usize,
    timesteps: usize,
}

impl CubeGraph {
    pub fn new(width: usize, height: usize, depth: usize, timesteps: usize) -> Self {
        let mut id = 0;
        let mut cubes = Vec::new();
        for t in 0..timesteps {
            let mut cube = vec![vec![vec![0; depth]; height];width];
            for x in 0..width {
                for y in 0..height {
                    for z in 0..depth {
                        cube[x][y][z] = id;
                        id += 1;
                    }
                }
            }
            cubes.push(cube);
        }
        Self { cubes, width, height, depth, timesteps }
    }

    pub fn build(self) -> Vec<(usize, usize)> {
        let mut edges = Vec::new();
        let mut comm_id = self.width * self.height * self.depth * self.timesteps;

        for ts in 0..(self.timesteps - 1) {
            for x in 0..self.width {
                for y in 0..self.height {
                    for z in 0..self.depth {
                        let cur = self.cubes[ts][x][y][z];
                        self.get_neighbors(x, y, z, ts)
                            .into_iter()
                            .map(|n| (cur, n))
                            .for_each(|e| edges.push(e));
                        
                        if self.is_outer_vertex(x, y, z) {
                            edges.push((cur, comm_id));
                            edges.push((comm_id, self.cubes[ts + 1][x][y][z]));
                        }
                    }
                }
            }
            comm_id += 1;
        }

        edges
    }

    fn get_neighbors(&self, x: usize, y: usize, z: usize, ts: usize) -> Vec<usize> {
        let modifiers = [usize::MAX, 0, 1];
        let mut neighbors = Vec::new();
        for i in modifiers.clone() {
            for j in modifiers.clone() {
                for k in modifiers.clone() {
                    if i == 0 && j == 0 && k == 0 {
                        continue;
                    }
                    let n = self.cubes
                        .get(ts + 1)
                        .map(|xx| xx.get(x.wrapping_add(i)))
                        .flatten()
                        .map(|yy| yy.get(y.wrapping_add(j)))
                        .flatten()
                        .map(|zz| zz.get(z.wrapping_add(k)))
                        .flatten()
                        .copied();

                    if let Some(n) = n {
                        neighbors.push(n);
                    }
                }
            }
        }

        neighbors
    }

    fn is_outer_vertex(&self, x: usize, y: usize, z: usize) -> bool {
        x == 0 || x == self.width - 1 || 
        y == 0 || y == self.height - 1 ||
        z == 0 || z == self.depth - 1
    }
}

#[test]
fn cube_graph_vec() {
    let graph = CubeGraph::new(3, 3, 3, 2); 
    for ts in &graph.cubes {
        for x in ts {
            for y in x {
                println!("{y:?}");
            }
            println!("");
        }
        println!("");
    }
}

#[test]
fn cube_graph_neighbors() {
    let graph = CubeGraph::new(3, 3, 3, 2);
    println!("{:?}", graph.get_neighbors(0, 0, 0, 0));
    println!("{:?}", graph.get_neighbors(1, 1, 1, 0));
}

#[test]
fn cube_graph_is_outer() {
    let graph = CubeGraph::new(4, 4, 4, 1);
    assert!(graph.is_outer_vertex(0, 1, 2));
    assert!(!graph.is_outer_vertex(1, 1, 1));
}

#[test]
fn cube_graph_build() {
    let edges = CubeGraph::new(3, 3, 3, 2).build();
    println!("{edges:?}");

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
///     29 32 35     t = 1
///   28 31 34
/// 27 30 33  44
///         43
/// 36 39 42  53
///         52
/// 45 48 51
///    |
///    |
///    ...          t = 2
///```
///
/// Edges are between surrounding blocks, diagonals included (a block is a vertex)
/// Vertices on one of the sides of the cube are called 'outer vertices' (0 - 9 are
/// outer vertices for example)
/// outer vertices are connected to an additional 'comm vertex' between each timestep
pub struct CubeGraphOld {
    width: isize,
    height: isize,
    depth: isize,
    timesteps: isize,
}

impl CubeGraphOld {
    pub fn new(width: usize, height: usize, depth: usize, timesteps: usize) -> Self {
        Self {
            width: width as isize,
            height: height as isize,
            depth: depth as isize,
            timesteps: timesteps as isize,
        }
    }

    pub fn build(&self) -> Vec<(usize, usize)> {
        // has to be at least 3 by 3
        if self.height < 3 || self.width < 3 || self.depth < 3 {
            return Vec::new();
        }

        let number_of_vertices = self.size();
        let mut edges = Vec::new();

        // vertices are ordered from depth to width to height (/>v)
        for step in 0..(self.timesteps - 1) {
            let comm = step + number_of_vertices * self.timesteps;
            for id in 0..(number_of_vertices as usize) {
                // get neihbor indices
                let mut neighbor_edges = self
                    .get_neighbor_indices(id as isize)
                    .into_iter()
                    .map(|n| (id, n + ((step + 1) * number_of_vertices) as usize))
                    .collect();

                edges.append(&mut neighbor_edges);
                // add comm vertice between outer edge
                if self.is_outer_vertex(id as isize) {
                    edges.push((id, comm as usize));
                    edges.push((
                        comm as usize,
                        id + ((step + 1) * number_of_vertices) as usize,
                    ));
                }
            }
        }

        edges
            .into_iter()
            .collect::<HashSet<_>>()
            .into_iter()
            .collect()
    }

    fn is_outer_vertex(&self, id: isize) -> bool {
        // outer vertices can be determined in the following way:
        // top:
        id < self.depth * self.width ||
        // bottom: 
        id >= self.depth * self.width * (self.height - 1) ||
        // front:
        id % self.depth == 0 ||
        // back:
        id % self.depth == self.depth - 1 ||
        // left:
        id % (self.depth * self.width) < self.depth ||
        // right:
        id % (self.depth * self.width) >= (self.depth - 1) * self.width
    }

    fn get_neighbor_indices(&self, id: isize) -> Vec<usize> {
        // nodes always have to be on adjacent points
        (0..3)
            .flat_map(|i| {
                (0..3).flat_map(move |j| (0..3).map(move |k| self.calc_neighbor(id, i, j, k)))
            })
            .filter(|n| 
                    // top
                    *n >= 0 && 
                    // bottom
                    *n < self.width * self.height * self.depth && 
                    // left && right
                    self.y(id).abs_diff(self.y(*n)) <= 1 && 
                    // front && back
                    self.x(id).abs_diff(self.x(*n)) <= 1)
            .map(|n| n as usize)
            .collect()
    }

    #[inline(always)]
    fn size(&self) -> isize {
        self.width * self.height * self.depth
    }

    #[inline(always)]
    fn x(&self, id: isize) -> isize {
        id % self.depth
    }

    #[inline(always)]
    fn y(&self, id: isize) -> isize {
        id / (self.width * self.depth)
    }

    #[inline(always)]
    #[allow(dead_code)]
    fn z(&self, id: isize) -> isize {
        (id / self.depth) % self.width
    }

    #[inline(always)]
    fn calc_neighbor(&self, id: isize, i: usize, j: usize, k: usize) -> isize {
        let ops = [0, 1, -1];
        id + self.depth * self.width * ops[i] + 1 * ops[j] + self.depth * ops[k]
    }
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

fn test_cube_graph() {
    let mut edges = CubeGraphOld::new(3, 3, 3, 2).build();
    edges.sort_by(|a, b| a.0.cmp(&b.0));
    println!("{edges:?}\n{}", edges.len());
}

#[test]
fn test_neighbor_indices() {
    let g = CubeGraphOld::new(3, 3, 3, 3);
    let neighbors = g.get_neighbor_indices(13);
    println!("{neighbors:?}");
}
