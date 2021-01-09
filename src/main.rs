
use actix_web::middleware::{DefaultHeaders, Logger};
use actix_web::{App, HttpServer};
use bastion::prelude::*;
use uuid::Uuid;

mod maluco_cli;
mod maluco_cli_web;

use maluco_cli_web::routes::app_routes;
//{project_use}

#[actix_rt::main]
async fn web_main() -> Result<(), std::io::Error> {
    
    HttpServer::new(|| {
        App::new()
        //{data}
        
    .wrap(DefaultHeaders::new().header("x-request-id", Uuid::new_v4().to_string()))
    .wrap(Logger::new("IP:%a DATETIME:%t REQUEST:\"%r\" STATUS: %s DURATION:%D X-REQUEST-ID:%{x-request-id}o"))

            .configure(app_routes)
    })
    .workers(num_cpus::get() + 2)
    .bind("0.0.0.0:4000")
    .unwrap()
    .run()
    .await
}


#[fort::root]
async fn main(_: BastionContext) -> Result<(), ()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    let _ = web_main();

    Ok(())
}
