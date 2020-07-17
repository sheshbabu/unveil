use super::types::{CreateFlag, Flag, UpdateFlag};
use actix_web::web::Json;
use sqlx::{Error, PgPool};
use std::result::Result;

pub async fn get_all_flags(pool: &PgPool) -> Result<Vec<Flag>, Error> {
  let result = sqlx::query_as!(
    Flag,
    "
    SELECT
      *
    FROM
      flags
    ORDER BY
      is_on DESC,
      updated_at DESC
    ",
  )
  .fetch_all(pool)
  .await?;

  return Ok(result);
}

pub async fn create_flag(pool: &PgPool, flag: Json<CreateFlag>) -> Result<(), Error> {
  sqlx::query!(
    "
    INSERT INTO
      flags (key, name, description)
    VALUES
      ($1, $2, $3)
    ",
    flag.key,
    flag.name,
    flag.description
  )
  .execute(pool)
  .await?;

  return Ok(());
}

pub async fn update_flag(pool: &PgPool, flag: Json<UpdateFlag>) -> Result<(), Error> {
  sqlx::query!(
    "
    UPDATE
      flags
    SET
      key = $2,
      name = $3,
      description = $4,
      is_on = $5
    WHERE
      id = $1
    ",
    flag.id,
    flag.key,
    flag.name,
    flag.description,
    flag.is_on
  )
  .execute(pool)
  .await?;

  return Ok(());
}
