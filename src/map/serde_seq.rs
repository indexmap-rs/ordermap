//! Functions to serialize and deserialize an [`OrderMap`] as an ordered sequence.
//!
//! The default `serde` implementation serializes `OrderMap` as a normal map,
//! but there is no guarantee that serialization formats will preserve the order
//! of the key-value pairs. This module serializes `OrderMap` as a sequence of
//! `(key, value)` elements instead, in order.
//!
//! This module may be used in a field attribute for derived implementations:
//!
//! ```
//! # use ordermap::OrderMap;
//! # use serde_derive::{Deserialize, Serialize};
//! #[derive(Deserialize, Serialize)]
//! struct Data {
//!     #[serde(with = "ordermap::map::serde_seq")]
//!     map: OrderMap<i32, u64>,
//!     // ...
//! }
//! ```

use crate::OrderMap;
use core::hash::{BuildHasher, Hash};
use indexmap::map::serde_seq as ix;
use serde::de::{Deserialize, Deserializer};
use serde::ser::{Serialize, Serializer};

/// Serializes an [`OrderMap`] as an ordered sequence.
///
/// This function may be used in a field attribute for deriving [`Serialize`]:
///
/// ```
/// # use ordermap::OrderMap;
/// # use serde_derive::Serialize;
/// #[derive(Serialize)]
/// struct Data {
///     #[serde(serialize_with = "ordermap::map::serde_seq::serialize")]
///     map: OrderMap<i32, u64>,
///     // ...
/// }
/// ```
pub fn serialize<K, V, S, T>(map: &OrderMap<K, V, S>, serializer: T) -> Result<T::Ok, T::Error>
where
    K: Serialize,
    V: Serialize,
    T: Serializer,
{
    ix::serialize(&map.inner, serializer)
}

/// Deserializes an [`OrderMap`] from an ordered sequence.
///
/// This function may be used in a field attribute for deriving [`Deserialize`]:
///
/// ```
/// # use ordermap::OrderMap;
/// # use serde_derive::Deserialize;
/// #[derive(Deserialize)]
/// struct Data {
///     #[serde(deserialize_with = "ordermap::map::serde_seq::deserialize")]
///     map: OrderMap<i32, u64>,
///     // ...
/// }
/// ```
pub fn deserialize<'de, D, K, V, S>(deserializer: D) -> Result<OrderMap<K, V, S>, D::Error>
where
    D: Deserializer<'de>,
    K: Deserialize<'de> + Eq + Hash,
    V: Deserialize<'de>,
    S: Default + BuildHasher,
{
    Ok(OrderMap {
        inner: ix::deserialize(deserializer)?,
    })
}
