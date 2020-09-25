//! Actix Basic
//!
//! Tasks defined `rust_actix-basic.pdf`
//!
//! - Modeling endpoint behavior based on
//! [this test api](https://jsonplaceholder.typicode.com/api/v1/users)
//! - This server implementation based on
//! [this basic example](https://github.com/actix/examples/blob/master/basics/src/main.rs)

use simplelog::*;
use std::{env, io};

#[macro_use]
extern crate actix_web;
use actix_files as fs;
use actix_web::{middleware, web, App, HttpServer};

#[path = "models/user.rs"]
mod user;

#[path = "handlers/handle.rs"]
mod handle;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    let _ = SimpleLogger::init(LevelFilter::Info, Config::default());

    HttpServer::new(|| {
        App::new()
            // Logger
            .wrap(middleware::Logger::default())
            // Limit size of the payload (global configuration)
            .data(web::JsonConfig::default().limit(4096))
            // register simple route, handle all methods
            .service(handle::health)
            // with version path parameters
            .service(
                web::resource("/api/{vers}/users")
                    // Limit size of the payload (resource level)
                    .data(web::JsonConfig::default().limit(1024))
                    .route(web::get().to(handle::get_users))
                    .route(web::post().to(handle::post_user)),
            )
            // static files
            .service(fs::Files::new("/static", "static").show_files_listing())
            // default
            .default_service(
                // 404 for any other request
                web::resource("").route(web::get().to(handle::p404)),
            )
    })
    .bind(env::var("BIND_URL").unwrap_or("127.0.0.1:9090".into()))?
    .run()
    .await
}
