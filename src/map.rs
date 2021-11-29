use fixedbitset::FixedBitSet;
use itermore::IterMore;
use petgraph::matrix_graph::{node_index, MatrixGraph, NodeIndex};
use rand::prelude::*;
use std::mem::swap;
use text_io::scan;

/// Read a graph map from stdin in the format of adjugate matrix.
///
/// Example:
/// ```text
/// 0 1 2 3
/// 1 0 1 1
/// 2 1 0 1
/// 3 1 1 0
/// ```
/// Returns a MatrixGraph<(), f64>.
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

/// Generate a random solution from a graph.
///
/// Specify the start point in `from`.
///
/// Returns a Vec<NodeIndex> with graph.size() + 1 elements.
///
/// The first and last element are `from`.
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
            let mut t = graph.neighbors(now).collect::<Vec<_>>();
            t.shuffle(&mut rng);
            for n in t {
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

/// Calculate the cost of a solution.
pub fn verify_solution<'a, T>(graph: &MatrixGraph<(), f64>, path: T) -> f64
    where
        T: Iterator<Item=NodeIndex>,
{
    let mut visited = FixedBitSet::with_capacity(graph.node_count());
    let length = path
        .windows()
        .map(|[prev, next]| {
            if visited.contains(next.index()) {
                return f64::MAX;
            }
            visited.put(next.index());
            if !graph.has_edge(prev, next) {
                return f64::MAX;
            }
            return *graph.edge_weight(prev, next);
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

#[derive(Clone, Copy)]
pub enum OperateType {
    Swap,
    Span,
    Move,
}

/// Change a solution in given ways.
///
/// Yields the changed solution.
pub struct ChangeSolutionOperator<'a> {
    t: OperateType,
    path: &'a Vec<NodeIndex>,
    x: usize,
    y: usize,
    z: usize,
    next: usize,
}

impl Iterator for ChangeSolutionOperator<'_> {
    type Item = NodeIndex;
    fn next(&mut self) -> Option<Self::Item> {
        if self.next == self.path.len() {
            None
        } else {
            match self.t {
                OperateType::Move => {
                    // move [x, y] after z.
                    // before:
                    // ..., x, ..., y, ..., z, ...
                    // after:
                    // ..., ..., z, x, ..., y, ...
                    if self.next < self.x {
                        self.next += 1;
                        Some(self.path[self.next - 1])
                    } else if self.next < self.z - self.y + self.x {
                        let diff = self.next + 1 - self.x;
                        self.next += 1;
                        Some(self.path[self.y + diff])
                    } else if self.next <= self.z {
                        let diff = self.next - (self.x + self.z - self.y);
                        self.next += 1;
                        Some(self.path[self.x + diff])
                    } else {
                        self.next += 1;
                        Some(self.path[self.next - 1])
                    }
                }
                OperateType::Swap => {
                    if self.x == self.next {
                        self.next += 1;
                        Some(self.path[self.y])
                    } else if self.y == self.next {
                        self.next += 1;
                        Some(self.path[self.x])
                    } else {
                        self.next += 1;
                        Some(self.path[self.next - 1])
                    }
                }
                OperateType::Span => {
                    if self.x <= self.next && self.next <= self.y {
                        self.next += 1;
                        Some(self.path[self.y - (self.next - 1 - self.x)])
                    } else {
                        self.next += 1;
                        Some(self.path[self.next - 1])
                    }
                }
            }
        }
    }
}

/// Operate changes and returns a new solution.
impl ChangeSolutionOperator<'_> {
    pub fn operate(&mut self) -> Vec<NodeIndex> {
        match self.t {
            OperateType::Move => {
                // let copy = self.path.clone();
                let new_self = ChangeSolutionOperator {
                    t: self.t,
                    path: self.path,
                    x: self.x,
                    y: self.y,
                    z: self.z,
                    next: 0,
                };
                new_self.collect()
                // *self.path = new_self.collect();
            }
            OperateType::Swap => {
                let mut new_path = self.path.clone();
                new_path.swap(self.x, self.y);
                new_path
            }
            OperateType::Span => {
                let mut new_path = self.path.clone();
                for i in 0..(self.y - self.x) / 2 {
                    new_path.swap(self.x + i, self.y - i);
                }
                new_path
            }
        }
    }
}

/// Returns a changed solution
pub fn change_solution(path: &Vec<NodeIndex>) -> ChangeSolutionOperator {
    // generate a new solution by random swapping nodes in path
    let [mut x, mut y, mut z] = [
        rand::thread_rng().gen_range(1..path.len() - 1),
        rand::thread_rng().gen_range(1..path.len() - 1),
        rand::thread_rng().gen_range(1..path.len() - 1),
    ];
    if y > z {
        swap(&mut y, &mut z);
    }
    if x > z {
        swap(&mut x, &mut z);
    }
    if x > y {
        swap(&mut x, &mut y);
    }
    ChangeSolutionOperator {
        t: if random::<f64>() > 0.3 {
            OperateType::Swap
        } else if random::<f64>() > 0.3 {
            OperateType::Span
        } else {
            OperateType::Move
        },
        path,
        x,
        y,
        z,
        next: 0,
    }
}
