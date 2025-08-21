#![cfg_attr(docsrs, doc(cfg(feature = "sval")))]

use crate::{OrderMap, OrderSet};
use sval::{Stream, Value};

impl<K: Value, V: Value, S> Value for OrderMap<K, V, S> {
    fn stream<'sval, ST: Stream<'sval> + ?Sized>(&'sval self, stream: &mut ST) -> sval::Result {
        self.inner.stream(stream)
    }
}

impl<K: Value, S> Value for OrderSet<K, S> {
    fn stream<'sval, ST: Stream<'sval> + ?Sized>(&'sval self, stream: &mut ST) -> sval::Result {
        self.inner.stream(stream)
    }
}
