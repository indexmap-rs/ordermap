//! [`OrderMap`] is a hash table where the iteration order of the key-value
//! pairs is independent of the hash values of the keys.
//!
//! It is based on [`IndexMap`], and even shares many of the auxiliary types
//! like [`Slice`] and all of the iterators.
//!
//! **Unlike** `IndexMap`, `OrderMap` does consider the order for [`PartialEq`]
//! and [`Eq`], and it also implements [`PartialOrd`], [`Ord`], and [`Hash`].
//! Methods like [`OrderMap::remove`] use `IndexMap`'s "shift" semantics, so
//! they preserve the relative order of remaining entries.

mod entry;
mod iter;
mod mutable;
mod slice;

pub mod raw_entry_v1;

#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
pub mod serde_seq;

#[cfg(test)]
mod tests;

pub use self::entry::{Entry, IndexedEntry, OccupiedEntry, VacantEntry};
pub use self::mutable::MutableEntryKey;
pub use self::mutable::MutableKeys;
pub use self::raw_entry_v1::RawEntryApiV1;
pub use indexmap::map::{
    Drain, IntoIter, IntoKeys, IntoValues, Iter, IterMut, IterMut2, Keys, Slice, Splice, Values,
    ValuesMut,
};

#[cfg(feature = "rayon")]
#[cfg_attr(docsrs, doc(cfg(feature = "rayon")))]
pub mod rayon;

use alloc::boxed::Box;
use core::cmp::Ordering;
use core::fmt;
use core::hash::{BuildHasher, Hash, Hasher};
use core::ops::{Index, IndexMut, RangeBounds};
use indexmap::IndexMap;

#[cfg(doc)]
use alloc::vec::Vec;

#[cfg(feature = "std")]
use std::collections::hash_map::RandomState;

use crate::{Equivalent, TryReserveError};

/// A hash table where the iteration order of the key-value pairs is independent
/// of the hash values of the keys.
///
/// The interface is closely compatible with the standard
/// [`HashMap`][std::collections::HashMap],
/// but also has additional features.
///
/// # Order
///
/// The key-value pairs have a consistent order that is determined by
/// the sequence of insertion and removal calls on the map. The order does
/// not depend on the keys or the hash function at all.
///
/// All iterators traverse the map in *the order*.
///
/// The insertion order is preserved, with **notable exceptions** like the
/// [`.swap_remove()`][Self::swap_remove] method.
/// Methods such as [`.sort_by()`][Self::sort_by] of
/// course result in a new order, depending on the sorting order.
///
/// # Indices
///
/// The key-value pairs are indexed in a compact range without holes in the
/// range `0..self.len()`. For example, the method `.get_full` looks up the
/// index for a key, and the method `.get_index` looks up the key-value pair by
/// index.
///
/// # Complexity
///
/// Internally, `OrderMap<K, V, S>` just holds an [`IndexMap<K, V, S>`](IndexMap).
/// Thus the complexity of the two are the same for most methods.
///
/// # Examples
///
/// ```
/// use ordermap::OrderMap;
///
/// // count the frequency of each letter in a sentence.
/// let mut letters = OrderMap::new();
/// for ch in "a short treatise on fungi".chars() {
///     *letters.entry(ch).or_insert(0) += 1;
/// }
///
/// assert_eq!(letters[&'s'], 2);
/// assert_eq!(letters[&'t'], 3);
/// assert_eq!(letters[&'u'], 1);
/// assert_eq!(letters.get(&'y'), None);
/// ```
#[cfg(feature = "std")]
pub struct OrderMap<K, V, S = RandomState> {
    pub(crate) inner: IndexMap<K, V, S>,
}
#[cfg(not(feature = "std"))]
pub struct OrderMap<K, V, S> {
    pub(crate) inner: IndexMap<K, V, S>,
}

impl<K, V, S> Clone for OrderMap<K, V, S>
where
    K: Clone,
    V: Clone,
    S: Clone,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }

    fn clone_from(&mut self, other: &Self) {
        self.inner.clone_from(&other.inner);
    }
}

impl<K, V, S> fmt::Debug for OrderMap<K, V, S>
where
    K: fmt::Debug,
    V: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl<K, V> OrderMap<K, V> {
    /// Create a new map. (Does not allocate.)
    #[inline]
    pub fn new() -> Self {
        Self {
            inner: IndexMap::new(),
        }
    }

    /// Create a new map with capacity for `n` key-value pairs. (Does not
    /// allocate if `n` is zero.)
    ///
    /// Computes in **O(n)** time.
    #[inline]
    pub fn with_capacity(n: usize) -> Self {
        Self {
            inner: IndexMap::with_capacity(n),
        }
    }
}

impl<K, V, S> OrderMap<K, V, S> {
    /// Create a new map with capacity for `n` key-value pairs. (Does not
    /// allocate if `n` is zero.)
    ///
    /// Computes in **O(n)** time.
    #[inline]
    pub fn with_capacity_and_hasher(n: usize, hash_builder: S) -> Self {
        Self {
            inner: IndexMap::with_capacity_and_hasher(n, hash_builder),
        }
    }

    /// Create a new map with `hash_builder`.
    ///
    /// This function is `const`, so it
    /// can be called in `static` contexts.
    pub const fn with_hasher(hash_builder: S) -> Self {
        Self {
            inner: IndexMap::with_hasher(hash_builder),
        }
    }

    /// Return the number of elements the map can hold without reallocating.
    ///
    /// This number is a lower bound; the map might be able to hold more,
    /// but is guaranteed to be able to hold at least this many.
    ///
    /// Computes in **O(1)** time.
    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    /// Return a reference to the map's `BuildHasher`.
    pub fn hasher(&self) -> &S {
        self.inner.hasher()
    }

    /// Return the number of key-value pairs in the map.
    ///
    /// Computes in **O(1)** time.
    #[inline]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Returns true if the map contains no elements.
    ///
    /// Computes in **O(1)** time.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Return an iterator over the key-value pairs of the map, in their order
    pub fn iter(&self) -> Iter<'_, K, V> {
        self.inner.iter()
    }

    /// Return an iterator over the key-value pairs of the map, in their order
    pub fn iter_mut(&mut self) -> IterMut<'_, K, V> {
        self.inner.iter_mut()
    }

    /// Return an iterator over the keys of the map, in their order
    pub fn keys(&self) -> Keys<'_, K, V> {
        self.inner.keys()
    }

    /// Return an owning iterator over the keys of the map, in their order
    pub fn into_keys(self) -> IntoKeys<K, V> {
        self.inner.into_keys()
    }

    /// Return an iterator over the values of the map, in their order
    pub fn values(&self) -> Values<'_, K, V> {
        self.inner.values()
    }

    /// Return an iterator over mutable references to the values of the map,
    /// in their order
    pub fn values_mut(&mut self) -> ValuesMut<'_, K, V> {
        self.inner.values_mut()
    }

    /// Return an owning iterator over the values of the map, in their order
    pub fn into_values(self) -> IntoValues<K, V> {
        self.inner.into_values()
    }

    /// Remove all key-value pairs in the map, while preserving its capacity.
    ///
    /// Computes in **O(n)** time.
    pub fn clear(&mut self) {
        self.inner.clear();
    }

    /// Shortens the map, keeping the first `len` elements and dropping the rest.
    ///
    /// If `len` is greater than the map's current length, this has no effect.
    pub fn truncate(&mut self, len: usize) {
        self.inner.truncate(len);
    }

    /// Clears the `OrderMap` in the given index range, returning those
    /// key-value pairs as a drain iterator.
    ///
    /// The range may be any type that implements [`RangeBounds<usize>`],
    /// including all of the `std::ops::Range*` types, or even a tuple pair of
    /// `Bound` start and end values. To drain the map entirely, use `RangeFull`
    /// like `map.drain(..)`.
    ///
    /// This shifts down all entries following the drained range to fill the
    /// gap, and keeps the allocated memory for reuse.
    ///
    /// ***Panics*** if the starting point is greater than the end point or if
    /// the end point is greater than the length of the map.
    pub fn drain<R>(&mut self, range: R) -> Drain<'_, K, V>
    where
        R: RangeBounds<usize>,
    {
        self.inner.drain(range)
    }

    /// Splits the collection into two at the given index.
    ///
    /// Returns a newly allocated map containing the elements in the range
    /// `[at, len)`. After the call, the original map will be left containing
    /// the elements `[0, at)` with its previous capacity unchanged.
    ///
    /// ***Panics*** if `at > len`.
    pub fn split_off(&mut self, at: usize) -> Self
    where
        S: Clone,
    {
        Self {
            inner: self.inner.split_off(at),
        }
    }

    /// Reserve capacity for `additional` more key-value pairs.
    ///
    /// Computes in **O(n)** time.
    pub fn reserve(&mut self, additional: usize) {
        self.inner.reserve(additional);
    }

    /// Reserve capacity for `additional` more key-value pairs, without over-allocating.
    ///
    /// Unlike `reserve`, this does not deliberately over-allocate the entry capacity to avoid
    /// frequent re-allocations. However, the underlying data structures may still have internal
    /// capacity requirements, and the allocator itself may give more space than requested, so this
    /// cannot be relied upon to be precisely minimal.
    ///
    /// Computes in **O(n)** time.
    pub fn reserve_exact(&mut self, additional: usize) {
        self.inner.reserve_exact(additional);
    }

    /// Try to reserve capacity for `additional` more key-value pairs.
    ///
    /// Computes in **O(n)** time.
    pub fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.inner.try_reserve(additional)
    }

    /// Try to reserve capacity for `additional` more key-value pairs, without over-allocating.
    ///
    /// Unlike `try_reserve`, this does not deliberately over-allocate the entry capacity to avoid
    /// frequent re-allocations. However, the underlying data structures may still have internal
    /// capacity requirements, and the allocator itself may give more space than requested, so this
    /// cannot be relied upon to be precisely minimal.
    ///
    /// Computes in **O(n)** time.
    pub fn try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.inner.try_reserve_exact(additional)
    }

    /// Shrink the capacity of the map as much as possible.
    ///
    /// Computes in **O(n)** time.
    pub fn shrink_to_fit(&mut self) {
        self.inner.shrink_to(0);
    }

    /// Shrink the capacity of the map with a lower limit.
    ///
    /// Computes in **O(n)** time.
    pub fn shrink_to(&mut self, min_capacity: usize) {
        self.inner.shrink_to(min_capacity);
    }
}

impl<K, V, S> OrderMap<K, V, S>
where
    K: Hash + Eq,
    S: BuildHasher,
{
    /// Insert a key-value pair in the map.
    ///
    /// If an equivalent key already exists in the map: the key remains and
    /// retains in its place in the order, its corresponding value is updated
    /// with `value`, and the older value is returned inside `Some(_)`.
    ///
    /// If no equivalent key existed in the map: the new key-value pair is
    /// inserted, last in order, and `None` is returned.
    ///
    /// Computes in **O(1)** time (amortized average).
    ///
    /// See also [`entry`][Self::entry] if you want to insert *or* modify,
    /// or [`insert_full`][Self::insert_full] if you need to get the index of
    /// the corresponding key-value pair.
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.inner.insert(key, value)
    }

    /// Insert a key-value pair in the map, and get their index.
    ///
    /// If an equivalent key already exists in the map: the key remains and
    /// retains in its place in the order, its corresponding value is updated
    /// with `value`, and the older value is returned inside `(index, Some(_))`.
    ///
    /// If no equivalent key existed in the map: the new key-value pair is
    /// inserted, last in order, and `(index, None)` is returned.
    ///
    /// Computes in **O(1)** time (amortized average).
    ///
    /// See also [`entry`][Self::entry] if you want to insert *or* modify.
    pub fn insert_full(&mut self, key: K, value: V) -> (usize, Option<V>) {
        self.inner.insert_full(key, value)
    }

    /// Insert a key-value pair in the map at its ordered position among sorted keys.
    ///
    /// This is equivalent to finding the position with
    /// [`binary_search_keys`][Self::binary_search_keys], then either updating
    /// it or calling [`shift_insert`][Self::shift_insert] for a new key.
    ///
    /// If the sorted key is found in the map, its corresponding value is
    /// updated with `value`, and the older value is returned inside
    /// `(index, Some(_))`. Otherwise, the new key-value pair is inserted at
    /// the sorted position, and `(index, None)` is returned.
    ///
    /// If the existing keys are **not** already sorted, then the insertion
    /// index is unspecified (like [`slice::binary_search`]), but the key-value
    /// pair is moved to or inserted at that position regardless.
    ///
    /// Computes in **O(n)** time (average). Instead of repeating calls to
    /// `insert_sorted`, it may be faster to call batched [`insert`][Self::insert]
    /// or [`extend`][Self::extend] and only call [`sort_keys`][Self::sort_keys]
    /// or [`sort_unstable_keys`][Self::sort_unstable_keys] once.
    pub fn insert_sorted(&mut self, key: K, value: V) -> (usize, Option<V>)
    where
        K: Ord,
    {
        self.inner.insert_sorted(key, value)
    }

    /// Insert a key-value pair in the map at the given index.
    ///
    /// If an equivalent key already exists in the map: the key remains and
    /// is moved to the new position in the map, its corresponding value is updated
    /// with `value`, and the older value is returned inside `Some(_)`.
    ///
    /// If no equivalent key existed in the map: the new key-value pair is
    /// inserted at the given index, and `None` is returned.
    ///
    /// ***Panics*** if `index` is out of bounds.
    ///
    /// Computes in **O(n)** time (average).
    ///
    /// See also [`entry`][Self::entry] if you want to insert *or* modify,
    /// perhaps only using the index for new entries with [`VacantEntry::shift_insert`].
    pub fn shift_insert(&mut self, index: usize, key: K, value: V) -> Option<V> {
        self.inner.shift_insert(index, key, value)
    }

    /// Get the given key’s corresponding entry in the map for insertion and/or
    /// in-place manipulation.
    ///
    /// Computes in **O(1)** time (amortized average).
    pub fn entry(&mut self, key: K) -> Entry<'_, K, V> {
        Entry::new(self.inner.entry(key))
    }

    /// Creates a splicing iterator that replaces the specified range in the map
    /// with the given `replace_with` key-value iterator and yields the removed
    /// items. `replace_with` does not need to be the same length as `range`.
    ///
    /// The `range` is removed even if the iterator is not consumed until the
    /// end. It is unspecified how many elements are removed from the map if the
    /// `Splice` value is leaked.
    ///
    /// The input iterator `replace_with` is only consumed when the `Splice`
    /// value is dropped. If a key from the iterator matches an existing entry
    /// in the map (outside of `range`), then the value will be updated in that
    /// position. Otherwise, the new key-value pair will be inserted in the
    /// replaced `range`.
    ///
    /// ***Panics*** if the starting point is greater than the end point or if
    /// the end point is greater than the length of the map.
    ///
    /// # Examples
    ///
    /// ```
    /// use ordermap::OrderMap;
    ///
    /// let mut map = OrderMap::from([(0, '_'), (1, 'a'), (2, 'b'), (3, 'c'), (4, 'd')]);
    /// let new = [(5, 'E'), (4, 'D'), (3, 'C'), (2, 'B'), (1, 'A')];
    /// let removed: Vec<_> = map.splice(2..4, new).collect();
    ///
    /// // 1 and 4 got new values, while 5, 3, and 2 were newly inserted.
    /// assert!(map.into_iter().eq([(0, '_'), (1, 'A'), (5, 'E'), (3, 'C'), (2, 'B'), (4, 'D')]));
    /// assert_eq!(removed, &[(2, 'b'), (3, 'c')]);
    /// ```
    pub fn splice<R, I>(&mut self, range: R, replace_with: I) -> Splice<'_, I::IntoIter, K, V, S>
    where
        R: RangeBounds<usize>,
        I: IntoIterator<Item = (K, V)>,
    {
        self.inner.splice(range, replace_with)
    }
}

impl<K, V, S> OrderMap<K, V, S>
where
    S: BuildHasher,
{
    /// Return `true` if an equivalent to `key` exists in the map.
    ///
    /// Computes in **O(1)** time (average).
    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        Q: ?Sized + Hash + Equivalent<K>,
    {
        self.inner.contains_key(key)
    }

    /// Return a reference to the value stored for `key`, if it is present,
    /// else `None`.
    ///
    /// Computes in **O(1)** time (average).
    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        Q: ?Sized + Hash + Equivalent<K>,
    {
        self.inner.get(key)
    }

    /// Return references to the key-value pair stored for `key`,
    /// if it is present, else `None`.
    ///
    /// Computes in **O(1)** time (average).
    pub fn get_key_value<Q>(&self, key: &Q) -> Option<(&K, &V)>
    where
        Q: ?Sized + Hash + Equivalent<K>,
    {
        self.inner.get_key_value(key)
    }

    /// Return item index, key and value
    pub fn get_full<Q>(&self, key: &Q) -> Option<(usize, &K, &V)>
    where
        Q: ?Sized + Hash + Equivalent<K>,
    {
        self.inner.get_full(key)
    }

    /// Return item index, if it exists in the map
    ///
    /// Computes in **O(1)** time (average).
    pub fn get_index_of<Q>(&self, key: &Q) -> Option<usize>
    where
        Q: ?Sized + Hash + Equivalent<K>,
    {
        self.inner.get_index_of(key)
    }

    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
    where
        Q: ?Sized + Hash + Equivalent<K>,
    {
        self.inner.get_mut(key)
    }

    pub fn get_full_mut<Q>(&mut self, key: &Q) -> Option<(usize, &K, &mut V)>
    where
        Q: ?Sized + Hash + Equivalent<K>,
    {
        self.inner.get_full_mut(key)
    }

    /// Remove the key-value pair equivalent to `key` and return its value.
    ///
    /// **NOTE:** This is equivalent to [`IndexMap::shift_remove`], and
    /// like [`Vec::remove`], the pair is removed by shifting all of the
    /// elements that follow it, preserving their relative order.
    /// **This perturbs the index of all of those elements!**
    ///
    /// Return `None` if `key` is not in map.
    ///
    /// Computes in **O(n)** time (average).
    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        Q: ?Sized + Hash + Equivalent<K>,
    {
        self.inner.shift_remove(key)
    }

    /// Remove and return the key-value pair equivalent to `key`.
    ///
    /// **NOTE:** This is equivalent to [`IndexMap::shift_remove_entry`], and
    /// like [`Vec::remove`], the pair is removed by shifting all of the
    /// elements that follow it, preserving their relative order.
    /// **This perturbs the index of all of those elements!**
    ///
    /// Return `None` if `key` is not in map.
    ///
    /// Computes in **O(n)** time (average).
    pub fn remove_entry<Q>(&mut self, key: &Q) -> Option<(K, V)>
    where
        Q: ?Sized + Hash + Equivalent<K>,
    {
        self.inner.shift_remove_entry(key)
    }

    /// Remove the key-value pair equivalent to `key` and return it and
    /// the index it had.
    ///
    /// **NOTE:** This is equivalent to [`IndexMap::shift_remove_full`], and
    /// like [`Vec::remove`], the pair is removed by shifting all of the
    /// elements that follow it, preserving their relative order.
    /// **This perturbs the index of all of those elements!**
    ///
    /// Return `None` if `key` is not in map.
    ///
    /// Computes in **O(n)** time (average).
    pub fn remove_full<Q>(&mut self, key: &Q) -> Option<(usize, K, V)>
    where
        Q: ?Sized + Hash + Equivalent<K>,
    {
        self.inner.shift_remove_full(key)
    }

    /// Remove the key-value pair equivalent to `key` and return
    /// its value.
    ///
    /// Like [`Vec::swap_remove`], the pair is removed by swapping it with the
    /// last element of the map and popping it off. **This perturbs
    /// the position of what used to be the last element!**
    ///
    /// Return `None` if `key` is not in map.
    ///
    /// Computes in **O(1)** time (average).
    pub fn swap_remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        Q: ?Sized + Hash + Equivalent<K>,
    {
        self.inner.swap_remove(key)
    }

    /// Remove and return the key-value pair equivalent to `key`.
    ///
    /// Like [`Vec::swap_remove`], the pair is removed by swapping it with the
    /// last element of the map and popping it off. **This perturbs
    /// the position of what used to be the last element!**
    ///
    /// Return `None` if `key` is not in map.
    ///
    /// Computes in **O(1)** time (average).
    pub fn swap_remove_entry<Q>(&mut self, key: &Q) -> Option<(K, V)>
    where
        Q: ?Sized + Hash + Equivalent<K>,
    {
        self.inner.swap_remove_entry(key)
    }

    /// Remove the key-value pair equivalent to `key` and return it and
    /// the index it had.
    ///
    /// Like [`Vec::swap_remove`], the pair is removed by swapping it with the
    /// last element of the map and popping it off. **This perturbs
    /// the position of what used to be the last element!**
    ///
    /// Return `None` if `key` is not in map.
    ///
    /// Computes in **O(1)** time (average).
    pub fn swap_remove_full<Q>(&mut self, key: &Q) -> Option<(usize, K, V)>
    where
        Q: ?Sized + Hash + Equivalent<K>,
    {
        self.inner.swap_remove_full(key)
    }
}

impl<K, V, S> OrderMap<K, V, S> {
    /// Remove the last key-value pair
    ///
    /// This preserves the order of the remaining elements.
    ///
    /// Computes in **O(1)** time (average).
    pub fn pop(&mut self) -> Option<(K, V)> {
        self.inner.pop()
    }

    /// Scan through each key-value pair in the map and keep those where the
    /// closure `keep` returns `true`.
    ///
    /// The elements are visited in order, and remaining elements keep their
    /// order.
    ///
    /// Computes in **O(n)** time (average).
    pub fn retain<F>(&mut self, keep: F)
    where
        F: FnMut(&K, &mut V) -> bool,
    {
        self.inner.retain(keep);
    }

    /// Sort the map’s key-value pairs by the default ordering of the keys.
    ///
    /// This is a stable sort -- but equivalent keys should not normally coexist in
    /// a map at all, so [`sort_unstable_keys`][Self::sort_unstable_keys] is preferred
    /// because it is generally faster and doesn't allocate auxiliary memory.
    ///
    /// See [`sort_by`](Self::sort_by) for details.
    pub fn sort_keys(&mut self)
    where
        K: Ord,
    {
        self.inner.sort_keys();
    }

    /// Sort the map’s key-value pairs in place using the comparison
    /// function `cmp`.
    ///
    /// The comparison function receives two key and value pairs to compare (you
    /// can sort by keys or values or their combination as needed).
    ///
    /// Computes in **O(n log n + c)** time and **O(n)** space where *n* is
    /// the length of the map and *c* the capacity. The sort is stable.
    pub fn sort_by<F>(&mut self, cmp: F)
    where
        F: FnMut(&K, &V, &K, &V) -> Ordering,
    {
        self.inner.sort_by(cmp);
    }

    /// Sort the key-value pairs of the map and return a by-value iterator of
    /// the key-value pairs with the result.
    ///
    /// The sort is stable.
    pub fn sorted_by<F>(self, cmp: F) -> IntoIter<K, V>
    where
        F: FnMut(&K, &V, &K, &V) -> Ordering,
    {
        self.inner.sorted_by(cmp)
    }

    /// Sort the map's key-value pairs by the default ordering of the keys, but
    /// may not preserve the order of equal elements.
    ///
    /// See [`sort_unstable_by`](Self::sort_unstable_by) for details.
    pub fn sort_unstable_keys(&mut self)
    where
        K: Ord,
    {
        self.inner.sort_unstable_keys();
    }

    /// Sort the map's key-value pairs in place using the comparison function `cmp`, but
    /// may not preserve the order of equal elements.
    ///
    /// The comparison function receives two key and value pairs to compare (you
    /// can sort by keys or values or their combination as needed).
    ///
    /// Computes in **O(n log n + c)** time where *n* is
    /// the length of the map and *c* is the capacity. The sort is unstable.
    pub fn sort_unstable_by<F>(&mut self, cmp: F)
    where
        F: FnMut(&K, &V, &K, &V) -> Ordering,
    {
        self.inner.sort_unstable_by(cmp);
    }

    /// Sort the key-value pairs of the map and return a by-value iterator of
    /// the key-value pairs with the result.
    ///
    /// The sort is unstable.
    #[inline]
    pub fn sorted_unstable_by<F>(self, cmp: F) -> IntoIter<K, V>
    where
        F: FnMut(&K, &V, &K, &V) -> Ordering,
    {
        self.inner.sorted_unstable_by(cmp)
    }

    /// Sort the map’s key-value pairs in place using a sort-key extraction function.
    ///
    /// During sorting, the function is called at most once per entry, by using temporary storage
    /// to remember the results of its evaluation. The order of calls to the function is
    /// unspecified and may change between versions of `ordermap` or the standard library.
    ///
    /// Computes in **O(m n + n log n + c)** time () and **O(n)** space, where the function is
    /// **O(m)**, *n* is the length of the map, and *c* the capacity. The sort is stable.
    pub fn sort_by_cached_key<T, F>(&mut self, sort_key: F)
    where
        T: Ord,
        F: FnMut(&K, &V) -> T,
    {
        self.inner.sort_by_cached_key(sort_key);
    }

    /// Search over a sorted map for a key.
    ///
    /// Returns the position where that key is present, or the position where it can be inserted to
    /// maintain the sort. See [`slice::binary_search`] for more details.
    ///
    /// Computes in **O(log(n))** time, which is notably less scalable than looking the key up
    /// using [`get_index_of`][OrderMap::get_index_of], but this can also position missing keys.
    pub fn binary_search_keys(&self, x: &K) -> Result<usize, usize>
    where
        K: Ord,
    {
        self.inner.binary_search_keys(x)
    }

    /// Search over a sorted map with a comparator function.
    ///
    /// Returns the position where that value is present, or the position where it can be inserted
    /// to maintain the sort. See [`slice::binary_search_by`] for more details.
    ///
    /// Computes in **O(log(n))** time.
    #[inline]
    pub fn binary_search_by<'a, F>(&'a self, f: F) -> Result<usize, usize>
    where
        F: FnMut(&'a K, &'a V) -> Ordering,
    {
        self.inner.binary_search_by(f)
    }

    /// Search over a sorted map with an extraction function.
    ///
    /// Returns the position where that value is present, or the position where it can be inserted
    /// to maintain the sort. See [`slice::binary_search_by_key`] for more details.
    ///
    /// Computes in **O(log(n))** time.
    #[inline]
    pub fn binary_search_by_key<'a, B, F>(&'a self, b: &B, f: F) -> Result<usize, usize>
    where
        F: FnMut(&'a K, &'a V) -> B,
        B: Ord,
    {
        self.inner.binary_search_by_key(b, f)
    }

    /// Returns the index of the partition point of a sorted map according to the given predicate
    /// (the index of the first element of the second partition).
    ///
    /// See [`slice::partition_point`] for more details.
    ///
    /// Computes in **O(log(n))** time.
    #[must_use]
    pub fn partition_point<P>(&self, pred: P) -> usize
    where
        P: FnMut(&K, &V) -> bool,
    {
        self.inner.partition_point(pred)
    }

    /// Reverses the order of the map’s key-value pairs in place.
    ///
    /// Computes in **O(n)** time and **O(1)** space.
    pub fn reverse(&mut self) {
        self.inner.reverse()
    }

    /// Returns a slice of all the key-value pairs in the map.
    ///
    /// Computes in **O(1)** time.
    pub fn as_slice(&self) -> &Slice<K, V> {
        self.inner.as_slice()
    }

    /// Returns a mutable slice of all the key-value pairs in the map.
    ///
    /// Computes in **O(1)** time.
    pub fn as_mut_slice(&mut self) -> &mut Slice<K, V> {
        self.inner.as_mut_slice()
    }

    /// Converts into a boxed slice of all the key-value pairs in the map.
    ///
    /// Note that this will drop the inner hash table and any excess capacity.
    pub fn into_boxed_slice(self) -> Box<Slice<K, V>> {
        self.inner.into_boxed_slice()
    }

    /// Get a key-value pair by index
    ///
    /// Valid indices are *0 <= index < self.len()*
    ///
    /// Computes in **O(1)** time.
    pub fn get_index(&self, index: usize) -> Option<(&K, &V)> {
        self.inner.get_index(index)
    }

    /// Get a key-value pair by index
    ///
    /// Valid indices are *0 <= index < self.len()*
    ///
    /// Computes in **O(1)** time.
    pub fn get_index_mut(&mut self, index: usize) -> Option<(&K, &mut V)> {
        self.inner.get_index_mut(index)
    }

    /// Get an entry in the map by index for in-place manipulation.
    ///
    /// Valid indices are *0 <= index < self.len()*
    ///
    /// Computes in **O(1)** time.
    pub fn get_index_entry(&mut self, index: usize) -> Option<IndexedEntry<'_, K, V>> {
        self.inner.get_index_entry(index).map(IndexedEntry::new)
    }

    /// Returns a slice of key-value pairs in the given range of indices.
    ///
    /// Valid indices are *0 <= index < self.len()*
    ///
    /// Computes in **O(1)** time.
    pub fn get_range<R: RangeBounds<usize>>(&self, range: R) -> Option<&Slice<K, V>> {
        self.inner.get_range(range)
    }

    /// Returns a mutable slice of key-value pairs in the given range of indices.
    ///
    /// Valid indices are *0 <= index < self.len()*
    ///
    /// Computes in **O(1)** time.
    pub fn get_range_mut<R: RangeBounds<usize>>(&mut self, range: R) -> Option<&mut Slice<K, V>> {
        self.inner.get_range_mut(range)
    }

    /// Get the first key-value pair
    ///
    /// Computes in **O(1)** time.
    pub fn first(&self) -> Option<(&K, &V)> {
        self.inner.first()
    }

    /// Get the first key-value pair, with mutable access to the value
    ///
    /// Computes in **O(1)** time.
    pub fn first_mut(&mut self) -> Option<(&K, &mut V)> {
        self.inner.first_mut()
    }

    /// Get the last key-value pair
    ///
    /// Computes in **O(1)** time.
    pub fn last(&self) -> Option<(&K, &V)> {
        self.inner.last()
    }

    /// Get the last key-value pair, with mutable access to the value
    ///
    /// Computes in **O(1)** time.
    pub fn last_mut(&mut self) -> Option<(&K, &mut V)> {
        self.inner.last_mut()
    }

    /// Remove the key-value pair by index
    ///
    /// Valid indices are *0 <= index < self.len()*
    ///
    /// **NOTE:** This is equivalent to [`IndexMap::shift_remove_index`], and
    /// like [`Vec::remove`], the pair is removed by shifting all of the
    /// elements that follow it, preserving their relative order.
    /// **This perturbs the index of all of those elements!**
    ///
    /// Computes in **O(n)** time (average).
    pub fn remove_index(&mut self, index: usize) -> Option<(K, V)> {
        self.inner.shift_remove_index(index)
    }

    /// Remove the key-value pair by index
    ///
    /// Valid indices are *0 <= index < self.len()*
    ///
    /// Like [`Vec::swap_remove`], the pair is removed by swapping it with the
    /// last element of the map and popping it off. **This perturbs
    /// the position of what used to be the last element!**
    ///
    /// Computes in **O(1)** time (average).
    pub fn swap_remove_index(&mut self, index: usize) -> Option<(K, V)> {
        self.inner.swap_remove_index(index)
    }

    /// Moves the position of a key-value pair from one index to another
    /// by shifting all other pairs in-between.
    ///
    /// * If `from < to`, the other pairs will shift down while the targeted pair moves up.
    /// * If `from > to`, the other pairs will shift up while the targeted pair moves down.
    ///
    /// ***Panics*** if `from` or `to` are out of bounds.
    ///
    /// Computes in **O(n)** time (average).
    pub fn move_index(&mut self, from: usize, to: usize) {
        self.inner.move_index(from, to)
    }

    /// Swaps the position of two key-value pairs in the map.
    ///
    /// ***Panics*** if `a` or `b` are out of bounds.
    ///
    /// Computes in **O(1)** time (average).
    pub fn swap_indices(&mut self, a: usize, b: usize) {
        self.inner.swap_indices(a, b)
    }
}

/// Access [`OrderMap`] values corresponding to a key.
///
/// # Examples
///
/// ```
/// use ordermap::OrderMap;
///
/// let mut map = OrderMap::new();
/// for word in "Lorem ipsum dolor sit amet".split_whitespace() {
///     map.insert(word.to_lowercase(), word.to_uppercase());
/// }
/// assert_eq!(map["lorem"], "LOREM");
/// assert_eq!(map["ipsum"], "IPSUM");
/// ```
///
/// ```should_panic
/// use ordermap::OrderMap;
///
/// let mut map = OrderMap::new();
/// map.insert("foo", 1);
/// println!("{:?}", map["bar"]); // panics!
/// ```
impl<K, V, Q: ?Sized, S> Index<&Q> for OrderMap<K, V, S>
where
    Q: Hash + Equivalent<K>,
    S: BuildHasher,
{
    type Output = V;

    /// Returns a reference to the value corresponding to the supplied `key`.
    ///
    /// ***Panics*** if `key` is not present in the map.
    fn index(&self, key: &Q) -> &V {
        self.get(key).expect("OrderMap: key not found")
    }
}

/// Access [`OrderMap`] values corresponding to a key.
///
/// Mutable indexing allows changing / updating values of key-value
/// pairs that are already present.
///
/// You can **not** insert new pairs with index syntax, use `.insert()`.
///
/// # Examples
///
/// ```
/// use ordermap::OrderMap;
///
/// let mut map = OrderMap::new();
/// for word in "Lorem ipsum dolor sit amet".split_whitespace() {
///     map.insert(word.to_lowercase(), word.to_string());
/// }
/// let lorem = &mut map["lorem"];
/// assert_eq!(lorem, "Lorem");
/// lorem.retain(char::is_lowercase);
/// assert_eq!(map["lorem"], "orem");
/// ```
///
/// ```should_panic
/// use ordermap::OrderMap;
///
/// let mut map = OrderMap::new();
/// map.insert("foo", 1);
/// map["bar"] = 1; // panics!
/// ```
impl<K, V, Q: ?Sized, S> IndexMut<&Q> for OrderMap<K, V, S>
where
    Q: Hash + Equivalent<K>,
    S: BuildHasher,
{
    /// Returns a mutable reference to the value corresponding to the supplied `key`.
    ///
    /// ***Panics*** if `key` is not present in the map.
    fn index_mut(&mut self, key: &Q) -> &mut V {
        self.get_mut(key).expect("OrderMap: key not found")
    }
}

/// Access [`OrderMap`] values at indexed positions.
///
/// See [`Index<usize> for Keys`][keys] to access a map's keys instead.
///
/// [keys]: Keys#impl-Index<usize>-for-Keys<'a,+K,+V>
///
/// # Examples
///
/// ```
/// use ordermap::OrderMap;
///
/// let mut map = OrderMap::new();
/// for word in "Lorem ipsum dolor sit amet".split_whitespace() {
///     map.insert(word.to_lowercase(), word.to_uppercase());
/// }
/// assert_eq!(map[0], "LOREM");
/// assert_eq!(map[1], "IPSUM");
/// map.reverse();
/// assert_eq!(map[0], "AMET");
/// assert_eq!(map[1], "SIT");
/// map.sort_keys();
/// assert_eq!(map[0], "AMET");
/// assert_eq!(map[1], "DOLOR");
/// ```
///
/// ```should_panic
/// use ordermap::OrderMap;
///
/// let mut map = OrderMap::new();
/// map.insert("foo", 1);
/// println!("{:?}", map[10]); // panics!
/// ```
impl<K, V, S> Index<usize> for OrderMap<K, V, S> {
    type Output = V;

    /// Returns a reference to the value at the supplied `index`.
    ///
    /// ***Panics*** if `index` is out of bounds.
    fn index(&self, index: usize) -> &V {
        self.get_index(index)
            .expect("OrderMap: index out of bounds")
            .1
    }
}

/// Access [`OrderMap`] values at indexed positions.
///
/// Mutable indexing allows changing / updating indexed values
/// that are already present.
///
/// You can **not** insert new values with index syntax -- use [`.insert()`][OrderMap::insert].
///
/// # Examples
///
/// ```
/// use ordermap::OrderMap;
///
/// let mut map = OrderMap::new();
/// for word in "Lorem ipsum dolor sit amet".split_whitespace() {
///     map.insert(word.to_lowercase(), word.to_string());
/// }
/// let lorem = &mut map[0];
/// assert_eq!(lorem, "Lorem");
/// lorem.retain(char::is_lowercase);
/// assert_eq!(map["lorem"], "orem");
/// ```
///
/// ```should_panic
/// use ordermap::OrderMap;
///
/// let mut map = OrderMap::new();
/// map.insert("foo", 1);
/// map[10] = 1; // panics!
/// ```
impl<K, V, S> IndexMut<usize> for OrderMap<K, V, S> {
    /// Returns a mutable reference to the value at the supplied `index`.
    ///
    /// ***Panics*** if `index` is out of bounds.
    fn index_mut(&mut self, index: usize) -> &mut V {
        self.get_index_mut(index)
            .expect("OrderMap: index out of bounds")
            .1
    }
}

impl<K, V, S> FromIterator<(K, V)> for OrderMap<K, V, S>
where
    K: Hash + Eq,
    S: BuildHasher + Default,
{
    /// Create an `OrderMap` from the sequence of key-value pairs in the
    /// iterable.
    ///
    /// `from_iter` uses the same logic as `extend`. See
    /// [`extend`][OrderMap::extend] for more details.
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iterable: I) -> Self {
        Self {
            inner: IndexMap::from_iter(iterable),
        }
    }
}

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl<K, V, const N: usize> From<[(K, V); N]> for OrderMap<K, V, RandomState>
where
    K: Hash + Eq,
{
    /// # Examples
    ///
    /// ```
    /// use ordermap::OrderMap;
    ///
    /// let map1 = OrderMap::from([(1, 2), (3, 4)]);
    /// let map2: OrderMap<_, _> = [(1, 2), (3, 4)].into();
    /// assert_eq!(map1, map2);
    /// ```
    fn from(arr: [(K, V); N]) -> Self {
        Self::from_iter(arr)
    }
}

impl<K, V, S> Extend<(K, V)> for OrderMap<K, V, S>
where
    K: Hash + Eq,
    S: BuildHasher,
{
    /// Extend the map with all key-value pairs in the iterable.
    ///
    /// This is equivalent to calling [`insert`][OrderMap::insert] for each of
    /// them in order, which means that for keys that already existed
    /// in the map, their value is updated but it keeps the existing order.
    ///
    /// New keys are inserted in the order they appear in the sequence. If
    /// equivalents of a key occur more than once, the last corresponding value
    /// prevails.
    fn extend<I: IntoIterator<Item = (K, V)>>(&mut self, iterable: I) {
        self.inner.extend(iterable);
    }
}

impl<'a, K, V, S> Extend<(&'a K, &'a V)> for OrderMap<K, V, S>
where
    K: Hash + Eq + Copy,
    V: Copy,
    S: BuildHasher,
{
    /// Extend the map with all key-value pairs in the iterable.
    ///
    /// See the first extend method for more details.
    fn extend<I: IntoIterator<Item = (&'a K, &'a V)>>(&mut self, iterable: I) {
        self.inner.extend(iterable);
    }
}

impl<K, V, S> Default for OrderMap<K, V, S>
where
    S: Default,
{
    /// Return an empty [`OrderMap`]
    fn default() -> Self {
        Self::with_capacity_and_hasher(0, S::default())
    }
}

impl<K, V, S1, S2> PartialEq<OrderMap<K, V, S2>> for OrderMap<K, V, S1>
where
    K: PartialEq,
    V: PartialEq,
{
    fn eq(&self, other: &OrderMap<K, V, S2>) -> bool {
        self.len() == other.len() && self.iter().eq(other)
    }
}

impl<K, V, S> Eq for OrderMap<K, V, S>
where
    K: Eq,
    V: Eq,
{
}

impl<K, V, S1, S2> PartialOrd<OrderMap<K, V, S2>> for OrderMap<K, V, S1>
where
    K: PartialOrd,
    V: PartialOrd,
{
    fn partial_cmp(&self, other: &OrderMap<K, V, S2>) -> Option<Ordering> {
        self.iter().partial_cmp(other)
    }
}

impl<K, V, S> Ord for OrderMap<K, V, S>
where
    K: Ord,
    V: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.iter().cmp(other)
    }
}

impl<K, V, S> Hash for OrderMap<K, V, S>
where
    K: Hash,
    V: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.len().hash(state);
        for (key, value) in self {
            key.hash(state);
            value.hash(state);
        }
    }
}
