mod api;
mod db;
mod model;
mod types;

use actix_files::Files;
use actix_web::{middleware, web, App, HttpServer};
use dotenv::dotenv;
use std::io::Result;

#[actix_rt::main]
async fn main() -> Result<()> {
  dotenv().ok();
  env_logger::init();

  let db_pool = db::init().await;

  HttpServer::new(move || {
    App::new()
      .wrap(middleware::Logger::new("%s %r"))
      .data(db_pool.clone())
      .service(
        web::scope("/api")
          .route("/flags", web::get().to(api::get_all_flags))
          .route("/flags", web::post().to(api::create_flag))
          .route("/flags", web::put().to(api::update_flag)),
      )
      .service(Files::new("/", "./dist").index_file("index.html"))
  })
  .bind("127.0.0.1:3000")?
  .run()
  .await
}
