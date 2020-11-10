#![allow(unused_imports, unreachable_code, dead_code)]
extern crate clap;
#[macro_use]
extern crate debug_rs;
extern crate regex;
extern crate serde_yaml;
use actix_web::{App as WebApp, HttpServer};

pub mod app;
pub mod config;

pub async fn run() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    let cfg = config::Config::new().unwrap();
    HttpServer::new(|| {
        WebApp::new().configure(app::configure_web_app)
        //.wrap(middleware::Logger::default())
    })
    .bind(cfg.listen.to_string())?
    .run()
    .await
}
