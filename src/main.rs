//! Tasks defined rust_actix-basic.pdf
//! using endpoint: https://jsonplaceholder.typicode.com/

#[macro_use]
extern crate validator_derive;
extern crate validator;

use tokio;

mod user;

const URL: &str = "https://jsonplaceholder.typicode.com/api/v1/users";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("******\nGetting users from {}...", URL);
    let users = user::get_users(URL).await?;
    println!(
        "******\nResponse: (Address of a user):\n {:#?}\n******",
        users[2].address
    );

    println!("-------------------------------------\n");

    let user_str = r#"
    {
        "name": "Martin Fowler",
        "email": "martin@martinfowler.com"
    }"#;
    //
    println!("******\nPosting new user:\n{}...", user_str);
    let new_user = user::post_user(URL, user_str).await?;
    println!("******\nResponse (full body):\n{:?}\n******", new_user);

    Ok(())
}
