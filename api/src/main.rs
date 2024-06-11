mod controllers;
mod serializers;
use std::io::Result;
use actix_web::{
    web::ServiceConfig, 
    App, 
    HttpServer
};
use controllers::{
    post_controller, 
    healthcheck_controller
};

// Server's binding address
const HOST: &'static str = "0.0.0.0:8080"; 

#[actix_web::main]
async fn main() -> Result<()> {
    // Define the app factory closure
    let app = || App::new().configure(config);

    // Create and run the HTTP server
    HttpServer::new(app).bind(HOST)?.run().await
}

// Main configuration controllers function 
fn config(cfg: &mut ServiceConfig) {
    post_controller::config(cfg);
    healthcheck_controller::config(cfg);
}
