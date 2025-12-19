pub mod routes;
pub mod handlers;
pub mod models;

pub use routes::configure;
pub use handlers::{index_get, login_get, signup_get};

// Helper struct to hold the Tera template engine for application state
use tera::Tera;
pub struct AppState {
    pub tera: Tera,
    // pub user_svc: Arc<dyn UserService + Send + Sync>,
}