#![allow(unused_imports, unreachable_code, dead_code, unused_variables)]
#![warn(
    deprecated_in_future,
    keyword_idents,
    trivial_casts,
    unused_import_braces
)]
#[macro_use]
extern crate log;

mod runner;

pub mod sections;
pub mod config;
pub mod db;
pub mod plugins;
pub mod util;

#[doc(inline)]
pub use runner::Runner;
#[doc(inline)]
pub use config::Config;
#[doc(inline)]
pub use db::{Database, ItemType};
#[doc(inline)]
pub use sections::Section;
