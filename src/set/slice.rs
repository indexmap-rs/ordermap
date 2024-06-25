use super::OrderSet;
use core::ops::{self, Bound, Index};
use indexmap::set::Slice;

// We can't have `impl<I: RangeBounds<usize>> Index<I>` because that conflicts with `Index<usize>`.
// Instead, we repeat the implementations for all the core range types.
macro_rules! impl_index {
    ($($range:ty),*) => {$(
        impl<T, S> Index<$range> for OrderSet<T, S> {
            type Output = Slice<T>;

            fn index(&self, range: $range) -> &Self::Output {
                &self.inner[range]
            }
        }
    )*}
}
impl_index!(
    ops::Range<usize>,
    ops::RangeFrom<usize>,
    ops::RangeFull,
    ops::RangeInclusive<usize>,
    ops::RangeTo<usize>,
    ops::RangeToInclusive<usize>,
    (Bound<usize>, Bound<usize>)
);

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec::Vec;

    #[test]
    fn slice_index() {
        fn check(vec_slice: &[i32], set_slice: &Slice<i32>, sub_slice: &Slice<i32>) {
            assert_eq!(set_slice as *const _, sub_slice as *const _);
            itertools::assert_equal(vec_slice, set_slice);
        }

        let vec: Vec<i32> = (0..10).map(|i| i * i).collect();
        let set: OrderSet<i32> = vec.iter().cloned().collect();
        let slice = set.as_slice();

        // RangeFull
        check(&vec[..], &set[..], &slice[..]);

        for i in 0usize..10 {
            // Index
            assert_eq!(vec[i], set[i]);
            assert_eq!(vec[i], slice[i]);

            // RangeFrom
            check(&vec[i..], &set[i..], &slice[i..]);

            // RangeTo
            check(&vec[..i], &set[..i], &slice[..i]);

            // RangeToInclusive
            check(&vec[..=i], &set[..=i], &slice[..=i]);

            // (Bound<usize>, Bound<usize>)
            let bounds = (Bound::Excluded(i), Bound::Unbounded);
            check(&vec[i + 1..], &set[bounds], &slice[bounds]);

            for j in i..=10 {
                // Range
                check(&vec[i..j], &set[i..j], &slice[i..j]);
            }

            for j in i..10 {
                // RangeInclusive
                check(&vec[i..=j], &set[i..=j], &slice[i..=j]);
            }
        }
    }
}
