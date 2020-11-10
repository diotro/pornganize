#![allow(unused_imports, unreachable_code, dead_code)]
#[macro_use]
extern crate debug_rs;
use actix_web::{middleware, App as WebApp, HttpServer};
mod app;
mod config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //pornganize::run()
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    let cfg = config::Config::new().unwrap();
    HttpServer::new(|| {
        WebApp::new()
            .configure(app::configure_web_app)
            .wrap(middleware::Logger::default())
    })
    .bind(cfg.listen.to_string())?
    .run()
    .await
}
