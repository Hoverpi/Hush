use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct SignupForm {
    #[validate(length(min = 3, max = 32))]
    pub username: String,

    #[validate(length(min = 8))]
    pub password: String,

    #[validate(length(min = 8))]
    pub confirm_password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginForm {
    #[validate(length(min = 3, max = 32))]
    pub username: String,

    #[validate(length(min = 8))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct NewUser {
    #[validate(length(min = 3, max = 32))]
    pub username: String,

    #[validate(length(min = 8))]
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    // never serialize password_hash to client
    #[serde(skip)]
    pub password_hash: String,
}
