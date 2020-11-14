#![allow(unused_imports, unreachable_code, dead_code, unused_variables)]
#![warn(
    deprecated_in_future,
    keyword_idents,
    trivial_casts,
    unused_import_braces,
)]
extern crate clap;
#[macro_use]
extern crate derive_builder;
#[macro_use]
extern crate log;
extern crate num_cpus;
extern crate regex;
extern crate serde_yaml;
extern crate pretty_env_logger;

pub mod config;
mod detached;
mod app;
mod server;
mod cli;
pub mod model;

use cli::RuntimeContext;

pub fn run() {
    let ctx = RuntimeContext::new();
    ctx.run();
}
