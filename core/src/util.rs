use actix::{System, SystemRunner};

/// Creates the Actix SystemRunner.
pub fn create_actix_runner() -> SystemRunner {
    System::builder()
        .name(env!("CARGO_PKG_NAME"))
        .stop_on_panic(true)
        .build()
}
