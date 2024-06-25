#[cfg(feature = "arbitrary")]
#[cfg_attr(docsrs, doc(cfg(feature = "arbitrary")))]
mod impl_arbitrary {
    use crate::{OrderMap, OrderSet};
    use arbitrary::{Arbitrary, Result, Unstructured};
    use core::hash::{BuildHasher, Hash};
    use indexmap::{IndexMap, IndexSet};

    impl<'a, K, V, S> Arbitrary<'a> for OrderMap<K, V, S>
    where
        K: Arbitrary<'a> + Hash + Eq,
        V: Arbitrary<'a>,
        S: BuildHasher + Default,
    {
        fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
            IndexMap::arbitrary(u).map(|inner| Self { inner })
        }

        fn arbitrary_take_rest(u: Unstructured<'a>) -> Result<Self> {
            IndexMap::arbitrary_take_rest(u).map(|inner| Self { inner })
        }
    }

    impl<'a, T, S> Arbitrary<'a> for OrderSet<T, S>
    where
        T: Arbitrary<'a> + Hash + Eq,
        S: BuildHasher + Default,
    {
        fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
            IndexSet::arbitrary(u).map(|inner| Self { inner })
        }

        fn arbitrary_take_rest(u: Unstructured<'a>) -> Result<Self> {
            IndexSet::arbitrary_take_rest(u).map(|inner| Self { inner })
        }
    }
}

#[cfg(feature = "quickcheck")]
#[cfg_attr(docsrs, doc(cfg(feature = "quickcheck")))]
mod impl_quickcheck {
    use crate::{OrderMap, OrderSet};
    use alloc::boxed::Box;
    use core::hash::{BuildHasher, Hash};
    use indexmap::{IndexMap, IndexSet};
    use quickcheck::{Arbitrary, Gen};

    impl<K, V, S> Arbitrary for OrderMap<K, V, S>
    where
        K: Arbitrary + Hash + Eq,
        V: Arbitrary,
        S: BuildHasher + Default + Clone + 'static,
    {
        fn arbitrary(g: &mut Gen) -> Self {
            Self {
                inner: IndexMap::arbitrary(g),
            }
        }

        fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
            Box::new(self.inner.shrink().map(|inner| Self { inner }))
        }
    }

    impl<T, S> Arbitrary for OrderSet<T, S>
    where
        T: Arbitrary + Hash + Eq,
        S: BuildHasher + Default + Clone + 'static,
    {
        fn arbitrary(g: &mut Gen) -> Self {
            Self {
                inner: IndexSet::arbitrary(g),
            }
        }

        fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
            Box::new(self.inner.shrink().map(|inner| Self { inner }))
        }
    }
}
