#![allow(unused_imports, unreachable_code, dead_code, unused_variables)]
#![warn(
    deprecated_in_future,
    keyword_idents,
    trivial_casts,
    unused_import_braces,
)]

pub mod model;
mod persist;
pub use persist::Database;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
