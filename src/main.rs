//! Tasks defined rust_actix-basic.pdf
//! using endpoint: https://jsonplaceholder.typicode.com/

use log::info;
use simplelog::*;
use tokio;

mod user;

const URL: &str = "https://jsonplaceholder.typicode.com/api/v1/users";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = SimpleLogger::init(LevelFilter::Info, Config::default());

    info!("Getting users from {}...", URL);
    let users = user::get_users(URL).await?;
    info!("Response: (Address of a user):\n {:#?}", users[2].address);

    let user_str = r#"
    {
        "name": "Martin Fowler",
        "email": "martin@martinfowler.com"
    }"#;
    //
    info!("Posting new user:{}", user_str);
    let new_user = user::post_user(URL, user_str).await?;
    info!("Response (full body):\n{:?}", new_user);

    Ok(())
}
