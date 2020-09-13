mod api;
mod db;
mod error;
mod model;
mod routes;
mod types;

use actix_files::Files;
use actix_web::{middleware, App, HttpServer};
use dotenv::dotenv;
use std::io::Result;

#[actix_web::main]
async fn main() -> Result<()> {
  dotenv().ok();
  env_logger::init();

  let db_pool = db::init().await;

  HttpServer::new(move || {
    App::new()
      .wrap(middleware::Logger::new("%s %r"))
      .data(db_pool.clone())
      .service(routes::get_routes())
      .service(Files::new("/", "./dist").index_file("index.html"))
  })
  .bind("127.0.0.1:3000")?
  .run()
  .await
}
