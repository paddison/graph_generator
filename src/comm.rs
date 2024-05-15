pub fn comp_graph(inside: usize, outside: usize, n_layers: usize) -> Vec<(usize, usize)> {
    if n_layers <= 1 || inside + outside == 0{
        return Vec::new()
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
    let expected = HashSet::from([(0, 3), (0, 4), (1, 3), (1, 4), (1, 5), (2, 4), (2, 5), (0, 6), (1, 6), (2, 6), (6, 3), (6, 4), (6, 5)]);
    assert_eq!(actual, expected);

}

#[test]
fn test_create_comp_graph() {
    use std::collections::HashSet;
    let actual = comp_graph(3, 1, 2).into_iter().collect::<HashSet<_>>();
    let expected = HashSet::from([
        (0, 4), (0, 5), 
        (1, 4), (1, 5), (1, 6), 
        (2, 5), (2, 6), (2, 7),
        (3, 6), (3, 7), 
        (3, 8), (8, 7),
    ]);
    assert_eq!(actual, expected);

}
