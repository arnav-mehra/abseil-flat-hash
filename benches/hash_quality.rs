/*
REAL COOL TEST BENCH FOR HASH QUALITY.

IDEA:
    For a quality hash function,
    unique-valued keys, even when sampled from biased bit distributions,
    should be hashed to something non-biased.

INPUTS:
    1. BIT_DISTS:   Individually adjustable bit probabilities. index -> 2^index bit.
    2. N:           The ideal number of randomly generated unique inputs.
    3. HASHES:      Hash functions being tested.
    
    NOTE: Random sampling CAN NOT be done efficiently if mag(N) ~ mag(Sample Space)
    due to high P(sample repetition) slowing down compute times. Sample space
    enumeration would be possible, but a waste of time when N is already large.
    
    GUIDE:
        N ~ 1_000_000 * Var / (0.5 * 0.5 * 64).
        Where: Var = SUM_{i=1:64} p[i] * (1 - p[i]).

OUTPUTS:
    1. m:                   The number of unique samples generated.
    2. collision penalty:   Average number of collisions squared per sample.
    3. bit rate penalty:    Sum of (bit occurrance rate - 0.5, ideal rate) squares.
*/

use std::hash::Hasher;
use std::collections::hash_map::*;
use fxhash::FxHasher64;

use rand::prelude::Distribution;
use rand::distributions::Bernoulli;
use hash_histogram::HashHistogram;
use itertools::Itertools;

use criterion::*;

const N : usize = 500_000;
const BIT_DISTS : [[f64; 64]; 7] = [
    [0.1; 64],
    [0.3; 64],
    [0.4; 64],
    [0.5; 64],
    [0.6; 64],
    [0.7; 64],
    [0.9; 64],
];
const HASHES : [(&str, fn(u64) -> u64); 3] = [
    ("StdHash", std_hash),
    ("FxHash", fx_hash),
    ("CtrlHash", |x : u64| x >> 32)
];

fn fx_hash(x : u64) -> u64 {
    let mut hasher: FxHasher64 = FxHasher64::default();
    hasher.write_u64(x);
    hasher.finish()
}

fn std_hash(x : u64) -> u64 {
    let mut hasher: DefaultHasher = DefaultHasher::new();
    hasher.write_u64(x);
    hasher.finish()
}

// generate samples based on individual bit probabilities.
fn get_rands(bit_dist: &[f64; 64]) -> Vec<u64> {
    let mut res: Vec<u64> = vec![0; N];
    for i in 0..64 {
        let p_of_one : f64 = bit_dist[i];
        let sample: Vec<bool> = Bernoulli::new(p_of_one)
            .unwrap()
            .sample_iter(rand::thread_rng())
            .take(N)
            .collect();
        for j in 0..N {
            res[j] ^= (sample[j] as u64) << i;
        }
    }
    res.into_iter().unique().collect()
}

fn get_collision_avg_sq_sum(hist: &HashHistogram<u64>, m: usize) -> f32 {
    let mut coll_sq_sum: u64 = 0;
    for (_, &cnt) in hist.iter() {
        coll_sq_sum += ((cnt - 1) * (cnt - 1)) as u64;
    }
    (coll_sq_sum as f32) / (m as f32)
}

fn get_bit_rate_sq_sum(hist: &HashHistogram<u64>, m: usize) -> f32 {
    // build bit histogram.
    let mut bit_hist : [u64; 64] = [0; 64];
    for (&hash, _) in hist.iter() {
        for i in 0..64 {
            bit_hist[i] += (hash >> i) & 1;
        }
    }
    // add bit probability bias squares.
    let mut bit_rate_sq_sum : f32 = 0.0;
    for i in 0..64 {
        let delta : f32 = (bit_hist[i] as f32) / (m as f32) - 0.5;
        bit_rate_sq_sum += delta * delta;
    }
    bit_rate_sq_sum
}

fn criterion_benchmark(_ : &mut Criterion) {
    for (hash_name, hash_fn) in HASHES {
        println!("\nHash: {}", hash_name);

        for bit_dist in BIT_DISTS {
            let rands : Vec<u64> = get_rands(&bit_dist);
            let m : usize = rands.len();

            // place hash values into histogram.
            let hist : HashHistogram<u64> = rands
                .iter()
                .map(|&x| hash_fn(x))
                .collect();

            let collision_avg_sq_sum = get_collision_avg_sq_sum(&hist, m);
            let bit_rate_sq_sum = get_bit_rate_sq_sum(&hist, m);

            // println!("samples generated: {}", m);
            println!("collision penalty: {}", collision_avg_sq_sum);
            println!(" bit rate penalty: {}", bit_rate_sq_sum);
        }
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);