# Graph Generator

Generates graph layouts, which will be returned in the form of a Vec<(predecessor, successor)>.

## Example Usage:

```
use graph_generator as GG;

let num_edges = 1000;
let edges_per_node = 3;
let layout = GG::GraphLayout::new_from_num_nodes(num_edges, edges_per_node) 
let edges = layout.build_edges();

let random_layout = GG::RandomLayout::new(num_edges);
let random_edges = random_layout.build_edges();
```
