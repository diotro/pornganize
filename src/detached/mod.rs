cfg_if::cfg_if! {
    if #[cfg(unix)] {
        mod nix;
        use nix as base;
    } else if #[cfg(windows)] {
        mod win;
        use win as base;
    } else {
        panic!{"Unsupported environment!"};
    }
}
pub use base::run_detached;
pub use base::DetachError;
pub use base::RunOptions;
pub use base::RunOptionsBuilder;
