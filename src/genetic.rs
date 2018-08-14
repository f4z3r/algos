//! Implemetation of a genetic algorithm example.

#![allow(unused_assignments)]

use rand;
use rand::Rng;

pub fn run(max_gens: usize, pop_size: usize, num_bits: usize, prob_cross: f64, prob_mutation: f64) -> String {
    let mut population: Vec<(String, usize)> = Vec::new();
    for _ in 0..pop_size {
        let bitstring = random_bistring(num_bits);
        let fitness = onemax(&bitstring);
        population.push((bitstring, fitness));
    }

    population.sort_by(|a, b| b.1.cmp(&a.1));
    let mut best = population.first().unwrap().clone();

    for gen in 0..max_gens {
        let mut selected: Vec<(String, usize)> = Vec::new();
        for _ in 0..pop_size {
            selected.push(binary_tournament(&population));
        }
        let mut children = reproduce(selected, pop_size, prob_cross, prob_mutation);
        children.sort_by(|a, b| b.1.cmp(&a.1));

        let children_best = children.first().unwrap().clone();
        if children_best.1 >= best.1 {
            best = children_best;
        }

        population = children;
        println!(" > generation {}, best: {}, {}", gen, best.1, best.0);

        if best.1 == num_bits {
            break;
        }
    }

    best.0.clone()
}

fn reproduce(selected: Vec<(String, usize)>, pop_size: usize, prob_cross: f64, prob_mutation: f64) -> Vec<(String, usize)> {
    let mut children: Vec<(String, usize)> = Vec::new();
    for (val, (p1, _)) in selected.iter().enumerate() {
        let p2 = if val == selected.len() - 1 {
            selected[0].0.clone()
        } else if val % 2 == 0 {
            selected[val+1].0.clone()
        } else {
            selected[val-1].0.clone()
        };

        let mut child = String::new();
        child = crossover(&p1, &p2, prob_cross);
        child = point_mutation(&child, prob_mutation);
        let fitness = onemax(&child);
        children.push((child, fitness));
        if children.len() >= pop_size { break; }
    }
    children
}

fn point_mutation(string: &str, prob_mutation: f64) -> String {
    let mut child = String::new();
    for ch in string.chars() {
        if rand::random::<f64>() < prob_mutation {
            child.push(if ch == '0' { '1' } else { '0' });
        } else {
            child.push(ch);
        }
    }
    child
}

fn crossover(bitstring1: &str, bitstring2: &str, prob_cross: f64) -> String {
    if rand::random::<f64>() > prob_cross {
        return String::from(bitstring1);
    }

    let turnpoint: usize = 1 + rand::thread_rng().gen_range(0, bitstring1.len() - 2);
    let mut child = String::from(&bitstring1[..turnpoint]);
    child.push_str(&bitstring2[turnpoint..]);
    child
}

fn binary_tournament(pop: &Vec<(String, usize)>) -> (String, usize) {
    let mut rng = rand::thread_rng();
    let i: usize = rng.gen_range(0, pop.len());
    let mut j: usize = rng.gen_range(0, pop.len());
    while i == j { j = rng.gen_range(0, pop.len()); }

    if pop[i].1 > pop[j].1 {
        return pop[i].clone();
    } else {
        return pop[j].clone();
    }
}

fn onemax(bitstring: &str) -> usize {
    bitstring.chars().fold(0_usize, |acc, i| if i == '1' {acc + 1} else {acc})
}

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
