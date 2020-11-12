extern crate serde_yaml;
use crate::config::Config;
use clap::{load_yaml, App as CliApp, Arg, ArgMatches};
use std::fs::File;

trait Runnable {
    fn run(&self, config: &Config);
    fn modify_config(&self, config: &mut Config) {}
}

trait Command: Runnable {
    fn from_args(args: &ArgMatches) -> Self;
}

#[derive(Debug)]
pub enum CleanOptsWhat {
    Games,
    Images,
    Mangas,
    Videos,
}

#[derive(Debug)]
pub struct CleanOpts {
    what: Vec<CleanOptsWhat>,
}

impl Runnable for CleanOpts {
    fn run(&self, config: &Config) {
        unimplemented!()
    }
}

impl Command for CleanOpts {
    fn from_args(args: &ArgMatches) -> Self {
        unimplemented!()
    }
}

#[derive(Debug)]
pub struct MergeOpts {}

impl Runnable for MergeOpts {
    fn run(&self, config: &Config) {
        unimplemented!()
    }
}

impl Command for MergeOpts {
    fn from_args(args: &ArgMatches) -> Self {
        Self {}
    }
}

#[derive(Debug)]
pub struct StopServerOpts {}

impl Runnable for StopServerOpts {
    fn run(&self, config: &Config) {
        unimplemented!()
    }
}

impl Command for StopServerOpts {
    fn from_args(args: &ArgMatches) -> Self {
        Self {}
    }
}

#[derive(Debug)]
pub struct StartServerOpts {
    foreground: bool,
    port: Option<u32>,
}

impl Runnable for StartServerOpts {
    fn run(&self, config: &Config) {
        unimplemented!()
    }
    fn modify_config(&self, config: &mut Config) {
        config.server.listen.port = self.port.unwrap_or(config.server.listen.port);
    }
}

impl Command for StartServerOpts {
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
            foreground: subargs.is_present("foreground"),
        }
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
        self.command.run(&self.config);
    }

    fn get_command(args: &ArgMatches) -> Box<dyn Runnable> {
        fn from_server_args(args: &ArgMatches) -> Box<dyn Runnable> {
            let subargs = args.subcommand_matches("server").unwrap();
            match subargs.subcommand_name() {
                Some("start") => Box::new(StartServerOpts::from_args(args)),
                Some("stop") => Box::new(StopServerOpts::from_args(args)),
                _ => unimplemented!(),
            }
        }
        match args.subcommand_name() {
            Some("clean") => Box::new(CleanOpts::from_args(args)),
            Some("merge") => Box::new(MergeOpts::from_args(args)),
            Some("server") => from_server_args(args),
            _ => unimplemented!(),
        }
    }
}

impl Default for RuntimeContext {
    fn default() -> Self {
        Self::new()
    }
}
