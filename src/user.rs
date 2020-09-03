//! Tasks defined rust_actix-basic.pdf
//! Using endpoints at: https://jsonplaceholder.typicode.com/<API>

use validator::Validate;
use validator_derive;

use serde::{Deserialize, Serialize};

/// Struct used to serialize and deserialize responses from for a user
#[derive(Serialize, Deserialize, validator_derive::Validate, Default, Debug, PartialEq)]
pub struct User {
    #[serde(default)]
    pub id: usize,
    #[serde(default)]
    #[validate(length(min = 3, max = 80))]
    pub name: String,
    #[serde(default)]
    // #[validate(length(min = 3, max = 80))]
    pub username: String,
    #[serde(default)]
    #[validate(email)]
    pub email: String,
    #[serde(default)]
    pub address: Address,
    #[serde(default)]
    // #[validate(phone)] // FIXME:Fails with "unexpected validator" on compile.
    pub phone: String,
    #[serde(default)]
    pub website: String,
    #[serde(default)]
    pub company: Company,
}

/// Internal to `User` struct
#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct Address {
    #[serde(default)]
    pub street: String,
    #[serde(default)]
    pub suite: String,
    #[serde(default)]
    pub city: String,
    #[serde(default)]
    pub zipcode: String,
    #[serde(default)]
    pub geo: Geo,
}

/// Internal to `User` struct
#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct Geo {
    #[serde(default)]
    pub lat: String,
    #[serde(default)]
    pub lng: String,
}

/// Internal to `User` struct
#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct Company {
    #[serde(default)]
    pub name: String,
    #[serde(rename = "catchPhrase", default)]
    //snake case to camel for JSON format to keep idiomatic rust var names.
    pub catch_phrase: String, // NOTE: JSON uses catchPhrase
    #[serde(default)]
    pub bs: String,
}

/// Unit testing the server endpoints for ideal and non-ideal cases
#[cfg(test)]
mod tests {
    //! Tests for `get_user` and `post_user` methods
    //! Note: the endpoints for this lib are HARD CODED. only to be used with `http://127.0.0.1:9090/api/v1/users`
    //! Emulating the minimum behavior of `https://jsonplaceholder.typicode.com/api/v1/users`

    use super::*;

    use log::{debug, error, info};
    use reqwest;
    use serde_json::json;

    const URL: &str = "http://127.0.0.1:9090/api/v1/users";

    /// `POST` a new user with a JSON formatted raw string. Returns a `User` struct with fields filled as returned by the endpoint,if the post is valid. This will include a new `User.id` field if the user is set properly.
    async fn post_user(user_str: &str) -> Result<reqwest::Response, Box<dyn std::error::Error>> {
        let post: User = serde_json::from_str(user_str)?;

        let client = reqwest::Client::new();
        let resp = client.post(URL).json(&post).send().await?;

        Ok(resp)
    }

    #[actix_rt::main]
    #[test]
    async fn it_gets_users() -> Result<(), Box<dyn std::error::Error>> {
        let resp = reqwest::get(URL).await?;

        assert_eq!(reqwest::StatusCode::OK, resp.status());
        let users = resp.json::<Vec<User>>().await?;

        assert!(users.len() > 0);

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
        let resp = post_user(user_str).await?;

        let expected: User = serde_json::from_value(
            json!({ "apiId": String::from("v1"), "email": String::from("martin@martinfowler.com"), "id": 11, "name": String::from("Martin Fowler")}),
        )?; //NOTE: assumes id = 11, otherwise fails! Could we instead make a wildcard?

        assert_eq!(reqwest::StatusCode::CREATED, resp.status());
        assert_eq!(expected, resp.json::<User>().await?);

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
        let resp = post_user(user_str).await?;

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
        let resp = post_user(user_str).await?;

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
        let resp = post_user(user_str).await?;

        assert_eq!(reqwest::StatusCode::UNPROCESSABLE_ENTITY, resp.status());

        Ok(())
    }
}
