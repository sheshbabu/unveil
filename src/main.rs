mod api;
mod db;

use actix_files as fs;
use actix_web::{middleware, web, App, HttpServer};
use dotenv::dotenv;
use std::io::Result;

#[actix_rt::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env_logger::init();

    let database_url =
        std::env::var("DATABASE_URL").expect("Please set the DATABASE_URL environment variable");

    let db_pool = db::init(database_url).await;

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::DefaultHeaders::new().header("X-Version", "0.2"))
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .data(db_pool.clone())
            .service(
                web::scope("/api")
                    .route("/flags", web::get().to(api::get_all_flags))
                    .route("/flags", web::post().to(api::create_flag))
                    .route("/flags", web::put().to(api::update_flag)),
            )
            .service(fs::Files::new("/", "./dist").index_file("index.html"))
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}
