use actix_files as fs;
use actix_web::{middleware, web, App, HttpResponse, HttpServer, Responder};
use chrono::{DateTime, Utc};
use dotenv::dotenv;
use include_dir::{include_dir, Dir};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgQueryAs;
use sqlx::{FromRow, PgPool};
use sqlx_pg_migrate::migrate;
use std::io::Result;

static MIGRATIONS: Dir = include_dir!("src/migrations");

#[derive(FromRow, Serialize, Deserialize, Debug)]
struct Flag {
    id: i32,
    key: String,
    name: String,
    description: String,
    is_on: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
struct CreateFlag {
    key: String,
    name: String,
    description: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct UpdateFlag {
    id: i32,
    key: String,
    name: String,
    description: String,
    is_on: bool,
}

async fn get_all_flags(db_pool: web::Data<PgPool>) -> impl Responder {
    let statement = "
        SELECT
            *
        FROM
            flags
        ORDER BY
            is_on DESC,
            updated_at DESC
    ";
    let result = sqlx::query_as::<_, Flag>(statement)
        .fetch_all(db_pool.get_ref())
        .await
        .unwrap();

    HttpResponse::Ok().json(result)
}

async fn create_flag(item: web::Json<CreateFlag>, db_pool: web::Data<PgPool>) -> impl Responder {
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
    let result = sqlx::query_as::<_, Flag>(statement)
        .bind(&item.key)
        .bind(&item.name)
        .bind(&item.description)
        .fetch_all(db_pool.get_ref())
        .await
        .unwrap();

    HttpResponse::Ok().json(result)
}

async fn update_flag(item: web::Json<UpdateFlag>, db_pool: web::Data<PgPool>) -> impl Responder {
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
    let result = sqlx::query_as::<_, Flag>(statement)
        .bind(&item.id)
        .bind(&item.key)
        .bind(&item.name)
        .bind(&item.description)
        .bind(&item.is_on)
        .fetch_all(db_pool.get_ref())
        .await
        .unwrap();

    HttpResponse::Ok().json(result)
}

#[actix_rt::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env_logger::init();

    let database_url =
        std::env::var("DATABASE_URL").expect("Please set the DATABASE_URL environment variable");
    let db_pool = PgPool::new(&database_url).await.unwrap();
    migrate(&database_url, &MIGRATIONS).await.unwrap();

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
            .service(fs::Files::new("/", "./dist").index_file("index.html"))
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}
