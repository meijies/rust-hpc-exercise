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

pub fn sum_array_with_bit_operator(threshold: u32, array: [u32; 1000]) -> u32 {
    let mut sum = 0;
    for item in array {
        sum += (item - threshold >> 31 - 1) & item;
    }
    sum
}

#[cfg(test)]
mod tests {

    use super::{rand_arry, sum_array_with_bit_operator, sum_array_with_branch};

    #[test]
    fn test_sum_array() {
        let array = rand_arry();
        let threshold_array = [100, 200, 300, 400, 500, 600, 700, 800, 900];
        for threshold in threshold_array {
            assert_eq!(
                sum_array_with_branch(threshold, array),
                sum_array_with_bit_operator(threshold, array)
            );
        }
    }
}
