#![no_std]

use core::hash::BuildHasherDefault;
use core::hash::Hasher;

use ordermap::OrderMap;
use ordermap::OrderSet;

#[derive(Default)]
struct BadHasher(u64);

impl Hasher for BadHasher {
    fn finish(&self) -> u64 {
        self.0
    }
    fn write(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.0 += byte as u64
        }
    }
}

type Map<K, V> = OrderMap<K, V, BuildHasherDefault<BadHasher>>;
type Set<T> = OrderSet<T, BuildHasherDefault<BadHasher>>;

pub fn test_compile() {
    let mut map = Map::default();
    map.insert(1, 1);
    map.insert(2, 4);
    for (_, _) in map.iter() {}

    let _map2 = Map::from_iter(Some((1, 1)));

    let mut set = Set::default();
    set.insert("a");
}
