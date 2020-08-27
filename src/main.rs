//! Tasks defined rust_actix-basic.pdf
//! using endpoint: https://jsonplaceholder.typicode.com/

#![allow(non_snake_case)] // JSON does not in general conform to rust standards, so we subdue warnings about correct case to use `serde::Deserialize` without the compiler complaining.

use reqwest;
use tokio;
use serde::{Serialize,Deserialize};
use serde_json::{Value};


const URL: &str = "https://jsonplaceholder.typicode.com/api/v1/users";


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    println!("Getting users from {}...", URL);
    let users = get_users(URL).await?;
    println!("---Response: (Address of a user):\n {:#?}", users[2].address);

    let user_str = r#"
    {
        "name": "Martin Fowler",
        "email": "martin@martinfowler.com"
    }"#;

    println!("Posting new user:\n{}...", user_str);
    let new_user = post_user(URL, user_str).await?;
    println!("---Response (full body):\n{:#?}", new_user);

    Ok(())
}

/// Submits a GET request to a specified URL, and returns a vector of users. Expected response is in json format, and can be parsed by the `User` struct with accessible elements by key name.
async fn get_users (url: &str) -> Result<Vec<User>,Box<dyn std::error::Error>> {
    let users = reqwest::get(url)
    .await?
    .json::<Vec<User>>()
    .await?;

    Ok(users)
}


/// Post a new user with a JSON formatted raw string (&str) passed into `post_user`.
/// 
/// > Note: The name will need to be at least 3 characters long and the email address should be valid.
 
async fn post_user (url: &str, user_json: &str) -> Result<Value, Box<dyn std::error::Error>> {
    // The name will need to be at least 3 characters long and the email address should be valid.
    // Validate the length of the name and the validity of the email address.
    // Intentionally trigger the error case below.

    let post: Value = serde_json::from_str(user_json)?;

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
struct User{
    id: usize,
    name: String,
    username: String,
    email: String,
    address: Address,
    phone: String,
    website: String,
    company: Company,
}

#[derive(Serialize,Deserialize,Debug)]
struct Address{
    street: String,
    suite: String,
    city: String,
    zipcode: String,
    geo: Geo,
}

#[derive(Serialize,Deserialize,Debug)]
struct Geo{
    lat: String,
    lng: String,
}

#[derive(Serialize,Deserialize,Debug)]
struct Company{
    name: String,
    catchPhrase: String, // NOTE: JSON uses catchPhrase
    bs: String,
}