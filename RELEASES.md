# Releases

## 0.5.1

- Added trait `MutableEntryKey` for opt-in mutable access to map entry keys.
- Added method `MutableKeys::iter_mut2` for opt-in mutable iteration of map
  keys and values.

## 0.5.0

- Reinstated `ordermap` as a crate that wraps `indexmap` with stronger
  ordering semantics. It **does** consider order for `PartialEq` and `Eq`,
  also adding implementations of `PartialOrd`, `Ord`, and `Hash`. Methods
  like `remove` use the semantics of indexmap's `shift_remove`.

## 0.4.2

- Inserted more deprecation information in the documentation.
  Note: the crate ordermap has been renamed with no change in functionality
  to indexmap; please use it under its new name.

## 0.4.1

- Renamed crate to `indexmap`; the `ordermap` crate is now deprecated
  and the types `OrderMap/Set` now have a deprecation notice.

## 0.4.0

- This is the last release series for this `ordermap` under that name,
  because the crate is **going to be renamed** to `indexmap` (with types
  `IndexMap`, `IndexSet`) and no change in functionality!
- The map and its associated structs moved into the `map` submodule of the
  crate, so that the map and set are symmetric

    + The iterators, `Entry` and other structs are now under `ordermap::map::`

- Internally refactored `OrderMap<K, V, S>` so that all the main algorithms
  (insertion, lookup, removal etc) that don't use the `S` parameter (the
  hasher) are compiled without depending on `S`, which reduces generics bloat.

- `Entry<K, V>` no longer has a type parameter `S`, which is just like
  the standard `HashMap`'s entry.

- Minimum Rust version requirement increased to Rust 1.18

## 0.3.5

- Documentation improvements

## 0.3.4

- The `.retain()` methods for `OrderMap` and `OrderSet` now
  traverse the elements in order, and the retained elements **keep their order**
- Added new methods `.sort_by()`, `.sort_keys()` to `OrderMap` and
  `.sort_by()`, `.sort()` to `OrderSet`. These methods allow you to
  sort the maps in place efficiently.

## 0.3.3

- Document insertion behaviour better by @lucab
- Updated dependences (no feature changes) by @ignatenkobrain

## 0.3.2

- Add `OrderSet` by @cuviper!
- `OrderMap::drain` is now (too) a double ended iterator.

## 0.3.1

- In all ordermap iterators, forward the `collect` method to the underlying
  iterator as well.
- Add crates.io categories.

## 0.3.0

- The methods `get_pair`, `get_pair_index` were both replaced by
  `get_full` (and the same for the mutable case).
- Method `swap_remove_pair` replaced by `swap_remove_full`.
- Add trait `MutableKeys` for opt-in mutable key access. Mutable key access
  is only possible through the methods of this extension trait.
- Add new trait `Equivalent` for key equivalence. This extends the
  `Borrow` trait mechanism for `OrderMap::get` in a backwards compatible
  way, just some minor type inference related issues may become apparent.
  See [#10] for more information.
- Implement `Extend<(&K, &V)>` by @xfix.

[#10]: https://github.com/indexmap-rs/indexmap/pull/10

## 0.2.13

- Fix deserialization to support custom hashers by @Techcable.
- Add methods `.index()` on the entry types by @garro95.

## 0.2.12

- Add methods `.with_hasher()`, `.hasher()`.

## 0.2.11

- Support `ExactSizeIterator` for the iterators. By @Binero.
- Use `Box<[Pos]>` internally, saving a word in the `OrderMap` struct.
- Serde support, with crate feature `"serde-1"`. By @xfix.

## 0.2.10

- Add iterator `.drain(..)` by @stevej.

## 0.2.9

- Add method `.is_empty()` by @overvenus.
- Implement `PartialEq, Eq` by @overvenus.
- Add method `.sorted_by()`.

## 0.2.8

- Add iterators `.values()` and `.values_mut()`.
- Fix compatibility with 32-bit platforms.

## 0.2.7

- Add `.retain()`.

## 0.2.6

- Add `OccupiedEntry::remove_entry` and other minor entry methods,
  so that it now has all the features of `HashMap`'s entries.

## 0.2.5

- Improved `.pop()` slightly.

## 0.2.4

- Improved performance of `.insert()` ([#3]) by @pczarn.

[#3]: https://github.com/indexmap-rs/indexmap/pull/3

## 0.2.3

- Generalize `Entry` for now, so that it works on hashmaps with non-default
  hasher. However, there's a lingering compat issue since libstd `HashMap`
  does not parameterize its entries by the hasher (`S` typarm).
- Special case some iterator methods like `.nth()`.

## 0.2.2

- Disable the verbose `Debug` impl by default.

## 0.2.1

- Fix doc links and clarify docs.

## 0.2.0

- Add more `HashMap` methods & compat with its API.
- Experimental support for `.entry()` (the simplest parts of the API).
- Add `.reserve()` (placeholder impl).
- Add `.remove()` as synonym for `.swap_remove()`.
- Changed `.insert()` to swap value if the entry already exists, and
  return `Option`.
- Experimental support as an *indexed* hash map! Added methods
  `.get_index()`, `.get_index_mut()`, `.swap_remove_index()`,
  `.get_pair_index()`, `.get_pair_index_mut()`.

## 0.1.2

- Implement the 32/32 split idea for `Pos` which improves cache utilization
  and lookup performance.

## 0.1.1

- Initial release.
