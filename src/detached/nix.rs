use std::fs::File;
use std::io;
use std::path::PathBuf;
use daemonize::{Daemonize, DaemonizeError, Group, User};
use pornganize::config::Config;

#[derive(Default, Builder)]
#[builder(setter(into))]
pub struct RunOptions {
    #[builder(default = "None")]
    pid_file: Option<PathBuf>,
    #[builder(default = "None")]
    user: Option<String>,
    #[builder(default = "None")]
    group: Option<String>,
    #[builder(default = "None")]
    pwd: Option<PathBuf>,
    #[builder(default = "None")]
    stdout: Option<PathBuf>,
    #[builder(default = "None")]
    stderr: Option<PathBuf>,
}

impl From<&Config> for RunOptions {
    fn from(config: &Config) -> Self {
        Self {
            //TODO Do this better once you've become more skilled in rust
            pid_file: config.server.pid_file.as_ref().cloned(),
            user: config.server.user.as_ref().cloned(),
            group: config.server.group.as_ref().cloned(),
            stdout: config.server.log.as_ref().cloned(),
            stderr: config.server.error_log.as_ref().cloned(),
            pwd: Some(PathBuf::from(".")),
        }
    }
}

#[derive(Debug)]
pub enum DetachError {
    IoError(io::Error),
    DaemonizationError(DaemonizeError),
}

pub fn run_detached<F>(options: RunOptions, to_run: F) -> Result<(), DetachError>
where
    F: FnOnce(),
{
    let mut daemon = Daemonize::new();
    if let Some(path) = options.pid_file {
        daemon = daemon.pid_file(path).chown_pid_file(true);
    }
    if let Some(user) = options.user {
        daemon = daemon.user(User::Name(user));
    }
    if let Some(group) = options.group {
        daemon = daemon.group(Group::Name(group));
    }
    if let Some(pwd) = options.pwd {
        daemon = daemon.working_directory(pwd);
    }
    if let Some(stdout) = options.stdout {
        let file = match File::create(stdout) {
            Ok(f) => f,
            Err(e) => return Err(DetachError::IoError(e)),
        };
        daemon = daemon.stdout(file);
    }
    if let Some(stderr) = options.stderr {
        let file = match File::create(stderr) {
            Ok(f) => f,
            Err(e) => return Err(DetachError::IoError(e)),
        };
        daemon = daemon.stderr(file);
    }
    match daemon.start() {
        #[allow(clippy::unit_arg)]
        Ok(_) => Ok(to_run()),
        Err(e) => Err(DetachError::DaemonizationError(e)),
    }
}
