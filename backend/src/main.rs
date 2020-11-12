#![allow(unused_imports, unreachable_code, dead_code, unused_variables)]
#[macro_use]
extern crate clap;
#[macro_use]
extern crate debug_rs;
pub mod cli;
pub mod config;

use clap::{load_yaml, App as CliApp, Arg, ArgMatches};
use cli::RuntimeContext;

fn main() {
    std::env::set_var("DEBUG", "*");
    let ctx = RuntimeContext::new();
    println!("{}", ctx.config.server.listen)
    //ctx.run();
}
