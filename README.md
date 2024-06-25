# ordermap

[![build status](https://github.com/indexmap-rs/ordermap/workflows/Continuous%20integration/badge.svg?branch=master)](https://github.com/indexmap-rs/ordermap/actions)
[![crates.io](https://img.shields.io/crates/v/ordermap.svg)](https://crates.io/crates/ordermap)
[![docs](https://docs.rs/ordermap/badge.svg)](https://docs.rs/ordermap)
[![rustc](https://img.shields.io/badge/rust-1.63%2B-orange.svg)](https://img.shields.io/badge/rust-1.63%2B-orange.svg)

A pure-Rust hash table which preserves (in a limited sense) insertion order.

This crate implements compact map and set data-structures,
where the iteration order of the keys is independent from their hash or
value. It preserves insertion order in most mutating operations, and it
allows lookup of entries by either hash table key or numerical index.

Note: this crate was originally what became the `indexmap` crate, and
it was deprecated for a while in favor of that, but then `ordermap`
returned as a wrapper over `indexmap` with stronger ordering properties.

# Background

This was inspired by Python 3.6's new dict implementation (which remembers
the insertion order and is fast to iterate, and is compact in memory).

Some of those features were translated to Rust, and some were not. The results
were `ordermap` and `indexmap`, hash tables that have following properties:

- Order is **independent of hash function** and hash values of keys.
- Fast to iterate.
- Indexed in compact space.
- Preserves insertion order **as long** as you don't call `.swap_remove()`
  or other methods that explicitly change order.
  - In `ordermap`, the regular `.remove()` **does** preserve insertion order,
    equivalent to what `indexmap` calls `.shift_remove()`.
- Uses hashbrown for the inner table, just like Rust's libstd `HashMap` does.

Since its reintroduction in 0.5, `ordermap` has also used its entry order for
`PartialEq` and `Eq`, whereas `indexmap` considers the same entries in *any* order
to be equal for drop-in compatibility with `HashMap` semantices. Using the order
is faster, and also allows `ordermap` to implement `PartialOrd`, `Ord`, and `Hash`.

## Performance

`OrderMap` derives a couple of performance facts directly from how it is constructed,
which is roughly:

> A raw hash table of key-value indices, and a vector of key-value pairs.

- As a wrapper, `OrderMap` should maintain the same performance as `IndexMap`
  for most operations, with the main difference being the removal strategy.
- Iteration is very fast since it is on the dense key-values.
- Lookup is fast-ish because the initial 7-bit hash lookup uses SIMD, and indices are
  densely stored. Lookup also is slow-ish since the actual key-value pairs are stored
  separately. (Visible when cpu caches size is limiting.)
- In practice, `OrderMap` has been tested out as the hashmap in rustc in [PR45282] and
  the performance was roughly on par across the whole workload.
- If you want the properties of `OrderMap`, or its strongest performance points
  fits your workload, it might be the best hash table implementation.

[PR45282]: https://github.com/rust-lang/rust/pull/45282

# Recent Changes

See [RELEASES.md](https://github.com/indexmap-rs/ordermap/blob/master/RELEASES.md).
