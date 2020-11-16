use std::path::PathBuf;
use log::info;

use pornganize::{
    config::{Config, Listen},
    util,
};
use pornganize_web::run_server;

use super::opts::{Opts, RunIn, ServerCommand};
use super::detached::{RunOptions, run_detached};

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
            run_detached(run_opts, move || {
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
