use actix_web::{middleware, web, App, HttpServer};
use actix_files as fs;
use tera::Tera;
use std::{io, sync::Arc};

use Hush::{AppState, routes};

// Initialize the Tera template engine
fn init_tera() -> Tera {
    match Tera::new("templates/**/*.html") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            // Exit if template parsing fails
            ::std::process::exit(1);
        }
    }
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    // Initialize Tera once
    let tera_engine = init_tera();

    // Wrap Tera in AppState and web::Data for sharing
    let app_state = web::Data::new(AppState { tera: tera_engine });
    
    println!("Starting server at http://127.0.0.1:8080");

    HttpServer::new(move || {
        // Clone the web::Data for each worker thread
        App::new()
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .app_data(app_state.clone()) // Share the state
            .service(
                fs::Files::new("/public", "./public")
                    .show_files_listing()
                    // Disable file-not-found redirects
                    .disable_content_disposition()
            )
            .configure(routes::configure)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}