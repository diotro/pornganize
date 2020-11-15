use std::fs::File;
use std::io::Result;
use std::path::PathBuf;

use clap::arg_enum;
use structopt::StructOpt;

use super::config::Config;

// TODO Read the file to see if the contents are empty and only THEN return the defaults.
fn load_cfg(src: &str) -> Result<Config> {
    let cfg_file = File::open(src)?;
    let config: Config = match serde_yaml::from_reader(cfg_file) {
        Ok(c) => c,
        Err(e) => Default::default(),
    };
    Ok(config)
}

arg_enum! {
    #[derive(Debug)]
    pub enum RunIn {
        Foreground,
        Background,
    }
}
arg_enum! {
    #[derive(Debug)]
    pub enum CleanWhat {
        All,
        Games,
        Images,
        Mangas,
        Videos,
    }
}

#[derive(Debug, StructOpt)]
pub enum ServerCommand {
    #[structopt(about = "Starts the server.")]
    Start {
        #[structopt(
            short,
            long,
            help = "The port to use (overrides the value in the configuration file)."
        )]
        port: Option<u32>,
        #[structopt(short, long)]
        workers: Option<usize>,
        #[cfg(unix)]
        #[structopt(long, help = "The pid file (if any).")]
        pid_file: Option<PathBuf>,
        #[structopt(
            short,
            long,
            possible_values = &RunIn::variants(),
            env = "PORNGANIZE_RUN_IN",
            case_insensitive = true,
            default_value = "background",
            help = "Whether the server should be run in the background or the foreground."
        )]
        run_in: RunIn,
    },
    #[structopt(about = "Stops the server.")]
    Stop {
        #[cfg(unix)]
        #[structopt(long, help = "The pid file (if any).")]
        pid_file: Option<PathBuf>,
    },
}

#[derive(Debug, StructOpt)]
pub enum Command {
    #[structopt(about = "Merges one collection into the current collection.")]
    Merge,
    #[structopt(about = "Removes the entries for files that no longer exist.")]
    Clean {
        #[structopt(
            multiple = true,
            possible_values = &CleanWhat::variants(),
            required = true
        )]
        what: CleanWhat,
    },
    #[structopt(about = "Items related to controlling the server.")]
    Server {
        #[structopt(subcommand)]
        command: ServerCommand,
    },
}

#[derive(StructOpt, Debug)]
#[structopt(
    name=env!("CARGO_PKG_NAME"),
    version=env!("CARGO_PKG_VERSION"),
    rename_all="kebab",
    author=env!("CARGO_PKG_AUTHORS"),
    about=env!("CARGO_PKG_DESCRIPTION")
)]
pub struct Opts {
    //TODO add custom parser.
    #[structopt(
        short,
        long,
        env = "PORNGANIZE_CONFIG",
        default_value = "config.yaml",
        global = true,
        help = "Configuration file for pornganize.",
        value_name = "CONFIG_FILE",
        parse(try_from_str = load_cfg),
    )]
    pub config: Config,
    #[structopt(subcommand)]
    pub subcommand: Command,
}

impl Opts {
    pub fn get() -> Self {
        Self::from_args()
    }
}
