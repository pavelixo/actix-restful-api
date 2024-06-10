mod controllers;
mod serializers;

use std::io::Result;
use actix_web::{App, HttpServer};

use controllers::post_controller::config;

#[actix_web::main]
async fn main() -> Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
