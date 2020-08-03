use crate::error::Error;
use crate::model;
use crate::types::{CreateFlag, UpdateFlag};
use actix_web::web::{Data, Json};
use actix_web::HttpResponse;
use sqlx::PgPool;

type ApiResult = Result<HttpResponse, Error>;

pub async fn get_all_flags(pool: Data<PgPool>) -> ApiResult {
  let flags = model::get_all_flags(pool.get_ref()).await?;
  Ok(HttpResponse::Ok().json(flags))
}

pub async fn create_flag(flag: Json<CreateFlag>, pool: Data<PgPool>) -> ApiResult {
  model::create_flag(pool.get_ref(), flag).await?;
  Ok(HttpResponse::Ok().finish())
}

pub async fn update_flag(flag: Json<UpdateFlag>, pool: Data<PgPool>) -> ApiResult {
  model::update_flag(pool.get_ref(), flag).await?;
  Ok(HttpResponse::Ok().finish())
}
