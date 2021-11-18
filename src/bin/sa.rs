#![feature(destructuring_assignment)]

use fixedbitset::FixedBitSet;
use indicatif::ProgressIterator;
use itermore::IterMore;
use petgraph::matrix_graph::{node_index, MatrixGraph, NodeIndex};
use rand::prelude::*;
use text_io::read;
use tsp_sa::map::*;

fn change_solution(path: &mut Vec<NodeIndex>) {
    // generate a new solution by random swapping nodes in path
    if random::<f32>() < 0.5 {
        // generate a new solution by swapping nodes.
        let [x, y] = [
            rand::thread_rng().gen_range(1..path.len() - 1),
            rand::thread_rng().gen_range(1..path.len() - 1),
        ];
        (path[x], path[y]) = (path[y], path[x]);
    } else {
        let [mut x, mut y] = [
            rand::thread_rng().gen_range(1..path.len() - 1),
            rand::thread_rng().gen_range(1..path.len() - 1),
        ];
        if x > y {
            (y, x) = (x, y);
        }
        for i in 0..(y - x) / 2 {
            (path[x + i], path[y - i]) = (path[y - i], path[x + i]);
        }
    }
}

struct TemperatureIterator {
    start: f64,
    end: f64,
    q: f64,
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
        ((self.start / self.end).log2() / (1.0 / self.q).log2() + 1.0) as usize
    }
    // fn is_empty(&self) -> bool {
    //     self.start <= self.end
    // }
}

fn simulated_annealing(graph: &MatrixGraph<(), f64>) -> Option<Vec<NodeIndex>> {
    let mut solution = generate_solution(graph, node_index(0))?;
    let mut solution_length = verify_solution(graph, &mut solution.iter());
    let mut i = (TemperatureIterator {
        start: 500f64,
        end: 1e-5,
        q: 1.0 - 1e-5,
    })
    .progress();
    while let Some(mut t) = i.next() {
        for _ in 0..400 {
            let mut new_solution = solution.clone();
            change_solution(&mut new_solution);
            let diff = verify_solution(graph, &mut new_solution.iter()) - solution_length;
            if diff < 0.0 {
                solution = new_solution;
                solution_length = verify_solution(graph, &mut solution.iter());
            } else {
                let p = (-diff / t).exp();
                if random::<f64>() < p {
                    solution = new_solution;
                    solution_length = verify_solution(graph, &mut solution.iter());
                    break;
                }
            }
        }
        // print!("{}:", verify_solution(graph, &mut solution.iter()));
        // for n in &solution {
        //     print!("{} ", n.index());
        // }
        // println!();
    }
    // println!("simulated annealing done with {} cycles.", count);
    Some(solution)
}

fn main() {
    let graph = read_graph();
    let result = simulated_annealing(&graph);
    // println!("{:?}", result);
    println!("{:?}", verify_solution(&graph, &mut result.unwrap().iter()));
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
