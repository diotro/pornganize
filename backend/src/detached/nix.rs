extern crate daemonize;
use daemonize::{Daemonize, DaemonizeError};
use derive_more::{Error, Display};
use std::fs::File;
use std::io;

#[derive(Default, Builder)]
#[builder(setter(into))]
pub struct RunOptions<'a>
{
    #[builder(default = "None")]
    pid_file: Option<&'a str>,
    #[builder(default = "None")]
    user: Option<&'a str>,
    #[builder(default = "None")]
    group: Option<&'a str>,
    #[builder(default = "None")]
    pwd: Option<&'a str>,
    #[builder(default = "None")]
    stdout: Option<&'a str>,
    #[builder(default = "None")]
    stderr: Option<&'a str>,
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
        daemon = daemon.user(user);
    }
    if let Some(group) = options.group  {
        daemon = daemon.group(group);
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
