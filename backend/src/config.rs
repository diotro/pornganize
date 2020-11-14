extern crate serde_yaml;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::fs::File;
use std::net::{IpAddr, Ipv4Addr};

use clap::{load_yaml, App as CliApp, Arg, ArgMatches};
use serde::{Deserialize, Serialize};

use crate::cli;

#[derive(Debug, Serialize, Deserialize, Clone)]
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

impl Display for Listen {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self.address {
            IpAddr::V4(address) => write!(f, "{}:{}", address, self.port),
            IpAddr::V6(address) => write!(f, "[{}]:{}", address, self.port),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct ServerCfg {
    pub listen: Listen,
    pub workers: usize,
    pub user: Option<String>,
    pub group: Option<String>,
    pub pid_file: Option<String>,
    pub log: Option<String>,
    pub error_log: Option<String>,
}

impl Default for ServerCfg {
    fn default() -> Self {
        Self {
            listen: Default::default(),
            workers: num_cpus::get() / 2,
            user: None,
            group: None,
            pid_file: None,
            log: None,
            error_log: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
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
            //TODO make title
            title: String::from(id),
            include: Vec::new(),
            exclude: Vec::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct Config {
    pub server: ServerCfg,
    pub sections: Vec<SectionCfg>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server: Default::default(),
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
