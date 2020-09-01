//! Tasks defined rust_actix-basic.pdf
//! Using endpoints at: https://jsonplaceholder.typicode.com/<API>

use log::{debug, error, info};
use validator::Validate;
use validator_derive;

use reqwest;
use serde::{Deserialize, Serialize};

/// Submits a GET request to a specified URL, and returns a vector of users. Expected response is in json format, and can be parsed by the `User` struct with accessible elements by key name.
#[allow(dead_code)] // used in tests only
pub async fn get_all(url: &str) -> Result<Vec<User>, Box<dyn std::error::Error>> {
    // let users = reqwest::get(url).await?.json::<Vec<User>>().await?;
    let resp = reqwest::get(url).await?;
    print_status(&resp);
    Ok(resp.json::<Vec<User>>().await?)
}

/// `POST` a new user with a JSON formatted raw string (&str) passed into `post_user`. Returns a `User` struct with fields filled as returned by the endpoint,if the post is valid. This will include a new `User.id` field if the user is set properly.
///
/// The name will need to be at least 3 characters long and the email address should be valid.
///
/// ## Errors
///
/// This will panic if the `name` is not within range (3,80) characters, or `email` is not valid.
///
/// > Note: The name will need to be between 3 and 80 characters long and the email address should be valid.
#[allow(dead_code)] // used in tests only
pub async fn post(url: &str, user_json: &str) -> Result<User, Box<dyn std::error::Error>> {
    let post: User = serde_json::from_str(user_json)?;

    post.validate()?; // validation requirements defined in struct definition

    let client = reqwest::Client::new();
    let resp = client.post(url).json(&post).send().await?;
    print_status(&resp);

    let body = serde_json::from_str(resp.text().await?.as_str())?;

    Ok(body)
}

#[allow(dead_code)] // used in tests only
fn print_status(resp: &reqwest::Response) {
    let text = format!(
        "Response Status Code: {} ({}) - {}",
        resp.status().as_str(),
        resp.status().canonical_reason().unwrap(),
        resp.url().as_str()
    );
    match resp.status() {
        reqwest::StatusCode::OK => debug!("{}", &text),
        reqwest::StatusCode::NOT_FOUND => error!("{}", &text),
        _ => info!("{}", &text),
    };
}

/// Defining Structs used to serialize and deserialize responses from reqwest for a user
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

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct Geo {
    #[serde(default)]
    pub lat: String,
    #[serde(default)]
    pub lng: String,
}

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

// TESTS ------------------
#[cfg(test)]
mod tests {
    //! Tests for `get_user` and `post_user` methods
    //! Note: the endpoints for this lib are HARD CODED. only to be used with `https://jsonplaceholder.typicode.com/api/v1/users`

    use super::*;

    use serde_json::json;

    const URL: &str = "http://127.0.0.1:9090/api/v1/users";

    #[actix_rt::main]
    #[test]
    async fn it_gets_users() -> Result<(), Box<dyn std::error::Error>> {
        let users: Vec<User> = get_all(URL).await?;

        assert!(users.len() > 0);

        Ok(())
    }

    #[actix_rt::main]
    #[test]
    async fn it_posts_a_user() -> Result<(), Box<dyn std::error::Error>> {
        let user_str = r#"
    {
        "name": "Martin Fowler",
        "email": "martin@martinfowler.com"
    }"#;

        let resp_json_expected: User = serde_json::from_value(
            json!({ "apiId": String::from("v1"), "email": String::from("martin@martinfowler.com"), "id": 11, "name": String::from("Martin Fowler")}),
        )?; //NOTE: assumes id = 11, otherwise fails! Could we instead make a wildcard?

        // println!("Posting new user:\n{}...", user_str);
        let new_user = post(URL, user_str).await?;
        // println!("---Response (full body):\n{:#?}", new_user);

        assert_eq!(resp_json_expected, new_user);

        Ok(())
    }

    #[actix_rt::main]
    #[test]
    #[should_panic]
    async fn it_fails_to_post_a_short_user_name() {
        let user_str = r#"
    { 
        "name": "MF",
        "email": "fine@email.com"
    }"#;

        post(URL, user_str).await.unwrap();
    }

    #[actix_rt::main]
    #[test]
    #[should_panic]
    async fn it_fails_to_post_a_long_user_name() {
        let user_str = r#"
    { 
        "name": "9&mwuYuR&Hhp4p3%@bCvXVs5tbvZhFqmci8hcTXMSwi@x44e6M$mmQ#kE^agBNT3Brfnq757r8a#gJ$!vCYTd4SqMaHuAqSMbea4uhrC^2qi3%jFw",
        "email": "fine@email.com"
    }"#;

        post(URL, user_str).await.unwrap();
    }

    #[actix_rt::main]
    #[test]
    #[should_panic]
    async fn it_fails_to_post_invalid_user_email() {
        let user_str = r#"
    { 
        "name": "My Name",
        "email": "bad"
    }"#;

        post(URL, user_str).await.unwrap();
    }
}
