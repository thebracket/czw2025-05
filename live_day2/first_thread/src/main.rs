use rayon::prelude::*;

fn main() {
    let numbers: Vec<u64> = (0 .. 1_000_000).collect();
    let sum = numbers
        .par_iter()
        .sum::<u64>();
    println!("{sum}");
}