use std::io::Result;
use actix_web::{App, HttpServer};

mod controllers;
mod serializers;
use controllers::{post_controller, healthcheck_controller};

const HOST: &'static str = "0.0.0.0";
const PORT: u16 = 8080;

#[actix_web::main]
async fn main() -> Result<()> {
    HttpServer::new(|| {
            App::new()
                .configure(post_controller::config)
                .configure(healthcheck_controller::config)
        }
    )
    .bind((HOST, PORT))?
    .run()
    .await
}
