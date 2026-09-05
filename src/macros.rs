/// Create an [`OrderMap`][crate::OrderMap] from a list of key-value pairs
/// and a [`BuildHasherDefault`][core::hash::BuildHasherDefault]-wrapped custom hasher.
///
/// ## Example
///
/// ```
/// use ordermap::ordermap_with_default;
/// use fnv::FnvHasher;
///
/// let map = ordermap_with_default!{
///     FnvHasher;
///     "a" => 1,
///     "b" => 2,
/// };
/// assert_eq!(map["a"], 1);
/// assert_eq!(map["b"], 2);
/// assert_eq!(map.get("c"), None);
///
/// // "a" is the first key
/// assert_eq!(map.keys().next(), Some(&"a"));
/// ```
#[macro_export]
macro_rules! ordermap_with_default {
    ($H:ty; $($key:expr => $value:expr,)+) => { $crate::ordermap_with_default!($H; $($key => $value),+) };
    ($H:ty; $($key:expr => $value:expr),*) => {{
        #[allow(unused_mut)]
        let mut map = $crate::OrderMap::with_capacity_and_hasher(
            // Note: `stringify!($key)` is just here to consume the repetition,
            // but we throw away that string literal during constant evaluation.
            const { <[()]>::len(&[$({ stringify!($key); }),*]) },
            // Specify your custom `H` (must implement Default + Hasher) as the hasher:
            ::core::hash::BuildHasherDefault::<$H>::new(),
        );
        $(
            map.insert($key, $value);
        )*
        map
    }};
}

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
#[macro_export]
/// Create an [`OrderMap`][crate::OrderMap] from a list of key-value pairs
///
/// ## Example
///
/// ```
/// use ordermap::ordermap;
///
/// let map = ordermap!{
///     "a" => 1,
///     "b" => 2,
/// };
/// assert_eq!(map["a"], 1);
/// assert_eq!(map["b"], 2);
/// assert_eq!(map.get("c"), None);
///
/// // "a" is the first key
/// assert_eq!(map.keys().next(), Some(&"a"));
/// ```
macro_rules! ordermap {
    ($($key:expr => $value:expr,)+) => { $crate::ordermap!($($key => $value),+) };
    ($($key:expr => $value:expr),*) => {{
        let mut map = $crate::OrderMap::with_capacity(
            // Note: `stringify!($key)` is just here to consume the repetition,
            // but we throw away that string literal during constant evaluation.
            const { <[()]>::len(&[$({ stringify!($key); }),*]) },
        );
        $(
            map.insert($key, $value);
        )*
        map
    }};
}

/// Create an [`OrderSet`][crate::OrderSet] from a list of values
/// and a [`BuildHasherDefault`][core::hash::BuildHasherDefault]-wrapped custom hasher.
///
/// ## Example
///
/// ```
/// use ordermap::orderset_with_default;
/// use fnv::FnvHasher;
///
/// let set = orderset_with_default!{
///     FnvHasher;
///     "a",
///     "b",
/// };
/// assert!(set.contains("a"));
/// assert!(set.contains("b"));
/// assert!(!set.contains("c"));
///
/// // "a" is the first value
/// assert_eq!(set.iter().next(), Some(&"a"));
/// ```
#[macro_export]
macro_rules! orderset_with_default {
    ($H:ty; $($value:expr,)+) => { $crate::orderset_with_default!($H; $($value),+) };
    ($H:ty; $($value:expr),*) => {{
        #[allow(unused_mut)]
        let mut set = $crate::OrderSet::with_capacity_and_hasher(
            // Note: `stringify!($value)` is just here to consume the repetition,
            // but we throw away that string literal during constant evaluation.
            const { <[()]>::len(&[$({ stringify!($value); }),*]) },
            // Specify your custom `H` (must implement Default + Hash) as the hasher:
            ::core::hash::BuildHasherDefault::<$H>::new(),
        );
        $(
            set.insert($value);
        )*
        set
    }};
}

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
#[macro_export]
/// Create an [`OrderSet`][crate::OrderSet] from a list of values
///
/// ## Example
///
/// ```
/// use ordermap::orderset;
///
/// let set = orderset!{
///     "a",
///     "b",
/// };
/// assert!(set.contains("a"));
/// assert!(set.contains("b"));
/// assert!(!set.contains("c"));
///
/// // "a" is the first value
/// assert_eq!(set.iter().next(), Some(&"a"));
/// ```
macro_rules! orderset {
    ($($value:expr,)+) => { $crate::orderset!($($value),+) };
    ($($value:expr),*) => {{
        let mut set = $crate::OrderSet::with_capacity(
            // Note: `stringify!($value)` is just here to consume the repetition,
            // but we throw away that string literal during constant evaluation.
            const { <[()]>::len(&[$({ stringify!($value); }),*]) },
        );
        $(
            set.insert($value);
        )*
        set
    }};
}
