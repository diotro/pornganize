#![allow(unused_imports, unreachable_code, dead_code, unused_variables)]
#![warn(
    deprecated_in_future,
    keyword_idents,
    trivial_casts,
    unused_import_braces,
)]
extern crate actix_web;
extern crate log;
extern crate pornganize;

use std::path::PathBuf;
use std::sync::mpsc::{self, Receiver};
use std::thread::{Builder as ThreadBuilder, JoinHandle};

use actix_web::{dev::Server, middleware, rt, App as WebApp, HttpServer};
use log::info;

use pornganize::{
    config::Listen,
    util,
};

mod app;

fn create_server(listen: &Listen, workers: usize) -> Server {
    HttpServer::new(|| {
        WebApp::new()
            .configure(app::configure_web_app)
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
    })
    .bind(listen.to_string())
    .unwrap()
    .shutdown_timeout(10)
    .workers(workers)
    .run()
}

pub fn run_server(listen: &Listen, workers: usize) -> std::io::Result<()> {
    let mut sys = util::create_actix_runner();
    let srv = create_server(listen, workers);
    sys.block_on(srv)
}
