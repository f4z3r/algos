//! Implemetation of a genetic algorithm example.
//!
//! It seeks at finding a bitstring with a maximum number of 1s.

#![allow(unused_assignments)]

use rand;
use rand::Rng;

/// Run the algorithm.
///
/// # Args
/// - `max_gens`: the maximal number of generations to run.
/// - `pop_size`: the size of population in each generation.
/// - `num_bits`: the number of bits in the gene string.
/// - `prob_cross`: the probability of a cross over.
/// - `prob_mut`: the probability of mutation _in a single bit_ of the gene string.
pub fn run(max_gens: usize, pop_size: usize, num_bits: usize, prob_cross: f64, prob_mutation: f64) -> String {
    // initialise the population
    let mut population: Vec<(String, usize)> = Vec::new();
    for _ in 0..pop_size {
        let bitstring = random_bistring(num_bits);
        let fitness = onemax(&bitstring);
        population.push((bitstring, fitness));
    }

    // find the best gene in the population
    population.sort_by(|a, b| b.1.cmp(&a.1));
    let mut best = population.first().unwrap().clone();

    // create new generations
    for gen in 0..max_gens {
        // perform a binary selection to select good genes in the population
        let mut selected: Vec<(String, usize)> = Vec::new();
        for _ in 0..pop_size {
            selected.push(binary_tournament(&population));
        }
        // make the good genes reproduce
        let mut children = reproduce(selected, pop_size, prob_cross, prob_mutation);

        // find the best gene within the children and save it if it is better than previous generation
        children.sort_by(|a, b| b.1.cmp(&a.1));
        let children_best = children.first().unwrap().clone();
        if children_best.1 >= best.1 {
            best = children_best;
        }

        // let the children be the new population
        population = children;
        println!(" > generation {}, best: {}, {}", gen, best.1, best.0);

        // if the best gene in the population is perfect, stop
        if best.1 == num_bits {
            break;
        }
    }

    // return the best gene after convergence or max iteration reached
    best.0.clone()
}


/// Produces a new generation from an existing one. The population reproduces two by two.
///
/// # Args
/// - `prob_cross`: probability of a crossover, aka that the child inherits genes from both parents and not only one.
/// - `prob_mutation`: probability of a mutation to appear in the child _for each bit_.
fn reproduce(selected: Vec<(String, usize)>, pop_size: usize, prob_cross: f64, prob_mutation: f64) -> Vec<(String, usize)> {
    // initialise the new population
    let mut children: Vec<(String, usize)> = Vec::new();
    for (val, (p1, _)) in selected.iter().enumerate() {
        // get two consecutive parents
        let p2 = if val == selected.len() - 1 {
            selected[0].0.clone()
        } else if val % 2 == 0 {
            selected[val+1].0.clone()
        } else {
            selected[val-1].0.clone()
        };

        // build the child
        let mut child = String::new();
        // give him genes from parents based on crossover probability
        child = crossover(&p1, &p2, prob_cross);
        // add mutation to his gene based on mutation probability
        child = point_mutation(&child, prob_mutation);
        // compute his fitness and add him top the child pool
        let fitness = onemax(&child);
        children.push((child, fitness));
        if children.len() >= pop_size { break; }
    }
    children
}

/// Adds point-wise mutation to the gene bitstring of an individual based on the probability given.
fn point_mutation(string: &str, prob_mutation: f64) -> String {
    let mut child = String::new();
    for ch in string.chars() {
        // mutate based on mutation probability
        if rand::random::<f64>() < prob_mutation {
            child.push(if ch == '0' { '1' } else { '0' });
        } else {
            child.push(ch);
        }
    }
    child
}

/// Computes the gene bitstring of the child based on the parents gene bitstrings and crossover proabilities.
fn crossover(bitstring1: &str, bitstring2: &str, prob_cross: f64) -> String {
    // if no crossover, get only genes from a single parent
    if rand::random::<f64>() > prob_cross {
        return String::from(bitstring1);
    }

    // otherwise, get parts of both gene pools
    let turnpoint: usize = 1 + rand::thread_rng().gen_range(0, bitstring1.len() - 2);
    let mut child = String::from(&bitstring1[..turnpoint]);
    child.push_str(&bitstring2[turnpoint..]);
    child
}

/// Perform a sort of natural selection. This defines what genes get to reproduce. Note that in the worst case, only the
/// weakest gene is discarded. However, on average, about the better half of genes are selected.
fn binary_tournament(pop: &Vec<(String, usize)>) -> (String, usize) {
    // get two different individuals
    let mut rng = rand::thread_rng();
    let i: usize = rng.gen_range(0, pop.len());
    let mut j: usize = rng.gen_range(0, pop.len());
    while i == j { j = rng.gen_range(0, pop.len()); }

    // return the most fit one
    if pop[i].1 > pop[j].1 {
        return pop[i].clone();
    } else {
        return pop[j].clone();
    }
}

/// Function computing the fitness of a gene. This is the objective function one tries to maximise in this problem.
fn onemax(bitstring: &str) -> usize {
    // compute the number of 1s in the gene bitstring
    bitstring.chars().fold(0_usize, |acc, i| if i == '1' {acc + 1} else {acc})
}

/// Return a random bitstring. This function is only used to initialise the very first population. It initialises the
/// bitstring with half 1s and half 0s in expectation.
fn random_bistring(num_bits: usize) -> String {
    (0..num_bits).fold(String::new(), |s, _|
        if rand::random::<f64>() > 0.5 {
            let mut t = String::from(s);
            t.push('0');
            t
        } else {
            let mut t = String::from(s);
            t.push('1');
            t
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_genetic() {
        let num_bits = 64_usize;
        let max_gens = 100_usize;
        let pop_size = 100_usize;
        let prob_cross = 0.98_f64;
        let prob_mutation = 1.0_f64 / num_bits as f64;

        let best = run(max_gens, pop_size, num_bits, prob_cross, prob_mutation);

        assert_eq!(best, "1111111111111111111111111111111111111111111111111111111111111111");
    }
}
