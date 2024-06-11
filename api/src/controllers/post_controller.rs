use actix_web::{
  HttpRequest, 
  HttpResponse, 
  web::{
    ServiceConfig, 
    Json, 
    resource, 
    post
  }
};
use crate::serializers::post_serializer::Post;

// Controller
async fn create_post(payload: Json<Post>, _request: HttpRequest) -> HttpResponse {
  let data = payload.into_inner();

  HttpResponse::Created().json(data)
}

// Service configuration
pub fn config(cfg: &mut ServiceConfig) {
  cfg.service(
    resource("/posts").route(
      post().to(create_post)
    )
  );
}