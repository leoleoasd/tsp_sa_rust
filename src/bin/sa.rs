#![feature(trait_upcasting)]

use fixedbitset::FixedBitSet;
use image::error::ImageFormatHint::Name;
use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};
use itermore::IterMore;
use petgraph::matrix_graph::{node_index, MatrixGraph, NodeIndex};
use rand::prelude::*;
use std::cmp::max;
use std::mem::swap;
use text_io::read;
use tsp_sa::map::*;

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
}

fn simulated_annealing(graph: &MatrixGraph<(), f64>) -> Option<Vec<NodeIndex>> {
    let mut solution = generate_solution(graph, node_index(0))?;
    let mut solution_length = verify_solution(graph, solution.iter().copied());

    let mut i = (TemperatureIterator {
        start: 500f64,
        end: 1e-5,
        q: 1.0 - 1e-4,
    });
    let bar = ProgressBar::new(i.len() as u64);
    bar.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed} / {eta}]({per_sec}) {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
            .progress_chars("##-"),
    );
    let mut i = i.progress_with(bar);
    while let Some(mut t) = i.next() {
        for _ in 0..400 {
            // let mut new_solution = solution.clone();
            let mut iter = change_solution(&solution);
            let diff = verify_solution(graph, &mut iter) - solution_length;
            if diff < 0.0 {
                solution = iter.operate();
                solution_length = verify_solution(graph, solution.iter().copied());
            } else {
                let p = (-diff / t).exp();
                if random::<f64>() < p {
                    solution = iter.operate();
                    solution_length = verify_solution(graph, solution.iter().copied());
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
    let result1 = simulated_annealing(&graph);
    let result2 = simulated_annealing(&graph);
    // println!("{:?}", result);
    let ans1 = verify_solution(&graph, result1.unwrap().into_iter());
    let ans2 = verify_solution(&graph, result2.unwrap().into_iter());
    println!("{:?}", if ans1 < ans2 { ans1 } else { ans2 });
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
