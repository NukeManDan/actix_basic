//! Tasks defined rust_actix-basic.pdf
//! Using endpoints at: https://jsonplaceholder.typicode.com/<API>

use validator::Validate;
use validator_derive;

use serde::{Deserialize, Serialize};

/// Struct used to serialize and deserialize responses from for a user
#[serde(default)] // needed for incomplete or partial User structs Serialize/Deserialize
#[derive(Serialize, Deserialize, validator_derive::Validate, Default, Debug, PartialEq)]
pub struct User {
    pub id: usize,
    #[validate(length(min = 3, max = 80))]
    pub name: String,
    pub username: String,
    #[validate(email)]
    pub email: String,
    pub address: Address,
    pub phone: String,
    pub website: String,
    pub company: Company,
}

/// Internal to `User` struct
#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct Address {
    pub street: String,
    pub suite: String,
    pub city: String,
    pub zipcode: String,
    pub geo: Geo,
}

/// Internal to `User` struct
#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct Geo {
    pub lat: String,
    pub lng: String,
}

/// Internal to `User` struct
#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct Company {
    pub name: String,
    #[serde(rename = "catchPhrase", default)]
    //snake case to camel for JSON format to keep idiomatic rust var names.
    pub catch_phrase: String, // NOTE: JSON uses catchPhrase
    pub bs: String,
}