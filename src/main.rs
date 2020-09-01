//! Actix Basic
//!
//! Tasks defined `rust_actix-basic.pdf`
//!
//! - Modeling endpoint behavior base on [this test api](https://jsonplaceholder.typicode.com/api/v1/users)
//! - This server implementation based on [this basic example](https://github.com/actix/examples/blob/master/basics/src/main.rs)

use serde_json::json;
use simplelog::*;
use std::{env, io};

#[macro_use]
extern crate actix_web;
use actix_files as fs;
use actix_web::http::StatusCode;
use actix_web::{guard, middleware, web, App, HttpResponse, HttpServer, Result};

mod user;

// only v1 version supported
fn bad_vers(vers: web::Path<(String,)>) -> Option<HttpResponse> {
    if vers.0.as_str() != "v1" {
        Some(
            HttpResponse::VersionNotSupported()
                .content_type("text/plain; charset=utf-8")
                .body(format!(
                    "There is no API version \"{}\" ! Try .../api/v1/...",
                    vers.0
                )),
        )
    } else {
        None // API is correct version
    }
}
/// favicon handler
#[get("/favicon.ico")]
async fn favicon() -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("static/favicon.ico")?)
}

/// simple index handler
#[get("/")]
async fn welcome() -> Result<HttpResponse> {
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/welcome.html")))
}

/// 404 handler
async fn p404() -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("static/404.html")?.set_status_code(StatusCode::NOT_FOUND))
}

/// User requests specifying the API version `/api/{version}/users`
async fn get_users(vers: web::Path<(String,)>) -> HttpResponse {
    match bad_vers(vers) {
        Some(bad) => bad,
        None => {
            return HttpResponse::Ok()
                .content_type("application/json; charset=utf-8")
                .body(include_str!("../static/users.json"));
            // FIXME: this results in a static file of users, needs to give database values!!
        }
    }
}

/// User requests specifying the API version `/api/{version}/users`
async fn post_user(vers: web::Path<(String,)>, user: web::Json<user::User>) -> HttpResponse {
    match bad_vers(vers) {
        Some(bad) => bad,
        None => {
            let resp: String = serde_json::to_string(
        &json!({ "apiId": String::from("v1"), "email": user.email, "id": 11, "name": user.name}),
    )
    .unwrap(); // FIXME: this gives a single User struct with the req body and a dummy user ID. Needs to do database operations!!

            return HttpResponse::Ok()
                .content_type("application/json; charset=utf-8")
                .body(resp);
        }
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
            // register favicon
            .service(favicon)
            // register simple route, handle all methods
            .service(welcome)
            // with version path parameters
            .service(
                web::resource("/api/{vers}/users")
                    .route(web::get().to(get_users))
                    .route(web::post().to(post_user)),
            )
            // static files
            .service(fs::Files::new("/static", "static").show_files_listing())
            // default
            .default_service(
                // 404 for GET request
                web::resource("")
                    .route(web::get().to(p404))
                    // all requests that are not `GET`
                    .route(
                        web::route()
                            .guard(guard::Not(guard::Get()))
                            .to(HttpResponse::MethodNotAllowed),
                    ),
            )
    })
    .bind("127.0.0.1:9090")?
    .run()
    .await
}
