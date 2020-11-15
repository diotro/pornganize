#![allow(unused_imports, unreachable_code, dead_code, unused_variables)]
use std::path::PathBuf;
use std::sync::mpsc::{self, Receiver};
use std::thread::{Builder as ThreadBuilder, JoinHandle};

use actix_web::{dev::Server, middleware, rt, App as WebApp, HttpServer};
use log::info;

use super::cli::{Opts, RunIn, ServerCommand};
use super::config::{Config, Listen};

mod app;
mod detached;

use detached::RunOptions;

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
    let mut sys = rt::System::builder()
        .name("pornganize")
        .stop_on_panic(true)
        .build();
    let srv = create_server(listen, workers);
    sys.block_on(srv)
}

fn start_server(
    mut config: Config,
    port: Option<u32>,
    workers: Option<usize>,
    pid_file: Option<PathBuf>,
    run_in: RunIn,
) {
    if let Some(p) = port {
        config.server.listen.port = p;
    }
    if let Some(w) = workers {
        config.server.workers = w;
    }
    if let Some(f) = pid_file {
        config.server.pid_file = Some(f);
    }
    info!(
        "Starting {} server version {}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );
    match run_in {
        RunIn::Foreground => run_server(&config.server.listen, config.server.workers).unwrap(),
        RunIn::Background => {
            let run_opts = RunOptions::from(&config);
            detached::run_detached(run_opts, move || {
                let s = run_server(&config.server.listen, config.server.workers).unwrap();
            })
            .unwrap();
        }
    }
}

fn stop_server(config: Config, pid_file: Option<PathBuf>) {}

pub fn run_command(config: Config, subcommand: ServerCommand) {
    match subcommand {
        ServerCommand::Start {
            port,
            workers,
            pid_file,
            run_in,
        } => start_server(config, port, workers, pid_file, run_in),
        ServerCommand::Stop { pid_file } => stop_server(config, pid_file),
    }
}
