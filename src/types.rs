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
