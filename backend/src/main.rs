#![allow(unused_imports, unreachable_code, dead_code, unused_variables)]

use pornganize::run;

fn main() {
    dotenv::dotenv().ok();
    pretty_env_logger::init();
    run();
}
