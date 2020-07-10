#[path = "./model.rs"]
mod model;

use actix_web::web::{Data, Json};
use actix_web::{HttpResponse, Responder};
use model::{CreateFlag, UpdateFlag};
use sqlx::PgPool;

pub async fn get_all_flags(pool: Data<PgPool>) -> impl Responder {
  let result = model::get_all_flags(pool.get_ref()).await;

  HttpResponse::Ok().json(result)
}

pub async fn create_flag(flag: Json<CreateFlag>, pool: Data<PgPool>) -> impl Responder {
  let result = model::create_flag(pool.get_ref(), flag).await;

  HttpResponse::Ok().json(result)
}

pub async fn update_flag(flag: Json<UpdateFlag>, pool: Data<PgPool>) -> impl Responder {
  let result = model::update_flag(pool.get_ref(), flag).await;

  HttpResponse::Ok().json(result)
}
