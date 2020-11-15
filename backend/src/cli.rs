extern crate serde_yaml;

use clap::{load_yaml, App as CliApp, Arg, ArgMatches};
use std::fs::File;
use super::{server, config::Config};
use super::detached::{self, RunOptions};

trait Runnable {
    fn run(&self, ctx: &RuntimeContext);
    fn modify_config(&self, config: &mut Config) {}
}

trait Command: Runnable {
    fn from_args(args: &ArgMatches) -> Self;
}

#[derive(Debug)]
pub enum CleanWhat {
    Games,
    Images,
    Mangas,
    Videos,
}

#[derive(Debug)]
pub struct Clean {
    what: Vec<CleanWhat>,
}

impl Runnable for Clean {
    fn run(&self, ctx: &RuntimeContext) {
        todo!()
    }
}

impl Command for Clean {
    fn from_args(args: &ArgMatches) -> Self {
        todo!()
    }
}

#[derive(Debug)]
pub struct Merge {}

impl Runnable for Merge {
    fn run(&self, ctx: &RuntimeContext) {
        todo!()
    }
}

impl Command for Merge {
    fn from_args(args: &ArgMatches) -> Self {
        Self {}
    }
}

#[derive(Debug)]
pub struct StopServer {}

impl Runnable for StopServer {
    fn run(&self, ctx: &RuntimeContext) {
        todo!()
    }
}

impl Command for StopServer {
    fn from_args(args: &ArgMatches) -> Self {
        Self {}
    }
}

#[derive(Debug)]
pub struct StartServer {
    foreground: bool,
    port: Option<u32>,
}

impl Runnable for StartServer {
    fn run(&self, ctx: &RuntimeContext) {
        let config = &ctx.config;
        println!("{:#?}", config);
        info!("Starting {} server version {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        if self.foreground {
            server::run_server(config).unwrap();
        } else {
            let config_clone = config.clone();
            detached::run_detached(RunOptions::from(config), move || {
                let s = server::run_server(&config_clone).unwrap();
            }).unwrap();
        }
    }
    fn modify_config(&self, config: &mut Config) {
        config.server.listen.port = self.port.unwrap_or(config.server.listen.port);
    }
}

impl Command for StartServer {
    fn from_args(args: &ArgMatches) -> Self {
        let subargs = args
            .subcommand_matches("server")
            .unwrap()
            .subcommand_matches("start")
            .unwrap();
        let mut port = None;
        if let Some(the_port) = subargs.value_of("port") {
            port = Some(the_port.trim().parse().unwrap());
        }
        Self {
            port,
            foreground: subargs.value_of("run-in").unwrap() == "foreground",
        }
    }
}

#[cfg(debug_assertions)]
mod sandbox {
    use super::*;

    pub struct Sandbox {}

    impl Runnable for Sandbox {
        fn run(&self, ctx: &RuntimeContext) {
        }
    }

    impl Command for Sandbox {
        fn from_args(args: &ArgMatches) -> Self { Self {}}
    }
}

pub struct RuntimeContext {
    pub config: Config,
    command: Box<dyn Runnable>,
}

impl RuntimeContext {
    pub fn new() -> Self {
        let yaml = load_yaml!("../clap-config.yaml");
        let args = CliApp::from_yaml(yaml).get_matches();
        let cfg_file = File::open(&args.value_of("config").unwrap()).unwrap();
        let mut config: Config = match serde_yaml::from_reader(cfg_file) {
            Ok(c) => c,
            Err(e) => Default::default(),
        };
        let command = Self::get_command(&args);
        command.modify_config(&mut config);
        Self { config, command }
    }

    pub fn run(&self) {
        self.command.run(&self);
    }

    fn get_command(args: &ArgMatches) -> Box<dyn Runnable> {
        fn from_server_args(args: &ArgMatches) -> Box<dyn Runnable> {
            let subargs = args.subcommand_matches("server").unwrap();
            match subargs.subcommand_name() {
                Some("start") => Box::new(StartServer::from_args(args)),
                Some("stop") => Box::new(StopServer::from_args(args)),
                _ => todo!(),
            }
        }
        match args.subcommand_name() {
            #[cfg(debug_assertions)]
            Some("sandbox") => Box::new(sandbox::Sandbox::from_args(args)),
            Some("clean") => Box::new(Clean::from_args(args)),
            Some("merge") => Box::new(Merge::from_args(args)),
            Some("server") => from_server_args(args),
            _ => todo!(),
        }
    }
}

impl Default for RuntimeContext {
    fn default() -> Self {
        Self::new()
    }
}
