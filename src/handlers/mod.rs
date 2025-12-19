mod static_pages;
mod auth;

pub use static_pages::{index_get, login_get, signup_get};
pub use auth::signup_post;