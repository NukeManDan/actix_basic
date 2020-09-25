//! Tasks defined rust_actix-basic.pdf
//! Using endpoints at: https://jsonplaceholder.typicode.com/<API>

use actix_web::http::StatusCode;
use actix_web::{error, web, Error, HttpResponse, Result};

use validator::Validate;

#[path = "../models/user.rs"]
mod user;

/// simple health check handler
#[get("/health")]
pub async fn health() -> Result<HttpResponse> {
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/plain; charset=utf-8")
        .body("ok"))
}

/// 404 handler
pub async fn p404() -> Result<HttpResponse> {
    Ok(HttpResponse::new(StatusCode::NOT_FOUND))
}

/// User GET requests specifying the API version `/api/{version}/users`
pub async fn get_users(vers: web::Path<(String,)>) -> Result<HttpResponse> {
    vers_check(&vers)?;

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("application/json; charset=utf-8")
        .body(include_str!("../../static/users.json")))
    // FIXME: this results in a static file of users, needs to give database values!!
}

/// User POST requests specifying the API version `/api/{version}/users`
pub async fn post_user(
    vers: web::Path<(String,)>,
    user: web::Json<user::User>,
) -> Result<HttpResponse, Error> {
    vers_check(&vers)?;

    // NOTE: this results in *dummy* response
    // needs to write to and return database values in production!!
    let mut new_user = user.into_inner();
    new_user.id = 11;

    // validation requirements defined in User struct definition
    // for each element with a #[validate(...)] macro
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
pub fn vers_check(vers: &web::Path<(String,)>) -> Result<(), Error> {
    if vers.0.as_str() != "v1" {
        Err(error::ErrorMethodNotAllowed(format!(
            "There is no API version \"{}\" ! Try .../api/v1/...\n",
            vers.0
        )))
    } else {
        Ok(()) // API is correct version
    }
}