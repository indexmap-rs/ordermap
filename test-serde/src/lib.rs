#![cfg(test)]

use fnv::FnvBuildHasher;
use ordermap::{ordermap, orderset, OrderMap, OrderSet};
use serde::{Deserialize, Serialize};
use serde_test::{assert_tokens, Token};

#[test]
fn test_serde_map() {
    let map = ordermap! { 1 => 2, 3 => 4 };
    assert_tokens(
        &map,
        &[
            Token::Map { len: Some(2) },
            Token::I32(1),
            Token::I32(2),
            Token::I32(3),
            Token::I32(4),
            Token::MapEnd,
        ],
    );
}

#[test]
fn test_serde_set() {
    let set = orderset! { 1, 2, 3, 4 };
    assert_tokens(
        &set,
        &[
            Token::Seq { len: Some(4) },
            Token::I32(1),
            Token::I32(2),
            Token::I32(3),
            Token::I32(4),
            Token::SeqEnd,
        ],
    );
}

#[test]
fn test_serde_map_fnv_hasher() {
    let mut map: OrderMap<i32, i32, FnvBuildHasher> = Default::default();
    map.insert(1, 2);
    map.insert(3, 4);
    assert_tokens(
        &map,
        &[
            Token::Map { len: Some(2) },
            Token::I32(1),
            Token::I32(2),
            Token::I32(3),
            Token::I32(4),
            Token::MapEnd,
        ],
    );
}

#[test]
fn test_serde_set_fnv_hasher() {
    let mut set: OrderSet<i32, FnvBuildHasher> = Default::default();
    set.extend(1..5);
    assert_tokens(
        &set,
        &[
            Token::Seq { len: Some(4) },
            Token::I32(1),
            Token::I32(2),
            Token::I32(3),
            Token::I32(4),
            Token::SeqEnd,
        ],
    );
}

#[test]
fn test_serde_seq_map() {
    #[derive(Debug, Deserialize, Serialize)]
    #[serde(transparent)]
    struct SeqOrderMap {
        #[serde(with = "ordermap::map::serde_seq")]
        map: OrderMap<i32, i32>,
    }

    impl PartialEq for SeqOrderMap {
        fn eq(&self, other: &Self) -> bool {
            // explicitly compare items in order
            self.map.iter().eq(&other.map)
        }
    }

    let map = ordermap! { 1 => 2, 3 => 4, -1 => -2, -3 => -4 };
    assert_tokens(
        &SeqOrderMap { map },
        &[
            Token::Seq { len: Some(4) },
            Token::Tuple { len: 2 },
            Token::I32(1),
            Token::I32(2),
            Token::TupleEnd,
            Token::Tuple { len: 2 },
            Token::I32(3),
            Token::I32(4),
            Token::TupleEnd,
            Token::Tuple { len: 2 },
            Token::I32(-1),
            Token::I32(-2),
            Token::TupleEnd,
            Token::Tuple { len: 2 },
            Token::I32(-3),
            Token::I32(-4),
            Token::TupleEnd,
            Token::SeqEnd,
        ],
    );
}
