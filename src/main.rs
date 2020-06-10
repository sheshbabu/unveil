use actix_files as fs;
use actix_web::{middleware, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgQueryAs;
use sqlx::{FromRow, PgPool};
use std::io::Result;

#[derive(FromRow, Serialize, Deserialize, Debug)]
struct Flag {
    name: String,
    is_enabled: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct CreateFlag {
    name: String,
}

async fn get_all_flags(db_pool: web::Data<PgPool>) -> impl Responder {
    let result = sqlx::query_as::<_, Flag>("SELECT name, is_enabled from flags")
        .fetch_all(db_pool.get_ref())
        .await
        .unwrap();

    HttpResponse::Ok().json(result)
}

async fn create_flag(item: web::Json<CreateFlag>, db_pool: web::Data<PgPool>) -> impl Responder {
    let result = sqlx::query_as::<_, Flag>("INSERT INTO flags (name) VALUES ($1)")
        .bind(&item.name)
        .fetch_all(db_pool.get_ref())
        .await
        .unwrap();

    HttpResponse::Ok().json(result)
}

async fn update_flag(item: web::Json<Flag>, db_pool: web::Data<PgPool>) -> impl Responder {
    let result = sqlx::query_as::<_, Flag>("UPDATE flags SET is_enabled = $2 WHERE name = $1;")
        .bind(&item.name)
        .bind(&item.is_enabled)
        .fetch_all(db_pool.get_ref())
        .await
        .unwrap();

    HttpResponse::Ok().json(result)
}

#[actix_rt::main]
async fn main() -> Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();

    let database_url = std::env::var("DATABASE_URL").unwrap();
    let db_pool = PgPool::new(&database_url).await.unwrap();

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::DefaultHeaders::new().header("X-Version", "0.2"))
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .data(db_pool.clone())
            .service(
                web::scope("/api")
                    .route("/flags", web::get().to(get_all_flags))
                    .route("/flags", web::post().to(create_flag))
                    .route("/flags", web::put().to(update_flag)),
            )
            .service(fs::Files::new("/", "./src/web").index_file("index.html"))
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}
