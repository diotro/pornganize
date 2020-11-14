#![allow(unused_imports, unreachable_code, dead_code, unused_variables)]
use std::sync::mpsc::{self, Receiver};
use std::thread::{JoinHandle, Builder as ThreadBuilder};

use actix_web::{dev::Server, middleware, rt, App as WebApp, HttpServer};

use super::{app, config::Config};


fn create_server(config: &Config) -> Server {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    HttpServer::new(|| {
        WebApp::new()
            .configure(app::configure_web_app)
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
        })
        .bind(config.server.listen.to_string()).unwrap()
        .shutdown_timeout(10)
        .workers(config.server.workers)
        .run()
}

pub fn run_server(config: &Config) -> std::io::Result<()> {
    let mut sys = rt::System::builder()
        .name("pornganize")
        .stop_on_panic(true)
        .build();
    let srv = create_server(config);
    sys.block_on(srv)
}

//pub fn run_server_in_thread<'a>(config: &'a Config) -> JoinHandle<()> {
//    //pornganize::run()
//    let (tx, rx) = mpsc::channel();
//    let config_clone = config.clone();
//    let thread = ThreadBuilder::new()
//        .name(String::from("pornganize"));
//    let handle = thread.spawn(move || {
//        let mut sys = rt::System::builder()
//            .name("pornganize")
//            .stop_on_panic(true)
//            .build();
//        let srv = create_server(&config_clone);
//        let _ = tx.send(srv.clone()).unwrap();
//        let _ = sys.block_on(srv);
//    }).unwrap();
//    let srv = rx.recv().unwrap();
//    handle
//}
