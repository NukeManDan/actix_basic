//! Tasks defined rust_actix-basic.pdf
//! Using endpoints at: https://jsonplaceholder.typicode.com/<API>

use validator::Validate;
use validator_derive;

use reqwest;
use serde::{Deserialize, Serialize};

/// Submits a GET request to a specified URL, and returns a vector of users. Expected response is in json format, and can be parsed by the `User` struct with accessible elements by key name.
pub async fn get_users(url: &str) -> Result<Vec<User>, Box<dyn std::error::Error>> {
    let users = reqwest::get(url).await?.json::<Vec<User>>().await?;

    Ok(users)
}

/// `POST` a new user with a JSON formatted raw string (&str) passed into `post_user`. Returns a `User` struct with fields filled as returned by the endpoint,if the post is valid. This will include a new `User.id` field if the user is set properly.
///
/// The name will need to be at least 3 characters long and the email address should be valid.
///
/// ## Errors
///
/// This will panic if the `name` is less than 3 characters, or `email` is not valid.
///
/// > Note: The name will need to be at least 3 characters long and the email address should be valid.
pub async fn post_user(url: &str, user_json: &str) -> Result<User, Box<dyn std::error::Error>> {
    let post: User = serde_json::from_str(user_json)?;

    post.validate()?; // validation requirements defined in struct definition

    let client = reqwest::Client::new();
    let resp = client.post(url).json(&post).send().await?;

    let body = serde_json::from_str(resp.text().await?.as_str())?;

    Ok(body)
}

/// Defining Structs used to serialize and deserialize responses from reqwest for a user
#[derive(Serialize, Deserialize, validator_derive::Validate, Default, Debug)]
pub struct User {
    #[serde(default)]
    pub id: usize,
    #[serde(default)]
    #[validate(length(min = 3))]
    pub name: String,
    #[serde(default)]
    pub username: String,
    #[serde(default)]
    #[validate(email)]
    pub email: String,
    #[serde(default)]
    pub address: Address,
    #[serde(default)]
    pub phone: String,
    #[serde(default)]
    pub website: String,
    #[serde(default)]
    pub company: Company,
}

#[derive(Serialize, Deserialize, Default, Debug)]
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

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Geo {
    #[serde(default)]
    pub lat: String,
    #[serde(default)]
    pub lng: String,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Company {
    #[serde(default)]
    pub name: String,
    #[serde(rename = "catchPhrase", default)]
    //snake case to camel for JSON format to keep idiomatic rust var names.
    pub catch_phrase: String, // NOTE: JSON uses catchPhrase
    #[serde(default)]
    pub bs: String,
}

#[cfg(test)]
mod tests {
    //! Tests for `get_user` and `post_user` methods
    //! Note: the endpoints for this lib are HARD CODED. only to be used with `https://jsonplaceholder.typicode.com/api/v1/users`

    use super::*;

    use serde_json::json;
    use tokio;

    const URL: &str = "https://jsonplaceholder.typicode.com/api/v1/users";

    #[tokio::main]
    #[test]
    async fn it_gets_users() -> Result<(), Box<dyn std::error::Error>> {
        let address_check = Address {
            street: String::from("Douglas Extension"),
            suite: String::from("Suite 847"),
            city: String::from("McKenziehaven"),
            zipcode: String::from("59590-4157"),
            geo: Geo {
                lat: String::from("-68.6102"),
                lng: String::from("-47.0653"),
            },
        };

        let users: Vec<User> = get_users(URL).await?;
        // println!("---Response: (Address of a user):\n {:#?}", users[2].address.street);

        assert_eq!(address_check.street, users[2].address.street);

        Ok(())
    }

    #[tokio::main]
    #[test]
    async fn it_posts_a_user() -> Result<(), Box<dyn std::error::Error>> {
        let user_str = r#"
    {
        "name": "Martin Fowler",
        "email": "martin@martinfowler.com"
    }"#;

        let resp_json_expected = json!({ "apiId": String::from("v1"), "email": String::from("martin@martinfowler.com"), "id": 11, "name": String::from("Martin Fowler")});

        // println!("Posting new user:\n{}...", user_str);
        let new_user = post_user(URL, user_str).await?;
        // println!("---Response (full body):\n{:#?}", new_user);

        assert_eq!(
            resp_json_expected.get("email").unwrap(),
            new_user.get("email").unwrap()
        );

        Ok(())
    }

    #[tokio::main]
    #[test]
    #[should_panic]
    async fn it_fails_to_post_a_short_user_name() {
        let user_str = r#"
    { 
        "name": "MF",
        "email": "fine@email.com"
    }"#;

        post_user(URL, user_str).await.unwrap();
    }

    #[tokio::main]
    #[test]
    #[should_panic]
    async fn it_fails_to_post_invalid_user_email() {
        let user_str = r#"
    { 
        "name": "My Name",
        "email": "bad"
    }"#;

        post_user(URL, user_str).await.unwrap();
    }
}
