extern crate serde_yaml;
use std::fs::File;
use std::net::{IpAddr, Ipv4Addr};

use clap::{App as CliApp, Arg, ArgMatches};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Listen {
    pub address: IpAddr,
    pub port: u32,
}

impl Default for Listen {
    fn default() -> Self {
        Self {
            address: IpAddr::V4(Ipv4Addr::LOCALHOST),
            port: 3000,
        }
    }
}

impl ToString for Listen {
    fn to_string(&self) -> String {
        match self.address {
            IpAddr::V4(address) => format!("{}:{}", address, self.port),
            IpAddr::V6(address) => format!("[{}]:{}", address, self.port),
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct LibraryBackupsCfg {
    pub max: u32,
    pub path: String,
}

impl Default for LibraryBackupsCfg {
    fn default() -> Self {
        Self {
            max: 10,
            path: String::from("backups"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct LibraryCfg {
    pub path: String,
    pub backups: LibraryBackupsCfg,
}

impl Default for LibraryCfg {
    fn default() -> Self {
        Self {
            path: String::from("lib"),
            backups: Default::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SectionCfg {
    pub id: String,
    pub title: String,
    pub include: Vec<String>,
    pub exclude: Vec<String>,
}

impl From<&str> for SectionCfg {
    fn from(id: &str) -> Self {
        Self {
            id: String::from(id),
            title: String::from(id),
            include: Vec::new(),
            exclude: Vec::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub listen: Listen,
    pub sections: Vec<SectionCfg>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            listen: Default::default(),
            sections: vec![
                SectionCfg::from("actors"),
                SectionCfg::from("videos"),
                SectionCfg::from("studios"),
                SectionCfg::from("sites"),
                SectionCfg::from("dvds"),
            ],
        }
    }
}

//fn get_cli_args() -> ArgMatches {
fn get_cli_args() {
    CliApp::new("pornganize")
        .version("0.1.0")
        .author("Brendan McGloin")
        .about("Organize your smut!")
        .arg(
            Arg::with_name("port")
                .short("p")
                .long("port")
                .help("The port to use.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .help("Path to the configuration file.")
                .takes_value(true)
                .default_value("config.yaml"),
        )
        .get_matches();
    //.get_matches()
}

impl Config {
    pub fn new() -> Result<Self, serde_yaml::Error> {
        let args = CliApp::new("pornganize")
            .version("0.1.0")
            .author("Brendan McGloin")
            .about("Organize your smut!")
            .arg(
                Arg::with_name("port")
                    .short("p")
                    .long("port")
                    .help("The port to use.")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("config")
                    .short("c")
                    .long("config")
                    .help("Path to the configuration file.")
                    .takes_value(true)
                    .default_value("config.yaml"),
            )
            .get_matches();
        //let args = get_cli_args();
        let cfg_file = File::open(&args.value_of("config").unwrap()).unwrap();
        let mut cfg: Self = match serde_yaml::from_reader(cfg_file) {
            Ok(c) => c,
            //TODO ask to create default config
            //Err(e) => panic!(e),
            Err(e) => {
                println!("\n");
                debug!(e);
                println!("\n");
                panic!(e);
            } //Err(e) => match e {
              //"EndOfStream" => Default::default(),
              //_ => panic!(e),
              //}
        };
        if let Some(port) = args.value_of("port") {
            cfg.listen.port = port.trim().parse().unwrap()
        }
        Ok(cfg)
    }
}
