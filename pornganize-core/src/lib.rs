#![allow(unused_imports, unreachable_code, dead_code, unused_variables)]
#![warn(
    deprecated_in_future,
    keyword_idents,
    trivial_casts,
    unused_import_braces
)]
#[macro_use]
extern crate log;
extern crate actix;
extern crate num_cpus;
extern crate serde;

pub mod config;
pub mod util;
