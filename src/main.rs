use rayon::prelude::*;
use serde::Serialize;
use std::time::Instant;

#[derive(Serialize)]
struct Primes {
    primes: Vec<u32>,
}

fn sieve(n: u32) -> Vec<u32> {
    let mut is_prime = vec![true; n as usize + 1];
    is_prime[0] = false;
    is_prime[1] = false;

    let mut p = 2;
    while p * p <= n {
        if is_prime[p as usize] {
            let mut i = p * p;
            while i <= n {
                is_prime[i as usize] = false;
                i += p;
            }
        }
        p += 1;
    }

    is_prime
        .into_iter()
        .enumerate()
        .filter(|(_, prime)| *prime)
        .map(|(i, _)| i as u32)
        .collect()
}

fn main() {
    let start_time = Instant::now();
    let end_day = 2500000;

    let primes = sieve(end_day);

    let chunk_size = 10000;
    let primes_chunks: Vec<_> = primes
        .par_chunks(chunk_size)
        .map(|chunk| Primes {
            primes: chunk.to_vec(),
        })
        .collect();

    primes_chunks
        .par_iter()
        .enumerate()
        .for_each(|(i, chunk)| {
            let filename = format!("primes_{}.json", i);
            serde_json::to_writer(
                std::fs::File::create(filename).unwrap(),
                chunk,
            )
            .unwrap();
        });

    let duration = start_time.elapsed();
    println!("Time elapsed: {:?}", duration);
}