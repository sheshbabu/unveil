use crate::api;
use actix_web::{web, Scope};

pub fn get_routes() -> Scope {
  web::scope("/api")
    .route("/flags", web::get().to(api::get_all_flags))
    .route("/flags", web::post().to(api::create_flag))
    .route("/flags", web::put().to(api::update_flag))
}
