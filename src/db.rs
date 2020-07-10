use include_dir::{include_dir, Dir};
use sqlx::PgPool;
use sqlx_pg_migrate::migrate;

static MIGRATIONS: Dir = include_dir!("src/migrations");

pub async fn init(database_url: String) -> PgPool {
  let db_pool = PgPool::new(&database_url).await.unwrap();
  migrate(&database_url, &MIGRATIONS).await.unwrap();
  return db_pool;
}
