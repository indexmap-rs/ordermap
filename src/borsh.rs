#![cfg_attr(docsrs, doc(cfg(feature = "borsh")))]

use crate::{OrderMap, OrderSet};
use borsh::io::{Read, Result, Write};
use borsh::{BorshDeserialize, BorshSerialize};
use core::hash::BuildHasher;
use core::hash::Hash;

impl<K, V, S> BorshSerialize for OrderMap<K, V, S>
where
    K: BorshSerialize,
    V: BorshSerialize,
{
    #[inline]
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        self.inner.serialize(writer)
    }
}

impl<K, V, S> BorshDeserialize for OrderMap<K, V, S>
where
    K: BorshDeserialize + Eq + Hash,
    V: BorshDeserialize,
    S: BuildHasher + Default,
{
    #[inline]
    fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self> {
        Ok(Self {
            inner: <_>::deserialize_reader(reader)?,
        })
    }
}

impl<T, S> BorshSerialize for OrderSet<T, S>
where
    T: BorshSerialize,
{
    #[inline]
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        self.inner.serialize(writer)
    }
}

impl<T, S> BorshDeserialize for OrderSet<T, S>
where
    T: BorshDeserialize + Eq + Hash,
    S: BuildHasher + Default,
{
    #[inline]
    fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self> {
        Ok(Self {
            inner: <_>::deserialize_reader(reader)?,
        })
    }
}

#[cfg(test)]
mod borsh_tests {
    use super::*;

    #[test]
    fn map_borsh_roundtrip() {
        let original_map: OrderMap<i32, i32> = {
            let mut map = OrderMap::new();
            map.insert(1, 2);
            map.insert(3, 4);
            map.insert(5, 6);
            map
        };
        let serialized_map = borsh::to_vec(&original_map).unwrap();
        let deserialized_map: OrderMap<i32, i32> =
            BorshDeserialize::try_from_slice(&serialized_map).unwrap();
        assert_eq!(original_map, deserialized_map);
    }

    #[test]
    fn set_borsh_roundtrip() {
        let original_map: OrderSet<i32> = [1, 2, 3, 4, 5, 6].into_iter().collect();
        let serialized_map = borsh::to_vec(&original_map).unwrap();
        let deserialized_map: OrderSet<i32> =
            BorshDeserialize::try_from_slice(&serialized_map).unwrap();
        assert_eq!(original_map, deserialized_map);
    }
}
