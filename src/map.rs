use fixedbitset::FixedBitSet;
use itermore::IterMore;
use petgraph::matrix_graph::{node_index, MatrixGraph, NodeIndex};
use rand::prelude::*;
use text_io::scan;

pub fn read_graph() -> MatrixGraph<(), f64> {
    let n: usize;
    scan!("{}\n", n);
    if n <= 2 {
        panic!("Wrong graph!")
    }
    let mut graph = MatrixGraph::<(), f64>::with_capacity(n);
    for _ in 0..n {
        graph.add_node(());
    }
    for i in 0..n {
        for j in 0..n {
            let weight: f64;
            scan!("{}", weight);
            if i == j {
                if weight != 0.0 {
                    panic!("self-loop edge must have weight 0!");
                }
            } else {
                if weight != -1.0 {
                    graph.add_edge(node_index(i), node_index(j), weight);
                }
            }
        }
    }
    graph
}

pub fn generate_solution(graph: &MatrixGraph<(), f64>, from: NodeIndex) -> Option<Vec<NodeIndex>> {
    // search this graph in dfs-order.
    let mut stack = Vec::<(NodeIndex, i32)>::with_capacity(graph.node_count());
    let mut visited = FixedBitSet::with_capacity(graph.node_count());
    let mut rng = thread_rng();
    stack.push((from, 0));
    while !stack.is_empty() {
        let (now, action) = stack.pop().unwrap();
        if action == 0 {
            // should step in.
            if visited.contains(now.index()) {
                continue;
            }
            visited.toggle(now.index());
            stack.push((now, 1));
            for n in (graph.neighbors(now).collect::<Vec<_>>()) {
                if !visited.contains(n.index()) {
                    stack.push((n, 0));
                }
            }
            if visited.count_ones(..) == graph.node_count() && graph.has_edge(now, from) {
                let mut ret = Vec::new();
                for (n, _) in stack.iter().filter(|(_, a)| *a == 1) {
                    ret.push(*n)
                }
                ret.push(node_index(0));
                return Some(ret);
            }
        } else if action == 1 {
            if !visited.contains(now.index()) {
                panic!(
                    "Error! Visiting: {}, Stack: {:?}, Vis: {}",
                    now.index(),
                    stack,
                    visited
                )
            }
            visited.toggle(now.index());
            // println!("End visiting {}, vis {}, stack {:?}", now.index(), visited, stack);
        }
    }
    None
}

pub fn verify_solution(
    graph: &MatrixGraph<(), f64>,
    path: &mut dyn Iterator<Item = &NodeIndex>,
) -> f64 {
    let mut visited = FixedBitSet::with_capacity(graph.node_count());
    let length = path
        .windows()
        .map(|[prev, next]| {
            if visited.contains(next.index()) {
                return f64::MAX;
            }
            visited.put(next.index());
            if !graph.has_edge(*prev, *next) {
                return f64::MAX;
            }
            return *graph.edge_weight(*prev, *next);
        })
        .fold(0f64, |sum, v| {
            if sum == f64::MAX || v == f64::MAX {
                f64::MAX
            } else {
                sum + v
            }
        });
    if visited.count_ones(..) == graph.node_count() {
        length
    } else {
        f64::MAX
    }
}
