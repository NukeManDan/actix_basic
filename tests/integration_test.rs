//! Tests for `get_user` and `post_user` methods
//! Note: the endpoints for this lib are HARD CODED. only to be used with `https://jsonplaceholder.typicode.com/api/v1/users`

extern crate actix_basic;

use tokio;
use serde_json::json;

const URL: &str = "https://jsonplaceholder.typicode.com/api/v1/users";

#[tokio::main]
#[test]
async fn it_gets_users() -> Result<(),Box<dyn std::error::Error>>{

    let address_check =  actix_basic::Address {
        street: String::from("Douglas Extension"),
        suite: String::from("Suite 847"),
        city: String::from("McKenziehaven"),
        zipcode: String::from("59590-4157"),
        geo: actix_basic::Geo {
            lat: String::from("-68.6102"),
            lng: String::from("-47.0653"),
        },
    };

    let users: Vec<actix_basic::User> = actix_basic::get_users(URL).await?;
    // println!("---Response: (Address of a user):\n {:#?}", users[2].address.street);

    assert_eq!(address_check.street, users[2].address.street);

    Ok(())

}

#[tokio::main]
#[test]
async fn it_posts_a_user() -> Result<(),Box<dyn std::error::Error>>{
    let user_str = r#"
    {
        "name": "Martin Fowler",
        "email": "martin@martinfowler.com"
    }"#;

    let resp_json_expected = json!({ "apiId": String::from("v1"), "email": String::from("martin@martinfowler.com"), "id": 11, "name": String::from("Martin Fowler")});

    // println!("Posting new user:\n{}...", user_str);
    let new_user = actix_basic::post_user(URL, user_str).await?;
    // println!("---Response (full body):\n{:#?}", new_user);

    assert_eq!(resp_json_expected.get("email").unwrap(), new_user.get("email").unwrap());


    Ok(())
}

#[tokio::main]
#[test]
#[should_panic(expected = "name must be more than three characters")]
async fn it_fails_to_post_a_short_user_name(){
    let user_str = r#"
    { 
        "name": "MF",
        "email": "fine@email.com"
    }"#;

    actix_basic::post_user(URL, user_str).await.unwrap();
}

#[tokio::main]
#[test]
#[should_panic(expected = "email is invalid (must match expected pattern)")]
async fn it_fails_to_post_invalid_user_email(){
    let user_str = r#"
    { 
        "name": "My Name",
        "email": "bad"
    }"#;

    actix_basic::post_user(URL, user_str).await.unwrap();
}