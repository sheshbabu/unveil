use include_dir::{include_dir, Dir};
use sqlx::PgPool;
use sqlx_pg_migrate::migrate;

static MIGRATIONS: Dir = include_dir!("src/migrations");

pub async fn init() -> PgPool {
  let database_url = std::env::var("DATABASE_URL").expect("Please set the DATABASE_URL env var");
  log::info!("Creating connection pool");
  let db_pool = PgPool::new(&database_url)
    .await
    .expect("Error while creating connection pool");

  log::info!("Performing database migrations");
  migrate(&database_url, &MIGRATIONS)
    .await
    .expect("Error while performing database migrations");

  return db_pool;
}
