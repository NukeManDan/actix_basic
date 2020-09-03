//! Actix Basic
//!
//! Tasks defined `rust_actix-basic.pdf`
//!
//! - Modeling endpoint behavior base on [this test api](https://jsonplaceholder.typicode.com/api/v1/users)
//! - This server implementation based on [this basic example](https://github.com/actix/examples/blob/master/basics/src/main.rs)

use ::serde_json;
use simplelog::*;
use std::{env, io};
use validator::Validate;

#[macro_use]
extern crate actix_web;
use actix_files as fs;
use actix_web::http::StatusCode;
use actix_web::{error, middleware, web, App, Error, HttpResponse, HttpServer, Result};

mod user;

/// favicon handler
#[get("/favicon.ico")]
async fn favicon() -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("static/favicon.ico")?)
}

/// simple index handler
#[get("/health")]
async fn health() -> Result<HttpResponse> {
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/plain; charset=utf-8")
        .body("ok"))
}

/// 404 handler
async fn p404() -> Result<HttpResponse> {
    Ok(HttpResponse::new(StatusCode::NOT_FOUND))
}

/// User GET requests specifying the API version `/api/{version}/users`
async fn get_users(vers: web::Path<(String,)>) -> Result<HttpResponse> {
    vers_check(&vers)?;

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("application/json; charset=utf-8")
        .body(include_str!("../static/users.json")))
    // FIXME: this results in a static file of users, needs to give database values!!
}

/// User POST requests specifying the API version `/api/{version}/users`
async fn post_user(
    vers: web::Path<(String,)>,
    user: web::Json<user::User>,
) -> Result<HttpResponse, Error> {
    vers_check(&vers)?;

    // NOTE: this results in *dummy* response, needs to write to and return database values in production!!
    let mut new_user = user.into_inner();
    new_user.id = 11;

    // validation requirements defined in User struct definition, for each element with a #[validate(...)] macro
    match new_user.validate() {
        Ok(()) => {
            Ok(HttpResponse::build(StatusCode::CREATED)
                // .content_type("application/json; charset=utf-8")
                .json(serde_json::json!(&new_user)))
        }
        Err(e) => Err(error::ErrorUnprocessableEntity(e)),
    }
}

/// Presently, only v1 version supported
fn vers_check(vers: &web::Path<(String,)>) -> Result<(), Error> {
    if vers.0.as_str() != "v1" {
        Err(error::ErrorMethodNotAllowed(format!(
            "There is no API version \"{}\" ! Try .../api/v1/...\n",
            vers.0
        )))
    } else {
        Ok(()) // API is correct version
    }
}

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
            .service(health)
            // with version path parameters
            .service(
                web::resource("/api/{vers}/users")
                    // Limit size of the payload (resource level)
                    .data(web::JsonConfig::default().limit(1024))
                    .route(web::get().to(get_users))
                    .route(web::post().to(post_user)),
            )
            // static files
            .service(fs::Files::new("/static", "static").show_files_listing())
            // default
            .default_service(
                // 404 for any other request
                web::resource("").route(web::get().to(p404)),
            )
    })
    .bind(env::var("BIND_URL").unwrap_or("127.0.0.1:9090".into()))?
    .run()
    .await
}
