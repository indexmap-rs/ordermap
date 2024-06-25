use super::OrderSet;
use indexmap::set::{IntoIter, Iter};

impl<'a, T, S> IntoIterator for &'a OrderSet<T, S> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

impl<T, S> IntoIterator for OrderSet<T, S> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}
