#![forbid(unsafe_code)]
#![warn(rust_2018_idioms)]
#![no_std]

//! [`OrderMap`] is a hash table where the iteration order of the key-value
//! pairs is independent of the hash values of the keys.
//!
//! [`OrderSet`] is a corresponding hash set using the same implementation and
//! with similar properties.
//!
//! ### Highlights
//!
//! [`OrderMap`] and [`OrderSet`] are drop-in compatible with the std `HashMap`
//! and `HashSet`, but they also have some features of note:
//!
//! - The ordering semantics (see their documentation for details)
//! - Sorting methods and the [`.pop()`][OrderMap::pop] methods.
//! - The [`Equivalent`] trait, which offers more flexible equality definitions
//!   between borrowed and owned versions of keys.
//! - The [`MutableKeys`][map::MutableKeys] trait, which gives opt-in mutable
//!   access to map keys, and [`MutableValues`][set::MutableValues] for sets.
//!
//! ### Feature Flags
//!
//! To reduce the amount of compiled code in the crate by default, certain
//! features are gated behind [feature flags]. These allow you to opt in to (or
//! out of) functionality. Below is a list of the features available in this
//! crate.
//!
//! * `std`: Enables features which require the Rust standard library. For more
//!   information see the section on [`no_std`].
//! * `rayon`: Enables parallel iteration and other parallel methods.
//! * `serde`: Adds implementations for [`Serialize`] and [`Deserialize`]
//!   to [`OrderMap`] and [`OrderSet`]. Alternative implementations for
//!   (de)serializing [`OrderMap`] as an ordered sequence are available in the
//!   [`map::serde_seq`] module.
//! * `borsh`: Adds implementations for [`BorshSerialize`] and [`BorshDeserialize`]
//!   to [`OrderMap`] and [`OrderSet`].
//! * `arbitrary`: Adds implementations for the [`arbitrary::Arbitrary`] trait
//!   to [`OrderMap`] and [`OrderSet`].
//! * `quickcheck`: Adds implementations for the [`quickcheck::Arbitrary`] trait
//!   to [`OrderMap`] and [`OrderSet`].
//!
//! _Note: only the `std` feature is enabled by default._
//!
//! [feature flags]: https://doc.rust-lang.org/cargo/reference/manifest.html#the-features-section
//! [`no_std`]: #no-standard-library-targets
//! [`Serialize`]: `::serde::Serialize`
//! [`Deserialize`]: `::serde::Deserialize`
//! [`BorshSerialize`]: `::borsh::BorshSerialize`
//! [`BorshDeserialize`]: `::borsh::BorshDeserialize`
//! [`arbitrary::Arbitrary`]: `::arbitrary::Arbitrary`
//! [`quickcheck::Arbitrary`]: `::quickcheck::Arbitrary`
//!
//! ### Alternate Hashers
//!
//! [`OrderMap`] and [`OrderSet`] have a default hasher type
//! [`S = RandomState`][std::collections::hash_map::RandomState],
//! just like the standard `HashMap` and `HashSet`, which is resistant to
//! HashDoS attacks but not the most performant. Type aliases can make it easier
//! to use alternate hashers:
//!
//! ```
//! use fnv::FnvBuildHasher;
//! use fxhash::FxBuildHasher;
//! use ordermap::{OrderMap, OrderSet};
//!
//! type FnvOrderMap<K, V> = OrderMap<K, V, FnvBuildHasher>;
//! type FnvOrderSet<T> = OrderSet<T, FnvBuildHasher>;
//!
//! type FxOrderMap<K, V> = OrderMap<K, V, FxBuildHasher>;
//! type FxOrderSet<T> = OrderSet<T, FxBuildHasher>;
//!
//! let std: OrderSet<i32> = (0..100).collect();
//! let fnv: FnvOrderSet<i32> = (0..100).collect();
//! let fx: FxOrderSet<i32> = (0..100).collect();
//! assert_eq!(std, fnv);
//! assert_eq!(std, fx);
//! ```
//!
//! ### Rust Version
//!
//! This version of ordermap requires Rust 1.63 or later.
//!
//! The ordermap 0.x release series will use a carefully considered version
//! upgrade policy, where in a later 0.x version, we will raise the minimum
//! required Rust version.
//!
//! ## No Standard Library Targets
//!
//! This crate supports being built without `std`, requiring `alloc` instead.
//! This is chosen by disabling the default "std" cargo feature, by adding
//! `default-features = false` to your dependency specification.
//!
//! - Creating maps and sets using [`new`][OrderMap::new] and
//!   [`with_capacity`][OrderMap::with_capacity] is unavailable without `std`.
//!   Use methods [`OrderMap::default`], [`with_hasher`][OrderMap::with_hasher],
//!   [`with_capacity_and_hasher`][OrderMap::with_capacity_and_hasher] instead.
//!   A no-std compatible hasher will be needed as well, for example
//!   from the crate `twox-hash`.
//! - Macros [`ordermap!`] and [`orderset!`] are unavailable without `std`.

#![cfg_attr(docsrs, feature(doc_cfg))]

extern crate alloc;

#[cfg(feature = "std")]
#[cfg_attr(test, macro_use)]
extern crate std;

mod arbitrary;
#[macro_use]
mod macros;
#[cfg(feature = "borsh")]
mod borsh;
#[cfg(feature = "serde")]
mod serde;

pub mod map;
pub mod set;

pub use crate::map::OrderMap;
pub use crate::set::OrderSet;
pub use indexmap::{Equivalent, TryReserveError};
