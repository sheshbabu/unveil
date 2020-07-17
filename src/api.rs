use super::model;
use super::types::{CreateFlag, UpdateFlag};
use actix_web::web::{Data, Json};
use actix_web::{HttpResponse, Responder};
use sqlx::PgPool;

pub async fn get_all_flags(pool: Data<PgPool>) -> impl Responder {
  model::get_all_flags(pool.get_ref())
    .await
    .map(|flags| HttpResponse::Ok().json(flags))
    .map_err(|error| {
      log::error!("Error {:?}", error);
      HttpResponse::InternalServerError()
    })
}

pub async fn create_flag(flag: Json<CreateFlag>, pool: Data<PgPool>) -> impl Responder {
  model::create_flag(pool.get_ref(), flag)
    .await
    .map(|_| HttpResponse::Ok())
    .map_err(|error| {
      log::error!("Error {:?}", error);
      HttpResponse::InternalServerError()
    })
}

pub async fn update_flag(flag: Json<UpdateFlag>, pool: Data<PgPool>) -> impl Responder {
  model::update_flag(pool.get_ref(), flag)
    .await
    .map(|_| HttpResponse::Ok())
    .map_err(|error| {
      log::error!("Error {:?}", error);
      HttpResponse::InternalServerError()
    })
}
