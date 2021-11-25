#![feature(destructuring_assignment)]
#![feature(trait_upcasting)]

use fixedbitset::FixedBitSet;
use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};
use itermore::IterMore;
use petgraph::matrix_graph::{node_index, MatrixGraph, NodeIndex};
use rand::prelude::*;
use text_io::read;
use tsp_sa::map::*;

#[derive(Clone,Copy)]
enum OperateType {
    Swap,
    Span
}

struct SwapSpanIterator<'a> {
    t: OperateType,
    path: & 'a mut Vec<NodeIndex>,
    x: usize, y: usize,
    next: usize,
}

impl Iterator for SwapSpanIterator<'_> {
    type Item = NodeIndex;
    fn next(&mut self) -> Option<Self::Item> {
        if self.next == self.path.len() {
            None
        } else {
            match self.t {
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
                    } else{
                        self.next += 1;
                        Some(self.path[self.next - 1])
                    }
                }
            }
        }
    }
}

impl SwapSpanIterator<'_> {
    fn operate(& mut self) {
        match self.t {
            OperateType::Swap => {
                self.path.swap(self.x, self.y);
            }
            OperateType::Span => {
                for i in 0..(self.y - self.x) / 2 {
                    self.path.swap(self.x + i, self.y - i);
                    // (self.path[self.x + i], self.path[self.y - i]) = (self.path[self.y - i], self.path[self.x + i]);
                }
            }
        }
    }
}

fn change_solution(path: &mut Vec<NodeIndex>) -> SwapSpanIterator {
    // generate a new solution by random swapping nodes in path
    let [mut x, mut y] = [
        rand::thread_rng().gen_range(1..path.len() - 1),
        rand::thread_rng().gen_range(1..path.len() - 1),
    ];
    if x > y {
        (y, x) = (x, y);
    }
    SwapSpanIterator{
        t: if random::<f64>() > 0.5 {
            OperateType::Swap
        } else {
            OperateType::Span
        },
        path,
        x,
        y,
        next: 0
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
}

fn simulated_annealing(graph: &MatrixGraph<(), f64>) -> Option<Vec<NodeIndex>> {
    let mut solution = generate_solution(graph, node_index(0))?;
    let mut solution_length = verify_solution(graph,solution.iter().copied());

    let mut i = (TemperatureIterator {
        start: 500f64,
        end: 1e-5,
        q: 1.0 - 1e-3,
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
            let mut iter = change_solution(&mut solution);
            let diff = verify_solution(graph, &mut iter) - solution_length;
            if diff < 0.0 {
                iter.operate();
                solution_length = verify_solution(graph, solution.iter().copied());
            } else {
                let p = (-diff / t).exp();
                if random::<f64>() < p {
                    iter.operate();
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
    let result = simulated_annealing(&graph);
    // println!("{:?}", result);
    println!("{:?}", verify_solution(&graph, Box::new(result.unwrap().into_iter())));
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
