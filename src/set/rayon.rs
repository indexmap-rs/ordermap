//! Parallel iterator types for [`OrderSet`] with [rayon].
//!
//! You will rarely need to interact with this module directly unless you need to name one of the
//! iterator types.

pub use indexmap::set::rayon::{
    IntoParIter, ParDifference, ParDrain, ParIntersection, ParIter, ParSymmetricDifference,
    ParUnion,
};

use super::OrderSet;
use core::cmp::Ordering;
use core::hash::{BuildHasher, Hash};
use core::ops::RangeBounds;
use rayon::prelude::*;

impl<T, S> IntoParallelIterator for OrderSet<T, S>
where
    T: Send,
{
    type Item = T;
    type Iter = IntoParIter<T>;

    fn into_par_iter(self) -> Self::Iter {
        self.inner.into_par_iter()
    }
}

impl<'a, T, S> IntoParallelIterator for &'a OrderSet<T, S>
where
    T: Sync,
{
    type Item = &'a T;
    type Iter = ParIter<'a, T>;

    fn into_par_iter(self) -> Self::Iter {
        self.inner.par_iter()
    }
}

impl<'a, T, S> ParallelDrainRange<usize> for &'a mut OrderSet<T, S>
where
    T: Send,
{
    type Item = T;
    type Iter = ParDrain<'a, T>;

    fn par_drain<R: RangeBounds<usize>>(self, range: R) -> Self::Iter {
        self.inner.par_drain(range)
    }
}

impl<T, S> OrderSet<T, S>
where
    T: PartialEq + Sync,
{
    /// Returns `true` if `self` contains all of the same values as `other`,
    /// in the same indexed order, determined in parallel.
    pub fn par_eq<S2>(&self, other: &OrderSet<T, S2>) -> bool
    where
        S2: BuildHasher + Sync,
    {
        self.len() == other.len() && self.par_iter().eq(other)
    }
}

/// Parallel iterator methods and other parallel methods.
///
/// The following methods **require crate feature `"rayon"`**.
///
/// See also the `IntoParallelIterator` implementations.
impl<T, S> OrderSet<T, S>
where
    T: Hash + Eq + Sync,
    S: BuildHasher + Sync,
{
    /// Return a parallel iterator over the values that are in `self` but not `other`.
    ///
    /// While parallel iterators can process items in any order, their relative order
    /// in the `self` set is still preserved for operations like `reduce` and `collect`.
    pub fn par_difference<'a, S2>(
        &'a self,
        other: &'a OrderSet<T, S2>,
    ) -> ParDifference<'a, T, S, S2>
    where
        S2: BuildHasher + Sync,
    {
        self.inner.par_difference(&other.inner)
    }

    /// Return a parallel iterator over the values that are in `self` or `other`,
    /// but not in both.
    ///
    /// While parallel iterators can process items in any order, their relative order
    /// in the sets is still preserved for operations like `reduce` and `collect`.
    /// Values from `self` are produced in their original order, followed by
    /// values from `other` in their original order.
    pub fn par_symmetric_difference<'a, S2>(
        &'a self,
        other: &'a OrderSet<T, S2>,
    ) -> ParSymmetricDifference<'a, T, S, S2>
    where
        S2: BuildHasher + Sync,
    {
        self.inner.par_symmetric_difference(&other.inner)
    }

    /// Return a parallel iterator over the values that are in both `self` and `other`.
    ///
    /// While parallel iterators can process items in any order, their relative order
    /// in the `self` set is still preserved for operations like `reduce` and `collect`.
    pub fn par_intersection<'a, S2>(
        &'a self,
        other: &'a OrderSet<T, S2>,
    ) -> ParIntersection<'a, T, S, S2>
    where
        S2: BuildHasher + Sync,
    {
        self.inner.par_intersection(&other.inner)
    }

    /// Return a parallel iterator over all values that are in `self` or `other`.
    ///
    /// While parallel iterators can process items in any order, their relative order
    /// in the sets is still preserved for operations like `reduce` and `collect`.
    /// Values from `self` are produced in their original order, followed by
    /// values that are unique to `other` in their original order.
    pub fn par_union<'a, S2>(&'a self, other: &'a OrderSet<T, S2>) -> ParUnion<'a, T, S, S2>
    where
        S2: BuildHasher + Sync,
    {
        self.inner.par_union(&other.inner)
    }

    /// Returns `true` if `self` has no elements in common with `other`,
    /// determined in parallel.
    pub fn par_is_disjoint<S2>(&self, other: &OrderSet<T, S2>) -> bool
    where
        S2: BuildHasher + Sync,
    {
        self.inner.par_is_disjoint(&other.inner)
    }

    /// Returns `true` if all elements of `other` are contained in `self`,
    /// determined in parallel.
    pub fn par_is_superset<S2>(&self, other: &OrderSet<T, S2>) -> bool
    where
        S2: BuildHasher + Sync,
    {
        self.inner.par_is_superset(&other.inner)
    }

    /// Returns `true` if all elements of `self` are contained in `other`,
    /// determined in parallel.
    pub fn par_is_subset<S2>(&self, other: &OrderSet<T, S2>) -> bool
    where
        S2: BuildHasher + Sync,
    {
        self.inner.par_is_subset(&other.inner)
    }

    /// Returns `true` if `self` contains all of the same values as `other`,
    /// regardless of each set's indexed order, determined in parallel.
    pub fn par_set_eq<S2>(&self, other: &OrderSet<T, S2>) -> bool
    where
        S2: BuildHasher + Sync,
    {
        self.inner.par_eq(&other.inner)
    }
}

/// Parallel sorting methods.
///
/// The following methods **require crate feature `"rayon"`**.
impl<T, S> OrderSet<T, S>
where
    T: Send,
{
    /// Sort the set’s values in parallel by their default ordering.
    pub fn par_sort(&mut self)
    where
        T: Ord,
    {
        self.inner.par_sort();
    }

    /// Sort the set’s values in place and in parallel, using the comparison function `cmp`.
    pub fn par_sort_by<F>(&mut self, cmp: F)
    where
        F: Fn(&T, &T) -> Ordering + Sync,
    {
        self.inner.par_sort_by(cmp);
    }

    /// Sort the values of the set in parallel and return a by-value parallel iterator of
    /// the values with the result.
    pub fn par_sorted_by<F>(self, cmp: F) -> IntoParIter<T>
    where
        F: Fn(&T, &T) -> Ordering + Sync,
    {
        self.inner.par_sorted_by(cmp)
    }

    /// Sort the set's values in parallel by their default ordering.
    pub fn par_sort_unstable(&mut self)
    where
        T: Ord,
    {
        self.inner.par_sort_unstable();
    }

    /// Sort the set’s values in place and in parallel, using the comparison function `cmp`.
    pub fn par_sort_unstable_by<F>(&mut self, cmp: F)
    where
        F: Fn(&T, &T) -> Ordering + Sync,
    {
        self.inner.par_sort_unstable_by(cmp);
    }

    /// Sort the values of the set in parallel and return a by-value parallel iterator of
    /// the values with the result.
    pub fn par_sorted_unstable_by<F>(self, cmp: F) -> IntoParIter<T>
    where
        F: Fn(&T, &T) -> Ordering + Sync,
    {
        self.inner.par_sorted_unstable_by(cmp)
    }

    /// Sort the set’s values in place and in parallel, using a key extraction function.
    pub fn par_sort_by_cached_key<K, F>(&mut self, sort_key: F)
    where
        K: Ord + Send,
        F: Fn(&T) -> K + Sync,
    {
        self.inner.par_sort_by_cached_key(sort_key);
    }
}

impl<T, S> FromParallelIterator<T> for OrderSet<T, S>
where
    T: Eq + Hash + Send,
    S: BuildHasher + Default + Send,
{
    fn from_par_iter<I>(iter: I) -> Self
    where
        I: IntoParallelIterator<Item = T>,
    {
        Self {
            inner: <_>::from_par_iter(iter),
        }
    }
}

impl<T, S> ParallelExtend<T> for OrderSet<T, S>
where
    T: Eq + Hash + Send,
    S: BuildHasher + Send,
{
    fn par_extend<I>(&mut self, iter: I)
    where
        I: IntoParallelIterator<Item = T>,
    {
        self.inner.par_extend(iter);
    }
}

impl<'a, T: 'a, S> ParallelExtend<&'a T> for OrderSet<T, S>
where
    T: Copy + Eq + Hash + Send + Sync,
    S: BuildHasher + Send,
{
    fn par_extend<I>(&mut self, iter: I)
    where
        I: IntoParallelIterator<Item = &'a T>,
    {
        self.inner.par_extend(iter);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::vec::Vec;

    #[test]
    fn insert_order() {
        let insert = [0, 4, 2, 12, 8, 7, 11, 5, 3, 17, 19, 22, 23];
        let mut set = OrderSet::new();

        for &elt in &insert {
            set.insert(elt);
        }

        assert_eq!(set.par_iter().count(), set.len());
        assert_eq!(set.par_iter().count(), insert.len());
        insert.par_iter().zip(&set).for_each(|(a, b)| {
            assert_eq!(a, b);
        });
        (0..insert.len())
            .into_par_iter()
            .zip(&set)
            .for_each(|(i, v)| {
                assert_eq!(set.get_index(i).unwrap(), v);
            });
    }

    #[test]
    fn partial_eq_and_eq() {
        let mut set_a = OrderSet::new();
        set_a.insert(1);
        set_a.insert(2);
        let mut set_b = set_a.clone();
        assert!(set_a.par_eq(&set_b));
        set_b.swap_remove(&1);
        assert!(!set_a.par_eq(&set_b));
        set_b.insert(3);
        assert!(!set_a.par_eq(&set_b));

        let set_c: OrderSet<_> = set_b.into_par_iter().collect();
        assert!(!set_a.par_eq(&set_c));
        assert!(!set_c.par_eq(&set_a));
    }

    #[test]
    fn extend() {
        let mut set = OrderSet::new();
        set.par_extend(vec![&1, &2, &3, &4]);
        set.par_extend(vec![5, 6]);
        assert_eq!(
            set.into_par_iter().collect::<Vec<_>>(),
            vec![1, 2, 3, 4, 5, 6]
        );
    }

    #[test]
    fn comparisons() {
        let set_a: OrderSet<_> = (0..3).collect();
        let set_b: OrderSet<_> = (3..6).collect();
        let set_c: OrderSet<_> = (0..6).collect();
        let set_d: OrderSet<_> = (3..9).collect();

        assert!(!set_a.par_is_disjoint(&set_a));
        assert!(set_a.par_is_subset(&set_a));
        assert!(set_a.par_is_superset(&set_a));

        assert!(set_a.par_is_disjoint(&set_b));
        assert!(set_b.par_is_disjoint(&set_a));
        assert!(!set_a.par_is_subset(&set_b));
        assert!(!set_b.par_is_subset(&set_a));
        assert!(!set_a.par_is_superset(&set_b));
        assert!(!set_b.par_is_superset(&set_a));

        assert!(!set_a.par_is_disjoint(&set_c));
        assert!(!set_c.par_is_disjoint(&set_a));
        assert!(set_a.par_is_subset(&set_c));
        assert!(!set_c.par_is_subset(&set_a));
        assert!(!set_a.par_is_superset(&set_c));
        assert!(set_c.par_is_superset(&set_a));

        assert!(!set_c.par_is_disjoint(&set_d));
        assert!(!set_d.par_is_disjoint(&set_c));
        assert!(!set_c.par_is_subset(&set_d));
        assert!(!set_d.par_is_subset(&set_c));
        assert!(!set_c.par_is_superset(&set_d));
        assert!(!set_d.par_is_superset(&set_c));
    }

    #[test]
    fn iter_comparisons() {
        use std::iter::empty;

        fn check<'a, I1, I2>(iter1: I1, iter2: I2)
        where
            I1: ParallelIterator<Item = &'a i32>,
            I2: Iterator<Item = i32>,
        {
            let v1: Vec<_> = iter1.copied().collect();
            let v2: Vec<_> = iter2.collect();
            assert_eq!(v1, v2);
        }

        let set_a: OrderSet<_> = (0..3).collect();
        let set_b: OrderSet<_> = (3..6).collect();
        let set_c: OrderSet<_> = (0..6).collect();
        let set_d: OrderSet<_> = (3..9).rev().collect();

        check(set_a.par_difference(&set_a), empty());
        check(set_a.par_symmetric_difference(&set_a), empty());
        check(set_a.par_intersection(&set_a), 0..3);
        check(set_a.par_union(&set_a), 0..3);

        check(set_a.par_difference(&set_b), 0..3);
        check(set_b.par_difference(&set_a), 3..6);
        check(set_a.par_symmetric_difference(&set_b), 0..6);
        check(set_b.par_symmetric_difference(&set_a), (3..6).chain(0..3));
        check(set_a.par_intersection(&set_b), empty());
        check(set_b.par_intersection(&set_a), empty());
        check(set_a.par_union(&set_b), 0..6);
        check(set_b.par_union(&set_a), (3..6).chain(0..3));

        check(set_a.par_difference(&set_c), empty());
        check(set_c.par_difference(&set_a), 3..6);
        check(set_a.par_symmetric_difference(&set_c), 3..6);
        check(set_c.par_symmetric_difference(&set_a), 3..6);
        check(set_a.par_intersection(&set_c), 0..3);
        check(set_c.par_intersection(&set_a), 0..3);
        check(set_a.par_union(&set_c), 0..6);
        check(set_c.par_union(&set_a), 0..6);

        check(set_c.par_difference(&set_d), 0..3);
        check(set_d.par_difference(&set_c), (6..9).rev());
        check(
            set_c.par_symmetric_difference(&set_d),
            (0..3).chain((6..9).rev()),
        );
        check(
            set_d.par_symmetric_difference(&set_c),
            (6..9).rev().chain(0..3),
        );
        check(set_c.par_intersection(&set_d), 3..6);
        check(set_d.par_intersection(&set_c), (3..6).rev());
        check(set_c.par_union(&set_d), (0..6).chain((6..9).rev()));
        check(set_d.par_union(&set_c), (3..9).rev().chain(0..3));
    }
}
