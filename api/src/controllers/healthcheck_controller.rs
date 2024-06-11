use actix_web::{
    HttpResponse, 
    Responder, 
    web::{
        ServiceConfig, 
        resource, 
        get
    }
};
use crate::serializers::healthcheck_serializer::Healthcheck;

// Controller
async fn health_check() -> impl Responder {
    let response = Healthcheck {
        message: String::from("Ok"),
        version: env!("CARGO_PKG_VERSION").to_string(),
    };

    HttpResponse::Ok().json(response)
}

// Service configuration
pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        resource("/health").route(
            get().to(health_check)
        )
    );
}
