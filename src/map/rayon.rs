//! Parallel iterator types for [`OrderMap`] with [`rayon`].
//!
//! You will rarely need to interact with this module directly unless you need to name one of the
//! iterator types.

pub use indexmap::map::rayon::{
    IntoParIter, ParDrain, ParIter, ParIterMut, ParKeys, ParValues, ParValuesMut,
};

use super::OrderMap;
use core::cmp::Ordering;
use core::hash::{BuildHasher, Hash};
use core::ops::RangeBounds;
use rayon::prelude::*;

impl<K, V, S> IntoParallelIterator for OrderMap<K, V, S>
where
    K: Send,
    V: Send,
{
    type Item = (K, V);
    type Iter = IntoParIter<K, V>;

    fn into_par_iter(self) -> Self::Iter {
        self.inner.into_par_iter()
    }
}

impl<'a, K, V, S> IntoParallelIterator for &'a OrderMap<K, V, S>
where
    K: Sync,
    V: Sync,
{
    type Item = (&'a K, &'a V);
    type Iter = ParIter<'a, K, V>;

    fn into_par_iter(self) -> Self::Iter {
        self.inner.par_iter()
    }
}

impl<'a, K, V, S> IntoParallelIterator for &'a mut OrderMap<K, V, S>
where
    K: Sync + Send,
    V: Send,
{
    type Item = (&'a K, &'a mut V);
    type Iter = ParIterMut<'a, K, V>;

    fn into_par_iter(self) -> Self::Iter {
        self.inner.par_iter_mut()
    }
}

impl<'a, K, V, S> ParallelDrainRange<usize> for &'a mut OrderMap<K, V, S>
where
    K: Send,
    V: Send,
{
    type Item = (K, V);
    type Iter = ParDrain<'a, K, V>;

    fn par_drain<R: RangeBounds<usize>>(self, range: R) -> Self::Iter {
        self.inner.par_drain(range)
    }
}

/// Parallel iterator methods and other parallel methods.
///
/// The following methods **require crate feature `"rayon"`**.
///
/// See also the `IntoParallelIterator` implementations.
impl<K, V, S> OrderMap<K, V, S>
where
    K: Sync,
    V: Sync,
{
    /// Return a parallel iterator over the keys of the map.
    ///
    /// While parallel iterators can process items in any order, their relative order
    /// in the map is still preserved for operations like `reduce` and `collect`.
    pub fn par_keys(&self) -> ParKeys<'_, K, V> {
        self.inner.par_keys()
    }

    /// Return a parallel iterator over the values of the map.
    ///
    /// While parallel iterators can process items in any order, their relative order
    /// in the map is still preserved for operations like `reduce` and `collect`.
    pub fn par_values(&self) -> ParValues<'_, K, V> {
        self.inner.par_values()
    }
}

impl<K, V, S> OrderMap<K, V, S>
where
    K: PartialEq + Sync,
    V: Sync,
{
    /// Returns `true` if `self` contains all of the same key-value pairs as `other`,
    /// in the same indexed order, determined in parallel.
    pub fn par_eq<S2>(&self, other: &OrderMap<K, V, S2>) -> bool
    where
        V: PartialEq,
    {
        self.len() == other.len() && self.par_iter().eq(other)
    }
}

impl<K, V, S> OrderMap<K, V, S>
where
    K: Send,
    V: Send,
{
    /// Return a parallel iterator over mutable references to the values of the map
    ///
    /// While parallel iterators can process items in any order, their relative order
    /// in the map is still preserved for operations like `reduce` and `collect`.
    pub fn par_values_mut(&mut self) -> ParValuesMut<'_, K, V> {
        self.inner.par_values_mut()
    }
}

impl<K, V, S> OrderMap<K, V, S>
where
    K: Send,
    V: Send,
{
    /// Sort the map’s key-value pairs in parallel, by the default ordering of the keys.
    pub fn par_sort_keys(&mut self)
    where
        K: Ord,
    {
        self.inner.par_sort_keys();
    }

    /// Sort the map’s key-value pairs in place and in parallel, using the comparison
    /// function `cmp`.
    ///
    /// The comparison function receives two key and value pairs to compare (you
    /// can sort by keys or values or their combination as needed).
    pub fn par_sort_by<F>(&mut self, cmp: F)
    where
        F: Fn(&K, &V, &K, &V) -> Ordering + Sync,
    {
        self.inner.par_sort_by(cmp);
    }

    /// Sort the key-value pairs of the map in parallel and return a by-value parallel
    /// iterator of the key-value pairs with the result.
    pub fn par_sorted_by<F>(self, cmp: F) -> IntoParIter<K, V>
    where
        F: Fn(&K, &V, &K, &V) -> Ordering + Sync,
    {
        self.inner.par_sorted_by(cmp)
    }

    /// Sort the map's key-value pairs in parallel, by the default ordering of the keys.
    pub fn par_sort_unstable_keys(&mut self)
    where
        K: Ord,
    {
        self.inner.par_sort_unstable_keys();
    }

    /// Sort the map's key-value pairs in place and in parallel, using the comparison
    /// function `cmp`.
    ///
    /// The comparison function receives two key and value pairs to compare (you
    /// can sort by keys or values or their combination as needed).
    pub fn par_sort_unstable_by<F>(&mut self, cmp: F)
    where
        F: Fn(&K, &V, &K, &V) -> Ordering + Sync,
    {
        self.inner.par_sort_unstable_by(cmp);
    }

    /// Sort the key-value pairs of the map in parallel and return a by-value parallel
    /// iterator of the key-value pairs with the result.
    pub fn par_sorted_unstable_by<F>(self, cmp: F) -> IntoParIter<K, V>
    where
        F: Fn(&K, &V, &K, &V) -> Ordering + Sync,
    {
        self.inner.par_sorted_unstable_by(cmp)
    }

    /// Sort the map’s key-value pairs in place and in parallel, using a sort-key extraction
    /// function.
    pub fn par_sort_by_cached_key<T, F>(&mut self, sort_key: F)
    where
        T: Ord + Send,
        F: Fn(&K, &V) -> T + Sync,
    {
        self.inner.par_sort_by_cached_key(sort_key)
    }
}

impl<K, V, S> FromParallelIterator<(K, V)> for OrderMap<K, V, S>
where
    K: Eq + Hash + Send,
    V: Send,
    S: BuildHasher + Default + Send,
{
    fn from_par_iter<I>(iter: I) -> Self
    where
        I: IntoParallelIterator<Item = (K, V)>,
    {
        Self {
            inner: <_>::from_par_iter(iter),
        }
    }
}

impl<K, V, S> ParallelExtend<(K, V)> for OrderMap<K, V, S>
where
    K: Eq + Hash + Send,
    V: Send,
    S: BuildHasher + Send,
{
    fn par_extend<I>(&mut self, iter: I)
    where
        I: IntoParallelIterator<Item = (K, V)>,
    {
        self.inner.par_extend(iter);
    }
}

impl<'a, K: 'a, V: 'a, S> ParallelExtend<(&'a K, &'a V)> for OrderMap<K, V, S>
where
    K: Copy + Eq + Hash + Send + Sync,
    V: Copy + Send + Sync,
    S: BuildHasher + Send,
{
    fn par_extend<I>(&mut self, iter: I)
    where
        I: IntoParallelIterator<Item = (&'a K, &'a V)>,
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
        let mut map = OrderMap::new();

        for &elt in &insert {
            map.insert(elt, ());
        }

        assert_eq!(map.par_keys().count(), map.len());
        assert_eq!(map.par_keys().count(), insert.len());
        insert.par_iter().zip(map.par_keys()).for_each(|(a, b)| {
            assert_eq!(a, b);
        });
        (0..insert.len())
            .into_par_iter()
            .zip(map.par_keys())
            .for_each(|(i, k)| {
                assert_eq!(map.get_index(i).unwrap().0, k);
            });
    }

    #[test]
    fn partial_eq_and_eq() {
        let mut map_a = OrderMap::new();
        map_a.insert(1, "1");
        map_a.insert(2, "2");
        let mut map_b = map_a.clone();
        assert!(map_a.par_eq(&map_b));
        map_b.swap_remove(&1);
        assert!(!map_a.par_eq(&map_b));
        map_b.insert(3, "3");
        assert!(!map_a.par_eq(&map_b));
    }

    #[test]
    fn extend() {
        let mut map = OrderMap::new();
        map.par_extend(vec![(&1, &2), (&3, &4)]);
        map.par_extend(vec![(5, 6)]);
        assert_eq!(
            map.into_par_iter().collect::<Vec<_>>(),
            vec![(1, 2), (3, 4), (5, 6)]
        );
    }

    #[test]
    fn keys() {
        let vec = vec![(1, 'a'), (2, 'b'), (3, 'c')];
        let map: OrderMap<_, _> = vec.into_par_iter().collect();
        let keys: Vec<_> = map.par_keys().copied().collect();
        assert_eq!(keys.len(), 3);
        assert!(keys.contains(&1));
        assert!(keys.contains(&2));
        assert!(keys.contains(&3));
    }

    #[test]
    fn values() {
        let vec = vec![(1, 'a'), (2, 'b'), (3, 'c')];
        let map: OrderMap<_, _> = vec.into_par_iter().collect();
        let values: Vec<_> = map.par_values().copied().collect();
        assert_eq!(values.len(), 3);
        assert!(values.contains(&'a'));
        assert!(values.contains(&'b'));
        assert!(values.contains(&'c'));
    }

    #[test]
    fn values_mut() {
        let vec = vec![(1, 1), (2, 2), (3, 3)];
        let mut map: OrderMap<_, _> = vec.into_par_iter().collect();
        map.par_values_mut().for_each(|value| *value *= 2);
        let values: Vec<_> = map.par_values().copied().collect();
        assert_eq!(values.len(), 3);
        assert!(values.contains(&2));
        assert!(values.contains(&4));
        assert!(values.contains(&6));
    }
}
