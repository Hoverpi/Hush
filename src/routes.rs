use actix_web::web::{self, service};

use crate::handlers::{index_get, login_get, signup_get, signup_post};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(index_get)
       .service(login_get)
       .service(signup_get)
       .service(signup_post);
}