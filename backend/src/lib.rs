#![allow(unused_imports, unreachable_code, dead_code, unused_variables)]
extern crate clap;
#[macro_use]
extern crate debug_rs;
#[macro_use]
extern crate derive_builder;
extern crate num_cpus;
extern crate regex;
extern crate serde_yaml;

pub mod config;
mod detached;
mod app;
mod server;
mod cli;
pub mod model;

use cli::RuntimeContext;

pub fn run() {
    std::env::set_var("DEBUG", "*");
    let ctx = RuntimeContext::new();
    ctx.run();
}
