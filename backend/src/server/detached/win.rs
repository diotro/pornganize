todo!();
pub struct RunOptions {}

pub enum DetachError {}

pub fn run_detached<F>(options: RunOptions, to_run: F) -> Result<(), DetachError>
where
    F: FnOnce() -> (),
{
    todo!();
}
