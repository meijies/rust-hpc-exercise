use rand::prelude::*;

pub fn rand_arry() -> [u32; 1000] {
    let mut a: [u32; 1000] = [0; 1000];
    let mut rng = thread_rng();
    for i in 0..999 {
        a[i] = rng.gen_range(0..1000);
    }
    a
}

pub fn sum_array_with_branch(threshold: u32, array: [u32; 1000]) -> u32 {
    let mut sum = 0;
    for item in array {
        if item < threshold {
            sum += item;
        }
    }
    sum
}
