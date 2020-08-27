//! Tasks defined rust_actix-basic.pdf
//! using endpoint: https://jsonplaceholder.typicode.com/

#![allow(non_snake_case)] // JSON does not in general conform to rust standards, so we subdue warnings about correct case to use `serde::Deserialize` without the compiler complaining.

use tokio;

extern crate actix_basic;

const URL: &str = "https://jsonplaceholder.typicode.com/api/v1/users";


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    println!("******\nGetting users from {}...", URL);
    let users = actix_basic::get_users(URL).await?;
    println!("******\nResponse: (Address of a user):\n {:#?}\n******", users[2].address);

    println!("-------------------------------------\n");

    let user_str = r#"
    {
        "name": "Martin Fowler",
        "email": "martin@martinfowler.com"
    }"#;

    println!("******\nPosting new user:\n{}...", user_str);
    let new_user = actix_basic::post_user(URL, user_str).await?;
    println!("******\nResponse (full body):\n{:?}\n******", new_user);

    Ok(())
}