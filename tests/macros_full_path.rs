#[test]
fn test_create_map() {
    let _m = ordermap::ordermap! {
        1 => 2,
        7 => 1,
        2 => 2,
        3 => 3,
    };
}

#[test]
fn test_create_set() {
    let _s = ordermap::orderset! {
        1,
        7,
        2,
        3,
    };
}
