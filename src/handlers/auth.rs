use actix_web::{App, HttpResponse, Responder, Result, post, web};
use validator::Validate;

use crate::models::{LoginForm, SignupForm};

#[post("/signup")]
pub async fn signup_post(form: web::Json<SignupForm>) -> Result<HttpResponse> {
    let form = form.into_inner();

    println!("Username: {}, Password: {}, confirm_password: {}", form.username, form.password, form.confirm_password);

    if let Err(validation_errors) = form.validate() {
        return Ok(HttpResponse::BadRequest().body("invalid input"));
    }

    Ok(HttpResponse::Ok().into())
}