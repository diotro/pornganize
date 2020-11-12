#![allow(unused_imports, unreachable_code, dead_code, unused_variables)]
#[macro_use]
extern crate clap;
#[macro_use]
extern crate debug_rs;
use std::sync::mpsc;
use std::thread;

use actix_web::{middleware, rt::System, App as WebApp, HttpServer};

use pornganize::app;
use pornganize::config;

//#[actix_web::main]
//async fn main() -> std::io::Result<()> {
////pornganize::run()
//std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
//let cfg = config::Config::new().unwrap();
//HttpServer::new(|| {
//WebApp::new()
//.configure(app::configure_web_app)
//.wrap(middleware::Logger::default())
//.wrap(middleware::Compress::default())
//})
//.bind(cfg.server.listen.to_string())?
//.shutdown_timeout(10)
//.workers(cfg.server.workers)
//.run()
//.await
//}
fn main() {}
