#![feature(destructuring_assignment)]

use std::borrow::Borrow;
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::convert::Infallible;
use std::error::Error;
use std::io::BufRead;

use rand::prelude::*;
use petgraph::Directed;
use petgraph::Direction::Outgoing;
use petgraph::matrix_graph::{MatrixGraph, node_index, NodeIndex};
use petgraph::visit::{Bfs, Dfs, DfsPostOrder, EdgeRef, IntoNodeIdentifiers};
use petgraph::visit::NodeIndexable;
use fixedbitset::FixedBitSet;
use text_io::read;
use itertools::Itertools;
use itermore::IterMore;
use indicatif::ProgressIterator;

// use io::Error;

fn read_graph() -> MatrixGraph<(), f64> {
    let n: usize = read!("{}\n");
    if n <= 2 {
        panic!("Wrong graph!")
    }
    let mut graph = MatrixGraph::<(), f64>::with_capacity(n);
    for i in 0..n {
        graph.add_node(());
    }
    for i in 0..n {
        for j in 0..n {
            let weight: f64 = read!("{}");
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

fn generate_solution(graph: &MatrixGraph<(), f64>, from: NodeIndex) -> Option<Vec<NodeIndex>> {
    // search this graph in dfs-order.
    let mut stack = Vec::<(NodeIndex, i32)>::with_capacity(graph.node_count());
    let mut visited = FixedBitSet::with_capacity(graph.node_count());
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
            for n in graph.neighbors(now) {
                if !visited.contains(n.index()) {
                    stack.push((n, 0));
                }
            }
            if visited.count_ones(..) == graph.node_count() && graph.has_edge(now, from) {
                let mut ret = Vec::new();
                for (n, a) in stack.iter().filter(|(n, a)| {
                    *a == 1
                }) {
                    ret.push(*n)
                }
                ret.push(node_index(0));
                return Some(ret);
            }
        } else if action == 1 {
            if !visited.contains(now.index()) {
                panic!("Error! Visiting: {}, Stack: {:?}, Vis: {}", now.index(), stack, visited)
            }
            visited.toggle(now.index());
            // println!("End visiting {}, vis {}, stack {:?}", now.index(), visited, stack);
        }
    }
    None
}

fn verify_solution(graph: &MatrixGraph<(), f64>, path: &mut dyn Iterator<Item=&NodeIndex>) -> f64 {
    let mut visited = FixedBitSet::with_capacity(graph.node_count());
    let length = path.windows().map(|[prev, next]| {
        if visited.contains(next.index()) {
            return f64::MAX;
        }
        visited.put(next.index());
        if !graph.has_edge(*prev, *next) {
            return f64::MAX;
        }
        return *graph.edge_weight(*prev, *next);
    }).fold(0f64, |sum, v| {
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

fn change_solution(graph: &MatrixGraph<(), f64>, path: &mut Vec<NodeIndex>) {
    // generate a new solution by random swapping nodes in path
    if random::<f32>() < 0.5 {
        // generate a new solution by swapping nodes.
        let [x, y] = [rand::thread_rng().gen_range(0..graph.node_count()), rand::thread_rng().gen_range(0..graph.node_count())];
        (path[x], path[y]) = (path[y], path[x]);
    } else {
        let [x, y] = [rand::thread_rng().gen_range(0..graph.node_count()), rand::thread_rng().gen_range(0..graph.node_count())];
        for i in x..(x+y)/2 {
            (path[x + i], path[y - i]) =(path[y - i], path[x + i]);
        }
    }
}


struct TemperatureIterator {
    start: f64,
    end: f64,
    q: f64
}

impl Iterator for TemperatureIterator {
    type Item = f64;
    fn next(&mut self) -> Option<Self::Item> {
        if self.start <= self.end {
            None
        } else {
            self.start *= self.q;
            Some(self.start)
        }
    }
    // fn size_hint(&self) -> (usize, Option<usize>) {
    //     (((self.start / self.end).log2() / (1.0/self.q).log2()) as usize, Some(((self.start / self.end).log2() / (1.0/self.q).log2() + 1.0) as usize))
    // }
}

impl ExactSizeIterator for TemperatureIterator {
    fn len(&self) -> usize {
        ((self.start / self.end).log2() / (1.0/self.q).log2() + 1.0) as usize
    }
    // fn is_empty(&self) -> bool {
    //     self.start <= self.end
    // }
}

fn simulated_annealing(graph: &MatrixGraph<(), f64>) -> Option<Vec<NodeIndex>> {
    let mut solution = generate_solution(graph, node_index(0))?;
    let mut solution_length = verify_solution(graph, & mut solution.iter());
    let mut count = 0;
    // println!("{}", verify_solution(graph, &mut initial_solution.iter()));
    // let bar = ProgressBar::new(1448);
    for t in (TemperatureIterator{start: 50000f64, end: 1e-8, q: 0.98}).progress() {
        for _ in 0..1000 {
            // do 1000 times
            let mut new_solution = solution.clone();
            change_solution(graph, & mut new_solution);
            let diff = verify_solution(graph, & mut new_solution.iter()) - solution_length;
            if diff < 0.0 {
                solution = new_solution;
                solution_length = verify_solution(graph, & mut solution.iter());
            } else {
                let p =  (-diff / t).exp();
                if random::<f64>() < p {
                    solution = new_solution;
                    solution_length = verify_solution(graph, & mut solution.iter());
                }
            }
        }
        count += 1;
        // bar.inc(1)
    }
    // bar.finish();
    println!("simulated annealing done with {} cycles.", count);
    Some(solution)
    // vec![1, 2]
}

fn main() {
    let graph = read_graph();
    let result = simulated_annealing(&graph);
    println!("{:?}", result);
    println!("{:?}", verify_solution(&graph, & mut result.unwrap().iter()));
}

/*
4
0 1 1 1
1 0 100 -1
1 100 0 -1
1 -1 -1 0

4
0 -1 1 1
-1 0 1000 1
1 1000 0 -1
1 1 -1 0
 */
