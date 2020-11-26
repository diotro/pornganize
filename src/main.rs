#![allow(unused_imports, unreachable_code, dead_code, unused_variables)]
#![warn(
    deprecated_in_future,
    keyword_idents,
    trivial_casts,
    unused_import_braces,
)]
#[macro_use]
extern crate cfg_if;
#[macro_use]
extern crate derive_builder;

mod clean;
mod merge;
#[cfg(debug_assertions)]
mod sandbox;
#[cfg(feature = "web")]
mod detached;
#[cfg(feature = "web")]
mod server;

mod opts;

use opts::{Opts, Command};

fn main() {
    dotenv::dotenv().ok();
    pretty_env_logger::init();
    let Opts { subcommand, config } = Opts::get();
    match subcommand {
        Command::Merge => merge::run_command(config),
        Command::Clean{what} => clean::run_command(config, what),
        #[cfg(feature = "web")]
        Command::Server{command} => server::run_command(config, command),
        #[cfg(debug_assertions)]
        Command::Sandbox => sandbox::run_command(config),
    }
}
