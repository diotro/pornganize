extern crate derive_more;

#[cfg(unix)]
mod nix;
#[cfg(unix)]
use nix as base;
#[cfg(target_os = "windows")]
mod win;
#[cfg(target_os = "windows")]
use win as base;

pub use base::RunOptions;
pub use base::RunOptionsBuilder;
pub use base::run_detached;
pub use base::DetachError;
