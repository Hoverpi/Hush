use actix_web::{web, Result, HttpResponse, get};
use tera::Context;

use crate::AppState;

#[get("/")]
pub async fn index_get(data: web::Data<AppState>) -> Result<HttpResponse> {
    // Prepare data for the template (if any)
    let context = Context::new(); 
    
    // Render the template
    match data.tera.render("index.html", &context) {
        Ok(s) => Ok(HttpResponse::Ok().content_type("text/html").body(s)),
        Err(e) => {
            eprintln!("Template rendering error: {}", e);
            Ok(HttpResponse::InternalServerError().body("Template rendering error"))
        }
    }
}

#[get("/login")]
pub async fn login_get(data: web::Data<AppState>) -> Result<HttpResponse> {
    let context = Context::new();
    
    match data.tera.render("login.html", &context) {
        Ok(s) => Ok(HttpResponse::Ok().content_type("text/html").body(s)),
        Err(e) => {
            eprintln!("Template rendering error: {}", e);
            Ok(HttpResponse::InternalServerError().body("Template rendering error"))
        }
    }
}

#[get("/signup")] 
pub async fn signup_get(data: web::Data<AppState>) -> Result<HttpResponse> {
    let context = Context::new();
    
    match data.tera.render("signup.html", &context) {
        Ok(s) => Ok(HttpResponse::Ok().content_type("text/html").body(s)),
        Err(e) => {
            eprintln!("Template rendering error: {}", e);
            Ok(HttpResponse::InternalServerError().body("Template rendering error"))
        }
    }
}