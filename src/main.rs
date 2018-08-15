#![allow(dead_code)]


extern crate rand;

mod genetic;

fn main() {
    let num_bits = 64_usize;
    let max_gens = 100_usize;
    let pop_size = 100_usize;
    let prob_cross = 0.98_f64;
    let prob_mutation = 1.0_f64 / num_bits as f64;

    let best = genetic::run(max_gens, pop_size, num_bits, prob_cross, prob_mutation);
    println!("Solution: {}", best);
}
