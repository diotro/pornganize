#![allow(unused_imports, unreachable_code, dead_code, unused_variables)]
use std::env;
use pornganize::run;

fn main() {
    if env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "INFO");
        #[cfg(debug_assertions)]
        std::env::set_var("RUST_LOG", "DEBUG");
    }
    pretty_env_logger::init();
    run();
}
