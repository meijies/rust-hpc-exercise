#![feature(core_intrinsics)]
use std::vec;

use rand::prelude::*;
pub mod binary_search;
pub mod cache_associativity;
pub mod sum_array;

pub fn rand_arry() -> [i32; 1000] {
    let mut a: [i32; 1000] = [0; 1000];
    let mut rng = thread_rng();
    for i in 0..999 {
        a[i] = rng.gen_range(0..1000);
    }
    a
}

pub fn reand_vec(n: u32) -> vec::Vec<u32> {
    let mut vec = Vec::new();
    let mut rng = thread_rng();
    for _ in 0..n {
        vec.push(rng.gen_range(0..n));
    }
    vec
}
