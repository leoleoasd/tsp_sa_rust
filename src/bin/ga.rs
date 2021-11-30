// #![feature(std)]
use fixedbitset::FixedBitSet;
use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};
use permutation_iterator::Permutor;
use petgraph::graph::Node;
use petgraph::matrix_graph::{node_index, MatrixGraph, NodeIndex};
use rand::prelude::*;
use rayon::prelude::*;
use tsp_sa::map::*;

/// Mix two solutions.
/// Select a random point from solution a. Copy [0, pos) to result
/// then fill the rest in the order of b
fn mix(a: &Vec<NodeIndex>, b: &Vec<NodeIndex>) -> Vec<NodeIndex> {
    let mut rng = rand::thread_rng();
    let mut result = Vec::new();
    result.reserve_exact(a.len());
    let pos = rand::thread_rng().gen_range(1..a.len() - 1);
    // println!("{}", pos);
    let mut vis = FixedBitSet::with_capacity(a.len());
    for i in 0..pos {
        result.push(a[i]);
        if i != 0 {
            vis.insert(a[i].index());
        }
    }
    for i in b.iter().skip(1) {
        if !vis.contains(i.index()) {
            result.push(*i);
            vis.insert(i.index());
        }
    }
    result
}

/// Run genetic algorithm
fn genetic_algorithm(graph: &MatrixGraph<(), f64>) -> Option<Vec<NodeIndex>> {
    let mut solutions: Vec<Box<Vec<NodeIndex>>> = (0..200)
        .into_par_iter()
        .map(|_: u32| generate_solution(&graph, node_index(0)))
        .filter_map(|x| Some(Box::new(x?)))
        .collect();
    let bar = ProgressBar::new(200);
    bar.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed} / {eta}]({per_sec}) {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
            .progress_chars("##-"),
    );
    let mut p = (0..200).progress_with(bar);
    while let Some(_) = p.next() {
        let mut mixed_solutions = Permutor::new(solutions.len() as u64)
            .zip(0..solutions.len() as u64)
            .collect::<Vec<_>>()
            .par_iter()
            .map(|(i, j)| mix(&solutions[*i as usize], &solutions[*j as usize]))
            .collect::<Vec<_>>();
        solutions.append(&mut mixed_solutions);
        let mut changed_solutions = solutions
            .par_iter()
            .map(|x| Box::new(change_solution(x).collect::<Vec<_>>()))
            .collect::<Vec<_>>();
        solutions.append(&mut changed_solutions);

        let distances: Vec<f64> = solutions
            .par_iter()
            .map(|x: &Box<Vec<NodeIndex>>| verify_solution(&graph, x.iter().copied()))
            .collect();
        let mut rng = rand::thread_rng();
        let min_dist = *distances.iter().min_by(|a, b| a.partial_cmp(b).unwrap())?;
        solutions = solutions
            .iter()
            .zip(distances.iter().copied())
            .collect::<Vec<_>>()
            .choose_multiple_weighted(&mut rng, 200, |x| 1.0 / (x.1 - min_dist + 1e-4).exp())
            .ok()?
            .map(|x| (*x.0).clone())
            .collect::<Vec<_>>();
        p.progress.set_message(format!(
            "{}",
            solutions
                .iter()
                .map(|x| { verify_solution(&graph, x.iter().copied()) })
                .min_by(|a, b| { a.partial_cmp(b).unwrap() })?
        ));
        // for n in solutions.iter().min_by(|a, b| {
        //     verify_solution(&graph, a.iter().copied())
        //         .partial_cmp(&verify_solution(&graph, b.iter().copied()))
        //         .unwrap()
        // })?.iter() {
        //     print!("{} ", n.index());
        // }
        // println!();
    }
    Some(
        *solutions
            .into_iter()
            .min_by(|x, y| {
                verify_solution(&graph, x.iter().copied())
                    .partial_cmp(&verify_solution(&graph, y.iter().copied()))
                    .unwrap()
            })
            .unwrap(),
    )
    // let solutions_with_
}

fn main() {
    let graph = read_graph();
    let solution = genetic_algorithm(&graph);
    // println!("{:?}", solution);
    println!(
        "{}",
        verify_solution(&graph, solution.unwrap().iter().copied())
    );
}
