#![allow(unused_imports, unreachable_code, dead_code, unused_variables)]
#![warn(
    deprecated_in_future,
    keyword_idents,
    trivial_casts,
    unused_import_braces,
)]
extern crate clap;
#[macro_use]
extern crate cfg_if;
#[macro_use]
extern crate derive_builder;
#[macro_use]
extern crate log;
extern crate num_cpus;
extern crate regex;
extern crate serde_yaml;
extern crate dotenv;
extern crate pretty_env_logger;

pub mod config;
mod cli;
pub mod model;

mod clean;
mod merge;
mod server;

#[cfg(feature = "plugins")]
pub mod plugins;

use cli::{Opts, Command};

pub fn run() {
    let cli::Opts { subcommand, config } = cli::Opts::get();
    match subcommand {
        Command::Merge => merge::run_command(config),
        Command::Clean{what} => clean::run_command(config, what),
        Command::Server{command} => server::run_command(config, command),
    }
}
