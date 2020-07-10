use actix_web::web::Json;
use sqlx::postgres::PgQueryAs;
use sqlx::PgPool;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct Flag {
  pub id: i32,
  pub key: String,
  pub name: String,
  pub description: String,
  pub is_on: bool,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateFlag {
  pub key: String,
  pub name: String,
  pub description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateFlag {
  pub id: i32,
  pub key: String,
  pub name: String,
  pub description: String,
  pub is_on: bool,
}

pub async fn get_all_flags(pool: &PgPool) -> Vec<Flag> {
  let statement = "
      SELECT
          *
      FROM
          flags
      ORDER BY
          is_on DESC,
          updated_at DESC
  ";
  let result: Vec<Flag> = sqlx::query_as(statement).fetch_all(pool).await.unwrap();

  return result;
}

pub async fn create_flag(pool: &PgPool, flag: Json<CreateFlag>) -> Vec<Flag> {
  let statement = "
      INSERT INTO
          flags (
              key,
              name,
              description
          )
      VALUES
          ($1, $2, $3)
  ";
  let result: Vec<Flag> = sqlx::query_as(statement)
    .bind(&flag.key)
    .bind(&flag.name)
    .bind(&flag.description)
    .fetch_all(pool)
    .await
    .unwrap();

  return result;
}

pub async fn update_flag(pool: &PgPool, flag: Json<UpdateFlag>) -> Vec<Flag> {
  let statement = "
      UPDATE
          flags
      SET
          key = $2,
          name = $3,
          description = $4,
          is_on = $5
      WHERE
          id = $1
  ";
  let result: Vec<Flag> = sqlx::query_as(statement)
    .bind(&flag.id)
    .bind(&flag.key)
    .bind(&flag.name)
    .bind(&flag.description)
    .bind(&flag.is_on)
    .fetch_all(pool)
    .await
    .unwrap();

  return result;
}
