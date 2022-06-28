#[allow(unused)]
fn low_bound(array: [i32; 100], x: i32) -> i32 {
    let mut l = 0;
    let mut r = array.len() - 1;
    while l < r {
        let m = (l + r) / 2;
        if array[m] < x {
            l = m + 1;
        } else {
            r = m;
        }
    }
    array[l]
}
