#![cfg_attr(docsrs, doc(cfg(feature = "serde")))]

use crate::{OrderMap, OrderSet};
use core::hash::{BuildHasher, Hash};
use serde::de::value::{MapDeserializer, SeqDeserializer};
use serde::de::{Deserialize, Deserializer, Error, IntoDeserializer};
use serde::ser::{Serialize, Serializer};

impl<K, V, S> Serialize for OrderMap<K, V, S>
where
    K: Serialize,
    V: Serialize,
{
    fn serialize<T>(&self, serializer: T) -> Result<T::Ok, T::Error>
    where
        T: Serializer,
    {
        self.inner.serialize(serializer)
    }
}

impl<'de, K, V, S> Deserialize<'de> for OrderMap<K, V, S>
where
    K: Deserialize<'de> + Eq + Hash,
    V: Deserialize<'de>,
    S: Default + BuildHasher,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Self {
            inner: <_>::deserialize(deserializer)?,
        })
    }
}

impl<'de, K, V, S, E> IntoDeserializer<'de, E> for OrderMap<K, V, S>
where
    K: IntoDeserializer<'de, E> + Eq + Hash,
    V: IntoDeserializer<'de, E>,
    S: BuildHasher,
    E: Error,
{
    type Deserializer = MapDeserializer<'de, <Self as IntoIterator>::IntoIter, E>;

    fn into_deserializer(self) -> Self::Deserializer {
        self.inner.into_deserializer()
    }
}

impl<T, S> Serialize for OrderSet<T, S>
where
    T: Serialize,
{
    fn serialize<Se>(&self, serializer: Se) -> Result<Se::Ok, Se::Error>
    where
        Se: Serializer,
    {
        self.inner.serialize(serializer)
    }
}

impl<'de, T, S> Deserialize<'de> for OrderSet<T, S>
where
    T: Deserialize<'de> + Eq + Hash,
    S: Default + BuildHasher,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Self {
            inner: <_>::deserialize(deserializer)?,
        })
    }
}

impl<'de, T, S, E> IntoDeserializer<'de, E> for OrderSet<T, S>
where
    T: IntoDeserializer<'de, E> + Eq + Hash,
    S: BuildHasher,
    E: Error,
{
    type Deserializer = SeqDeserializer<<Self as IntoIterator>::IntoIter, E>;

    fn into_deserializer(self) -> Self::Deserializer {
        self.inner.into_deserializer()
    }
}
