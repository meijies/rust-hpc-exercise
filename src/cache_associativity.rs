use std::intrinsics::prefetch_read_data;
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

/// 对一个二位数组设置值，可以有两种遍历方法，它们的性能不一样
/// https://zhuanlan.zhihu.com/p/102293437
pub fn init_array_1() {
    let mut a = [
        [0; 128], [0; 128], [0; 128], [0; 128], [0; 128], [0; 128], [0; 128], [0; 128], [0; 128],
        [0; 128],
    ];
    for i in 0..=9 {
        for k in 0..=127 {
            a[i][k] = 1;
        }
    }
}

pub fn init_array_2() {
    let mut a = [
        [0; 128], [0; 128], [0; 128], [0; 128], [0; 128], [0; 128], [0; 128], [0; 128], [0; 128],
        [0; 128],
    ];
    for k in 0..=127 {
        for i in 0..=9 {
            a[i][k] = 1;
        }
    }
}

#[no_mangle]
pub fn init_array_1_prefetch() {
    let mut a = [
        [0; 128], [0; 128], [0; 128], [0; 128], [0; 128], [0; 128], [0; 128], [0; 128], [0; 128],
        [0; 128],
    ];
    unsafe { prefetch_read_data(&a.as_ptr(), 3) }
    for i in 0..=9 {
        for k in 0..=127 {
            a[i][k] = 1;
        }
    }
}

#[no_mangle]
pub fn init_array_2_prefetch() {
    let mut a = [
        [0; 128], [0; 128], [0; 128], [0; 128], [0; 128], [0; 128], [0; 128], [0; 128], [0; 128],
        [0; 128],
    ];
    unsafe { prefetch_read_data(&a.as_ptr(), 3) }
    for k in 0..=127 {
        for i in 0..=9 {
            a[i][k] = 1;
        }
    }
}
