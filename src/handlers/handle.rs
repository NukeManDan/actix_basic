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
        Ok(()) => Ok(HttpResponse::build(StatusCode::CREATED).json(serde_json::json!(&new_user))),
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

/// Unit testing the server endpoints for ideal and non-ideal cases
#[cfg(test)]
mod tests {
    //! Tests for `get_user` and `post_user` methods
    //! **Note: the endpoints for this lib are HARD CODED.**
    //! Only to be used with `http://127.0.0.1:9090/api/v1/users`
    //! Emulating the minimum behavior of
    //! `https://jsonplaceholder.typicode.com/api/v1/users`

    use super::*;

    use reqwest;
    use serde_json::json;
    use std::env;

    /// Set the endpoint URL of a request.
    /// Looks for env var "BIND_URL" as a base,
    /// or uses default http://127.0.0.1:9090
    fn endpoint(end: &str) -> String {
        let tmp = env::var("BIND_URL").unwrap_or("http://127.0.0.1:9090".into()) + end;
        eprintln!("{}", tmp);
        tmp
    }

    /// `POST` a new user with a JSON formatted raw string.
    /// Returns a `User` struct with fields filled as returned by the endpoint
    ///  if the post is valid.
    ///  This will include a new `User.id` field if the user is set properly.
    async fn post_user(
        ver: &str,
        user_str: &str,
    ) -> Result<reqwest::Response, Box<dyn std::error::Error>> {
        let post: user::User = serde_json::from_str(user_str)?;

        let client = reqwest::Client::new();
        let resp = client
            .post(endpoint(format!("/api/{}/users", ver).as_str()).as_str())
            .json(&post)
            .send()
            .await?;

        Ok(resp)
    }

    #[actix_rt::main]
    #[test]
    async fn it_gets_users() -> Result<(), Box<dyn std::error::Error>> {
        let resp = reqwest::get(endpoint("/api/v1/users").as_str()).await?;

        assert_eq!(reqwest::StatusCode::OK, resp.status());
        let users = resp.json::<Vec<user::User>>().await?;

        assert!(users.len() > 0);

        Ok(())
    }

    #[actix_rt::main]
    #[test]
    async fn health_check_pass() -> Result<(), Box<dyn std::error::Error>> {
        let resp = reqwest::get(endpoint("/health").as_str()).await?;

        assert_eq!(reqwest::StatusCode::OK, resp.status());
        let h = resp.text().await?;

        assert_eq!(h, "ok");

        Ok(())
    }

    #[actix_rt::main]
    #[test]
    async fn p404_returned_for_bad_get_endpoint() -> Result<(), Box<dyn std::error::Error>> {
        let resp = reqwest::get(endpoint("/not_an_endpoint").as_str()).await?;

        assert_eq!(reqwest::StatusCode::NOT_FOUND, resp.status());

        Ok(())
    }

    #[actix_rt::main]
    #[test]
    async fn it_fails_for_bad_version() -> Result<(), Box<dyn std::error::Error>> {
        let user_str = r#"
    { 
        "name": "My Name",
        "email": "my@email.com"
    }"#;
        let resp = post_user("v3", user_str).await?;

        assert_eq!(reqwest::StatusCode::METHOD_NOT_ALLOWED, resp.status());

        Ok(())
    }

    #[actix_rt::main]
    #[test]
    async fn it_creates_a_user() -> Result<(), Box<dyn std::error::Error>> {
        let user_str = r#"
    {
        "name": "Martin Fowler",
        "email": "martin@martinfowler.com"
    }"#;
        let resp = post_user("v1", user_str).await?;

        let expected: user::User = serde_json::from_value(
            json!({ "apiId": String::from("v1"), "email": String::from("martin@martinfowler.com"), "id": 11, "name": String::from("Martin Fowler")}),
        )?; // NOTE: assumes id = 11,
            // otherwise fails! Could we instead make a wildcard?

        assert_eq!(reqwest::StatusCode::CREATED, resp.status());
        assert_eq!(expected, resp.json::<user::User>().await?);

        Ok(())
    }

    #[actix_rt::main]
    #[test]
    async fn it_fails_to_post_a_short_user_name() -> Result<(), Box<dyn std::error::Error>> {
        let user_str = r#"
    { 
        "name": "MF",
        "email": "fine@email.com"
    }"#;
        let resp = post_user("v1", user_str).await?;

        assert_eq!(reqwest::StatusCode::UNPROCESSABLE_ENTITY, resp.status());

        Ok(())
    }

    #[actix_rt::main]
    #[test]
    async fn it_fails_to_post_a_long_user_name() -> Result<(), Box<dyn std::error::Error>> {
        let user_str = r#"
    { 
        "name": "9&mwuYuR&Hhp4p3%@bCvXVs5tbvZhFqmci8hcTXMSwi@x44e6M$mmQ#kE^agBNT3Brfnq757r8a#gJ$!vCYTd4SqMaHuAqSMbea4uhrC^2qi3%jFw",
        "email": "fine@email.com"
    }"#;
        let resp = post_user("v1", user_str).await?;

        assert_eq!(reqwest::StatusCode::UNPROCESSABLE_ENTITY, resp.status());

        Ok(())
    }

    #[actix_rt::main]
    #[test]
    async fn it_fails_to_post_invalid_user_email() -> Result<(), Box<dyn std::error::Error>> {
        let user_str = r#"
    { 
        "name": "My Name",
        "email": "bad"
    }"#;
        let resp = post_user("v1", user_str).await?;

        assert_eq!(reqwest::StatusCode::UNPROCESSABLE_ENTITY, resp.status());

        Ok(())
    }
}
