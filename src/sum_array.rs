use std::intrinsics::prefetch_read_data;

pub fn sum_array_with_branch(threshold: i32, array: [i32; 1000]) -> i32 {
    let mut sum = 0;
    for item in array {
        if item < threshold {
            sum += item;
        }
    }
    sum
}

#[no_mangle]
pub fn sum_array_with_branch_prefetch(threshold: i32, array: &[i32; 1000]) -> i32 {
    let mut sum = 0;
    unsafe { prefetch_read_data(array.as_ptr(), 3) }
    for item in array {
        if *item < threshold {
            sum += item;
        }
    }
    sum
}

pub fn sum_array_with_bit_operator(threshold: i32, array: [i32; 1000]) -> i32 {
    let mut sum = 0;
    for item in array {
        sum += ((item - threshold) >> 31 - 1) & item;
    }
    sum
}

#[no_mangle]
pub fn sum_array_with_bit_operator_prefetch(threshold: i32, array: &[i32; 1000]) -> i32 {
    let mut sum = 0;
    unsafe { prefetch_read_data(array.as_ptr(), 3) }
    for item in array {
        sum += ((item - threshold) >> 31 - 1) & item;
    }
    sum
}

#[cfg(test)]
mod tests {

    use super::{sum_array_with_bit_operator, sum_array_with_branch};
    use crate::{
        rand_arry,
        sum_array::{sum_array_with_bit_operator_prefetch, sum_array_with_branch_prefetch},
    };

    #[test]
    fn test_sum_array() {
        let array = rand_arry();
        let threshold_array = [100, 200, 300, 400, 500, 600, 700, 800, 900];
        for threshold in threshold_array {
            assert_eq!(
                sum_array_with_branch(threshold, array),
                sum_array_with_bit_operator(threshold, array)
            );
            assert_eq!(
                sum_array_with_bit_operator_prefetch(threshold, &array),
                sum_array_with_branch_prefetch(threshold, &array)
            );
            assert_eq!(
                sum_array_with_bit_operator(threshold, array),
                sum_array_with_branch_prefetch(threshold, &array)
            )
        }
    }
}
