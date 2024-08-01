use core::fmt;
use indexmap::map as ix;

#[cfg(doc)]
use alloc::vec::Vec;

/// Entry for an existing key-value pair in an [`OrderMap`][crate::OrderMap]
/// or a vacant location to insert one.
pub enum Entry<'a, K, V> {
    /// Existing slot with equivalent key.
    Occupied(OccupiedEntry<'a, K, V>),
    /// Vacant slot (no equivalent key in the map).
    Vacant(VacantEntry<'a, K, V>),
}

impl<'a, K, V> Entry<'a, K, V> {
    pub(super) fn new(entry: ix::Entry<'a, K, V>) -> Self {
        match entry {
            ix::Entry::Occupied(inner) => Self::Occupied(OccupiedEntry { inner }),
            ix::Entry::Vacant(inner) => Self::Vacant(VacantEntry { inner }),
        }
    }

    /// Return the index where the key-value pair exists or will be inserted.
    pub fn index(&self) -> usize {
        match *self {
            Entry::Occupied(ref entry) => entry.index(),
            Entry::Vacant(ref entry) => entry.index(),
        }
    }

    /// Inserts the given default value in the entry if it is vacant and returns a mutable
    /// reference to it. Otherwise a mutable reference to an already existent value is returned.
    ///
    /// Computes in **O(1)** time (amortized average).
    pub fn or_insert(self, default: V) -> &'a mut V {
        match self {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert(default),
        }
    }

    /// Inserts the result of the `call` function in the entry if it is vacant and returns a mutable
    /// reference to it. Otherwise a mutable reference to an already existent value is returned.
    ///
    /// Computes in **O(1)** time (amortized average).
    pub fn or_insert_with<F>(self, call: F) -> &'a mut V
    where
        F: FnOnce() -> V,
    {
        match self {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert(call()),
        }
    }

    /// Inserts the result of the `call` function with a reference to the entry's key if it is
    /// vacant, and returns a mutable reference to the new value. Otherwise a mutable reference to
    /// an already existent value is returned.
    ///
    /// Computes in **O(1)** time (amortized average).
    pub fn or_insert_with_key<F>(self, call: F) -> &'a mut V
    where
        F: FnOnce(&K) -> V,
    {
        match self {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => {
                let value = call(&entry.key());
                entry.insert(value)
            }
        }
    }

    /// Gets a reference to the entry's key, either within the map if occupied,
    /// or else the new key that was used to find the entry.
    pub fn key(&self) -> &K {
        match *self {
            Entry::Occupied(ref entry) => entry.key(),
            Entry::Vacant(ref entry) => entry.key(),
        }
    }

    /// Modifies the entry if it is occupied.
    pub fn and_modify<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut V),
    {
        if let Entry::Occupied(entry) = &mut self {
            f(entry.get_mut());
        }
        self
    }

    /// Inserts a default-constructed value in the entry if it is vacant and returns a mutable
    /// reference to it. Otherwise a mutable reference to an already existent value is returned.
    ///
    /// Computes in **O(1)** time (amortized average).
    pub fn or_default(self) -> &'a mut V
    where
        V: Default,
    {
        match self {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert(V::default()),
        }
    }
}

impl<K: fmt::Debug, V: fmt::Debug> fmt::Debug for Entry<'_, K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut tuple = f.debug_tuple("Entry");
        match self {
            Entry::Vacant(v) => tuple.field(v),
            Entry::Occupied(o) => tuple.field(o),
        };
        tuple.finish()
    }
}

/// A view into an occupied entry in an [`OrderMap`][crate::OrderMap].
/// It is part of the [`Entry`] enum.
pub struct OccupiedEntry<'a, K, V> {
    pub(crate) inner: ix::OccupiedEntry<'a, K, V>,
}

impl<'a, K, V> OccupiedEntry<'a, K, V> {
    /// Return the index of the key-value pair
    #[inline]
    pub fn index(&self) -> usize {
        self.inner.index()
    }

    /// Gets a reference to the entry's key in the map.
    ///
    /// Note that this is not the key that was used to find the entry. There may be an observable
    /// difference if the key type has any distinguishing features outside of `Hash` and `Eq`, like
    /// extra fields or the memory address of an allocation.
    pub fn key(&self) -> &K {
        self.inner.key()
    }

    /// Gets a reference to the entry's value in the map.
    pub fn get(&self) -> &V {
        self.inner.get()
    }

    /// Gets a mutable reference to the entry's value in the map.
    ///
    /// If you need a reference which may outlive the destruction of the
    /// [`Entry`] value, see [`into_mut`][Self::into_mut].
    pub fn get_mut(&mut self) -> &mut V {
        self.inner.get_mut()
    }

    /// Converts into a mutable reference to the entry's value in the map,
    /// with a lifetime bound to the map itself.
    pub fn into_mut(self) -> &'a mut V {
        self.inner.into_mut()
    }

    /// Sets the value of the entry to `value`, and returns the entry's old value.
    pub fn insert(&mut self, value: V) -> V {
        self.inner.insert(value)
    }

    /// Remove the key, value pair stored in the map for this entry, and return the value.
    ///
    /// **NOTE:** This is equivalent to indexmap's
    /// [`OccupiedEntry::shift_remove`][ix::OccupiedEntry::shift_remove], and
    /// like [`Vec::remove`], the pair is removed by shifting all of the
    /// elements that follow it, preserving their relative order.
    /// **This perturbs the index of all of those elements!**
    ///
    /// Computes in **O(n)** time (average).
    pub fn remove(self) -> V {
        self.inner.shift_remove()
    }

    /// Remove and return the key, value pair stored in the map for this entry
    ///
    /// **NOTE:** This is equivalent to indexmap's
    /// [`OccupiedEntry::shift_remove_entry`][ix::OccupiedEntry::shift_remove_entry], and
    /// like [`Vec::remove`], the pair is removed by shifting all of the
    /// elements that follow it, preserving their relative order.
    /// **This perturbs the index of all of those elements!**
    ///
    /// Computes in **O(n)** time (average).
    pub fn remove_entry(self) -> (K, V) {
        self.inner.shift_remove_entry()
    }

    /// Remove the key, value pair stored in the map for this entry, and return the value.
    ///
    /// Like [`Vec::swap_remove`], the pair is removed by swapping it with
    /// the last element of the map and popping it off.
    /// **This perturbs the position of what used to be the last element!**
    ///
    /// Computes in **O(1)** time (average).
    pub fn swap_remove(self) -> V {
        self.inner.swap_remove()
    }

    /// Remove and return the key, value pair stored in the map for this entry
    ///
    /// Like [`Vec::swap_remove`], the pair is removed by swapping it with
    /// the last element of the map and popping it off.
    /// **This perturbs the position of what used to be the last element!**
    ///
    /// Computes in **O(1)** time (average).
    pub fn swap_remove_entry(self) -> (K, V) {
        self.inner.swap_remove_entry()
    }

    /// Moves the position of the entry to a new index
    /// by shifting all other entries in-between.
    ///
    /// This is equivalent to [`OrderMap::move_index`][`crate::OrderMap::move_index`]
    /// coming `from` the current [`.index()`][Self::index].
    ///
    /// * If `self.index() < to`, the other pairs will shift down while the targeted pair moves up.
    /// * If `self.index() > to`, the other pairs will shift up while the targeted pair moves down.
    ///
    /// ***Panics*** if `to` is out of bounds.
    ///
    /// Computes in **O(n)** time (average).
    pub fn move_index(self, to: usize) {
        self.inner.move_index(to);
    }

    /// Swaps the position of entry with another.
    ///
    /// This is equivalent to [`OrderMap::swap_indices`][`crate::OrderMap::swap_indices`]
    /// with the current [`.index()`][Self::index] as one of the two being swapped.
    ///
    /// ***Panics*** if the `other` index is out of bounds.
    ///
    /// Computes in **O(1)** time (average).
    pub fn swap_indices(self, other: usize) {
        self.inner.swap_indices(other);
    }
}

impl<K: fmt::Debug, V: fmt::Debug> fmt::Debug for OccupiedEntry<'_, K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OccupiedEntry")
            .field("key", self.key())
            .field("value", self.get())
            .finish()
    }
}

/// A view into a vacant entry in an [`OrderMap`][crate::OrderMap].
/// It is part of the [`Entry`] enum.
pub struct VacantEntry<'a, K, V> {
    pub(crate) inner: ix::VacantEntry<'a, K, V>,
}

impl<'a, K, V> VacantEntry<'a, K, V> {
    /// Return the index where a key-value pair may be inserted.
    pub fn index(&self) -> usize {
        self.inner.index()
    }

    /// Gets a reference to the key that was used to find the entry.
    pub fn key(&self) -> &K {
        self.inner.key()
    }

    /// Takes ownership of the key, leaving the entry vacant.
    pub fn into_key(self) -> K {
        self.inner.into_key()
    }

    /// Inserts the entry's key and the given value into the map, and returns a mutable reference
    /// to the value.
    pub fn insert(self, value: V) -> &'a mut V {
        self.inner.insert(value)
    }

    /// Inserts the entry's key and the given value into the map at its ordered
    /// position among sorted keys, and returns the new index and a mutable
    /// reference to the value.
    ///
    /// If the existing keys are **not** already sorted, then the insertion
    /// index is unspecified (like [`slice::binary_search`]), but the key-value
    /// pair is inserted at that position regardless.
    ///
    /// Computes in **O(n)** time (average).
    pub fn insert_sorted(self, value: V) -> (usize, &'a mut V)
    where
        K: Ord,
    {
        self.inner.insert_sorted(value)
    }

    /// Inserts the entry's key and the given value into the map at the given index,
    /// shifting others to the right, and returns a mutable reference to the value.
    ///
    /// ***Panics*** if `index` is out of bounds.
    ///
    /// Computes in **O(n)** time (average).
    pub fn shift_insert(self, index: usize, value: V) -> &'a mut V {
        self.inner.shift_insert(index, value)
    }
}

impl<K: fmt::Debug, V> fmt::Debug for VacantEntry<'_, K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("VacantEntry").field(self.key()).finish()
    }
}

/// A view into an occupied entry in an [`OrderMap`][crate::OrderMap] obtained by index.
///
/// This `struct` is created from the [`get_index_entry`][crate::OrderMap::get_index_entry] method.
pub struct IndexedEntry<'a, K, V> {
    pub(crate) inner: ix::IndexedEntry<'a, K, V>,
}

impl<'a, K, V> IndexedEntry<'a, K, V> {
    pub(super) fn new(inner: ix::IndexedEntry<'a, K, V>) -> Self {
        Self { inner }
    }

    /// Return the index of the key-value pair
    #[inline]
    pub fn index(&self) -> usize {
        self.inner.index()
    }

    /// Gets a reference to the entry's key in the map.
    pub fn key(&self) -> &K {
        self.inner.key()
    }

    /// Gets a reference to the entry's value in the map.
    pub fn get(&self) -> &V {
        self.inner.get()
    }

    /// Gets a mutable reference to the entry's value in the map.
    ///
    /// If you need a reference which may outlive the destruction of the
    /// `IndexedEntry` value, see [`into_mut`][Self::into_mut].
    pub fn get_mut(&mut self) -> &mut V {
        self.inner.get_mut()
    }

    /// Sets the value of the entry to `value`, and returns the entry's old value.
    pub fn insert(&mut self, value: V) -> V {
        self.inner.insert(value)
    }

    /// Converts into a mutable reference to the entry's value in the map,
    /// with a lifetime bound to the map itself.
    pub fn into_mut(self) -> &'a mut V {
        self.inner.into_mut()
    }

    /// Remove and return the key, value pair stored in the map for this entry
    ///
    /// **NOTE:** This is equivalent to indexmap's
    /// [`IndexedEntry::shift_remove_entry`][ix::IndexedEntry::shift_remove_entry], and
    /// like [`Vec::remove`], the pair is removed by shifting all of the
    /// elements that follow it, preserving their relative order.
    /// **This perturbs the index of all of those elements!**
    ///
    /// Computes in **O(n)** time (average).
    pub fn remove_entry(self) -> (K, V) {
        self.inner.shift_remove_entry()
    }

    /// Remove the key, value pair stored in the map for this entry, and return the value.
    ///
    /// **NOTE:** This is equivalent to indexmap's
    /// [`IndexedEntry::shift_remove`][ix::IndexedEntry::shift_remove], and
    /// like [`Vec::remove`], the pair is removed by shifting all of the
    /// elements that follow it, preserving their relative order.
    /// **This perturbs the index of all of those elements!**
    ///
    /// Computes in **O(n)** time (average).
    pub fn remove(self) -> V {
        self.inner.shift_remove()
    }

    /// Remove and return the key, value pair stored in the map for this entry
    ///
    /// Like [`Vec::swap_remove`], the pair is removed by swapping it with
    /// the last element of the map and popping it off.
    /// **This perturbs the position of what used to be the last element!**
    ///
    /// Computes in **O(1)** time (average).
    pub fn swap_remove_entry(self) -> (K, V) {
        self.inner.swap_remove_entry()
    }

    /// Remove the key, value pair stored in the map for this entry, and return the value.
    ///
    /// Like [`Vec::swap_remove`], the pair is removed by swapping it with
    /// the last element of the map and popping it off.
    /// **This perturbs the position of what used to be the last element!**
    ///
    /// Computes in **O(1)** time (average).
    pub fn swap_remove(self) -> V {
        self.inner.swap_remove()
    }

    /// Moves the position of the entry to a new index
    /// by shifting all other entries in-between.
    ///
    /// This is equivalent to [`OrderMap::move_index`][`crate::OrderMap::move_index`]
    /// coming `from` the current [`.index()`][Self::index].
    ///
    /// * If `self.index() < to`, the other pairs will shift down while the targeted pair moves up.
    /// * If `self.index() > to`, the other pairs will shift up while the targeted pair moves down.
    ///
    /// ***Panics*** if `to` is out of bounds.
    ///
    /// Computes in **O(n)** time (average).
    pub fn move_index(self, to: usize) {
        self.inner.move_index(to)
    }

    /// Swaps the position of entry with another.
    ///
    /// This is equivalent to [`OrderMap::swap_indices`][`crate::OrderMap::swap_indices`]
    /// with the current [`.index()`][Self::index] as one of the two being swapped.
    ///
    /// ***Panics*** if the `other` index is out of bounds.
    ///
    /// Computes in **O(1)** time (average).
    pub fn swap_indices(self, other: usize) {
        self.inner.swap_indices(other)
    }
}

impl<K: fmt::Debug, V: fmt::Debug> fmt::Debug for IndexedEntry<'_, K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("IndexedEntry")
            .field("index", &self.index())
            .field("key", self.key())
            .field("value", self.get())
            .finish()
    }
}
