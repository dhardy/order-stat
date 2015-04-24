/// Calculate an approximate median of `array`.
///
/// The return value is the index/reference to some value of `array`
/// that is guaranteed to lie between the 30th and 70th percentiles of
/// the values in `array`.
///
/// # Panics
///
/// This panics if `array` is empty.
///
/// # Examples
///
/// ```rust
/// // the numbers 0, 1, ..., 100.
/// let mut x = (0..101).rev().collect::<Vec<_>>();
/// let (_, &mut median) = order_stat::median_of_medians(&mut x);
/// assert!(30 <= median);
/// assert!(median <= 70);
/// ```
pub fn median_of_medians<T: Ord>(array: &mut [T]) -> (usize, &mut T) {
    if array.len() < 5 {
        let median = array.len() / 2;
        return (median, super::kth(array, median))
    }
    let num_medians = array.len() / 5;
    for i in 0..num_medians {
        let start = 5 * i;
        let idx = median5(&array[start..start+5]);
        array.swap(i, start + idx);
    }
    let idx = num_medians / 2;
    (idx, super::kth(&mut array[..num_medians], idx))
}

fn median5<T: Ord>(array: &[T]) -> usize {
    use std::mem;

    let array = array;
    debug_assert!(array.len() == 5);

    let mut a4 = &array[4];
    let mut a3 = &array[3];
    let mut a2 = &array[2];
    let mut a1 = &array[1];
    let mut a0 = &array[0];

    macro_rules! cmp {
        ($($a: ident, $b: ident;)*) => {
            $(
                if $a < $b {
                    mem::swap(&mut $a, &mut $b)
                }
                )*
        }
    }

    cmp! {
        a1, a0;
        a2, a0;
        a3, a0;
        a4, a0;
        a2, a1;
        a3, a1;
        a4, a1;
        a3, a2;
        a4, a2;
    }

    if mem::size_of::<T>() == 0 {
        0
    } else {
        (a2 as *const _ as usize - array.as_ptr() as usize) / mem::size_of::<T>()
    }
}

#[cfg(test)]
mod tests {
    use std::cmp;
    use super::median_of_medians;
    use quickcheck::{self, TestResult};

    #[test]
    fn qc() {
        fn run(mut x: Vec<i32>) -> TestResult {
            if x.is_empty() { return TestResult::discard() }

            let (_, &mut median) = median_of_medians(&mut x);
            x.sort();

            let thirty = x.len() * 3 / 10;
            let seventy = cmp::min((x.len() * 7 + 9) / 10, x.len() - 1);
            TestResult::from_bool(x[thirty] <= median && median <= x[seventy])
        }
        quickcheck::quickcheck(run as fn(Vec<i32>) -> TestResult)
    }

    #[test]
    fn smoke() {
        let mut x = (0..101).rev().collect::<Vec<_>>();
        let (_, &mut median) = median_of_medians(&mut x);
        assert!(30 <= median);
        assert!(median <= 70);
    }
}

#[cfg(all(test, feature = "experimental"))]
mod benches {
    extern crate test;
    use rand::{XorShiftRng, Rng};
    use super::median_of_medians;

    const N: usize = 20_000;

    #[bench]
    fn huge(b: &mut test::Bencher) {
        let v = XorShiftRng::new_unseeded().gen_iter::<i32>().take(N).collect::<Vec<_>>();
        b.iter(|| {
            let mut w = v.clone();
            median_of_medians(&mut w).0
        });
    }

    #[bench]
    fn huge_exact(b: &mut test::Bencher) {
        let v = XorShiftRng::new_unseeded().gen_iter::<i32>().take(N).collect::<Vec<_>>();
        b.iter(|| {
            let mut w = v.clone();
            ::kth(&mut w, N / 2) as *mut _
        });
    }

}
