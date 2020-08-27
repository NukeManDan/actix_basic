//! Tasks defined rust_actix-basic.pdf
//! Using endpoints at: https://jsonplaceholder.typicode.com/<API>

#![allow(non_snake_case)] // JSON does not in general conform to rust standards, so we suppress warnings about correct case to use `serde::Deserialize` without the compiler complaining.

use reqwest;
use serde::{Serialize,Deserialize};
use serde_json::{Value};

use regex::Regex;

/// Submits a GET request to a specified URL, and returns a vector of users. Expected response is in json format, and can be parsed by the `User` struct with accessible elements by key name.
pub async fn get_users (url: &str) -> Result<Vec<User>,Box<dyn std::error::Error>> {
    let users = reqwest::get(url)
    .await?
    .json::<Vec<User>>()
    .await?;

    Ok(users)
}


/// `POST` a new user with a JSON formatted raw string (&str) passed into `post_user`. Returns a  `serde_json::Value` object with the body of the response.
/// 
/// The name will need to be at least 3 characters long and the email address should be valid.
/// 
/// ## Errors
/// 
/// This will panic if the name is less than 3 characters, or email is not valid.
/// 
/// > Note: The name will need to be at least 3 characters long and the email address should be valid.
pub async fn post_user (url: &str, user_json: &str) -> Result<Value, Box<dyn std::error::Error>> {
   
    let post: Value = serde_json::from_str(user_json)?;

    if post.get("name")
        .expect("name not found in `user_json`") // throw if json is malformed
        .as_str()
        .expect("name must be a valid string") // throw if name is not a string (an int for example)
        .len() < 3 {
            panic!("name must be more than three characters");
            // FIXME: need to throw error here that produces a RESPONSE that we need, as the API throws a 201
    }

    // use know robust regex to validate email: https://emailregex.com/
    let re = Regex::new(r#"(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])"#).unwrap();

    // Err passed if email is bad, and the response is `422 Unprocessable Entity`
    if !re.is_match(post.get("email").unwrap().as_str().unwrap_or(&"")){
        panic!("email is invalid (must match expected pattern)");
            // FIXME: need to throw error here that produces a RESPONSE that we need, as the API throws a 201
    }

    let client = reqwest::Client::new();
    let resp = client.post(url)
        .json(&post)
        .send()
        .await?;

    let body = serde_json::from_str(
            resp
            .text()
            .await?
            .as_str()
            )?;

    Ok(body)
}


/// Defining Structs used to serialize and deserialize responses from reqwest for a user

#[derive(Serialize,Deserialize,Debug)]
pub struct User{
    pub id: usize,
    pub name: String,
    pub username: String,
    pub email: String,
    pub address: Address,
    pub phone: String,
    pub website: String,
    pub company: Company,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct Address{
    pub street: String,
    pub suite: String,
    pub city: String,
    pub zipcode: String,
    pub geo: Geo,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct Geo{
    pub lat: String,
    pub lng: String,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct Company{
    pub name: String,
    pub catchPhrase: String, // NOTE: JSON uses catchPhrase
    pub bs: String,
}