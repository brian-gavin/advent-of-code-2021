use std::io::{self, prelude::*};

pub fn solve() -> String {
    const DAYS: usize = 256;
    let initial: Vec<u64> = {
        let mut s = String::new();
        let stdin = io::stdin();
        stdin.lock().read_to_string(&mut s).unwrap();
        s.trim().split(',').filter_map(|n| n.parse().ok()).collect()
    };
    let buckets = {
        let mut b = [0; 9];
        initial.into_iter().for_each(|n| {
            b[n as usize] += 1;
        });
        b
    };
    let n = fishes(buckets, DAYS);
    format!("after {} days, {} fishies", DAYS, n)
}

pub fn fishes_slow(mut fishes: Vec<u64>, days: usize) -> Vec<u64> {
    for _d in 1..=days {
        let mut new = Vec::new();
        for fish in fishes.iter_mut() {
            if *fish == 0 {
                *fish = 6;
                new.push(8);
            } else {
                *fish -= 1;
            }
        }
        fishes.append(&mut new);
    }
    fishes
}

pub fn fishes(mut fishes: [u64; 9], days: usize) -> u64 {
    for _ in 0..days {
        fishes.rotate_left(1);
        fishes[6] += fishes[8];
    }
    fishes.into_iter().sum()
}
