use std::fmt::{Display, Formatter, Result as FmtResult};
use std::fs::File;
use std::net::{IpAddr, Ipv4Addr};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct Listen {
    pub address: IpAddr,
    pub port: u32,
}

impl Listen {
    pub fn with_port(&self, port: u32) -> Self {
        Self {
            port,
            ..self.clone()
        }
    }
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
    pub log: Option<PathBuf>,
    pub error_log: Option<PathBuf>,
    //TODO make unix only
    pub user: Option<String>,
    pub group: Option<String>,
    pub pid_file: Option<PathBuf>,
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
    pub path: PathBuf,
}

impl Default for LibraryBackupsCfg {
    fn default() -> Self {
        Self {
            max: 10,
            path: PathBuf::from("backups"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "backend")]
pub enum DatabaseCfg {
    #[serde(rename = "sqlite")]
    Sqlite {
        file: PathBuf,
        backups: LibraryBackupsCfg,
    },
    #[serde(rename = "postgres")]
    Postgres {},
    #[serde(rename = "mysql")]
    MySql {},
    #[serde(rename = "mssql")]
    MsSql {},
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct LibraryCfg {
    pub path: PathBuf,
    pub database: DatabaseCfg,
}

impl Default for LibraryCfg {
    fn default() -> Self {
        let mut current_path = PathBuf::from("lib");
        //let path = String::from(current_path.to_str().unwrap());
        let path = current_path.clone();
        current_path.push("lib.db");
        let db_path = current_path.clone();
        Self {
            path,
            database: DatabaseCfg::Sqlite {
                file: db_path,
                backups: Default::default(),
            },
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
            //TODO Capitalize id
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
    pub library: LibraryCfg,
    pub sections: Vec<SectionCfg>,
}

//impl<'de: 'a, 'a> Deserialize<'de> for Config<'a> {
//}

impl Default for Config {
    fn default() -> Self {
        Self {
            server: Default::default(),
            library: Default::default(),
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
