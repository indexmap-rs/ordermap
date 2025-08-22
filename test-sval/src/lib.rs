#![cfg(test)]

use fnv::FnvBuildHasher;
use ordermap::{ordermap, orderset, OrderMap, OrderSet};
use sval_test::{assert_tokens, Token};

#[test]
fn test_sval_map() {
    let map = ordermap! { 1 => 2, 3 => 4 };
    assert_tokens(
        &map,
        &[
            Token::MapBegin(Some(2)),
            Token::MapKeyBegin,
            Token::I32(1),
            Token::MapKeyEnd,
            Token::MapValueBegin,
            Token::I32(2),
            Token::MapValueEnd,
            Token::MapKeyBegin,
            Token::I32(3),
            Token::MapKeyEnd,
            Token::MapValueBegin,
            Token::I32(4),
            Token::MapValueEnd,
            Token::MapEnd,
        ],
    );
}

#[test]
fn test_sval_set() {
    let set = orderset! { 1, 2, 3, 4 };
    assert_tokens(
        &set,
        &[
            Token::SeqBegin(Some(4)),
            Token::SeqValueBegin,
            Token::I32(1),
            Token::SeqValueEnd,
            Token::SeqValueBegin,
            Token::I32(2),
            Token::SeqValueEnd,
            Token::SeqValueBegin,
            Token::I32(3),
            Token::SeqValueEnd,
            Token::SeqValueBegin,
            Token::I32(4),
            Token::SeqValueEnd,
            Token::SeqEnd,
        ],
    );
}

#[test]
fn test_sval_map_fnv_hasher() {
    let mut map: OrderMap<i32, i32, FnvBuildHasher> = Default::default();
    map.insert(1, 2);
    map.insert(3, 4);
    assert_tokens(
        &map,
        &[
            Token::MapBegin(Some(2)),
            Token::MapKeyBegin,
            Token::I32(1),
            Token::MapKeyEnd,
            Token::MapValueBegin,
            Token::I32(2),
            Token::MapValueEnd,
            Token::MapKeyBegin,
            Token::I32(3),
            Token::MapKeyEnd,
            Token::MapValueBegin,
            Token::I32(4),
            Token::MapValueEnd,
            Token::MapEnd,
        ],
    );
}

#[test]
fn test_sval_set_fnv_hasher() {
    let mut set: OrderSet<i32, FnvBuildHasher> = Default::default();
    set.extend(1..5);
    assert_tokens(
        &set,
        &[
            Token::SeqBegin(Some(4)),
            Token::SeqValueBegin,
            Token::I32(1),
            Token::SeqValueEnd,
            Token::SeqValueBegin,
            Token::I32(2),
            Token::SeqValueEnd,
            Token::SeqValueBegin,
            Token::I32(3),
            Token::SeqValueEnd,
            Token::SeqValueBegin,
            Token::I32(4),
            Token::SeqValueEnd,
            Token::SeqEnd,
        ],
    );
}
