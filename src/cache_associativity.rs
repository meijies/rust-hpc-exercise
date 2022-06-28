use std::vec;

/// sum 256 比 sum 257 慢很多，但它们唯一的区别在于 step，原因和缓存相关.
/// 具体原因见 https://en.algorithmica.org/hpc/cpu-cache/associativity/

#[allow(warnings)]
pub fn sum_256_step<'a>(vec: &'a mut vec::Vec<u32>) {
    for x in (0..vec.len() - 1).step_by(256) {
        let v = *vec.get(x).unwrap();
        std::mem::replace(&mut vec[x], v + 1);
    }
}

#[allow(warnings)]
pub fn sum_257_step(vec: &mut vec::Vec<u32>) {
    for x in (1..vec.len() - 1).step_by(257) {
        let v = *vec.get(x).unwrap();
        std::mem::replace(&mut vec[x], v + 1);
    }
}
