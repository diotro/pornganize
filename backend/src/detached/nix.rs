extern crate daemonize;
use daemonize::{Daemonize, DaemonizeError, User, Group};
use derive_more::{Error, Display};
use std::fs::File;
use std::io;
use crate::config::Config;

#[derive(Default, Builder)]
#[builder(setter(into))]
pub struct RunOptions
{
    #[builder(default = "None")]
    pid_file: Option<String>,
    #[builder(default = "None")]
    user: Option<String>,
    #[builder(default = "None")]
    group: Option<String>,
    #[builder(default = "None")]
    pwd: Option<String>,
    #[builder(default = "None")]
    stdout: Option<String>,
    #[builder(default = "None")]
    stderr: Option<String>,
}

impl From<& Config> for RunOptions {
    fn from(config: & Config) -> Self {
        Self {
            //TODO Do this better once you've become more skilled in rust
            pid_file: config.server.pid_file.as_ref().cloned(),
            user: config.server.user.as_ref().cloned(),
            group: config.server.group.as_ref().cloned(),
            stdout: config.server.log.as_ref().cloned(),
            stderr: config.server.error_log.as_ref().cloned(),
            pwd: Some(String::from(".")),
        }
    }
}


#[derive(Debug, Display, Error)]
pub enum DetachError {
    IoError(#[error(source)] io::Error),
    DaemonizationError(#[error(source)] DaemonizeError),
}

pub fn run_detached<F>(options: RunOptions, to_run: F) -> Result<(), DetachError>
    where F: FnOnce() -> ()
{
    let mut daemon = Daemonize::new();
    if let Some(path) = options.pid_file  {
        daemon = daemon.pid_file(path).chown_pid_file(true);
    }
    if let Some(user) = options.user  {
        daemon = daemon.user(User::Name(user));
    }
    if let Some(group) = options.group  {
        daemon = daemon.group(Group::Name(group));
    }
    if let Some(pwd) = options.pwd  {
        daemon = daemon.working_directory(pwd);
    }
    if let Some(stdout) = options.stdout  {
        let file = match File::create(stdout) {
            Ok(f) => f,
            Err(e) => return Err(DetachError::IoError(e)),
        };
        daemon = daemon.stdout(file);
    }
    if let Some(stderr) = options.stderr  {
        let file = match File::create(stderr) {
            Ok(f) => f,
            Err(e) => return Err(DetachError::IoError(e)),
        };
        daemon = daemon.stderr(file);
    }
    daemon.start().or_else(|e| Err(DetachError::DaemonizationError(e)))
}
