#[macro_use]
extern crate diesel;
extern crate dotenv;

use crate::handlers::items::*;

use actix_web::{web, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

mod config;
mod handlers;
mod models;
mod schema;

pub type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[actix_web::main]

async fn main() -> std::io::Result<()> {
    let config = config::load_config();
    let manager = ConnectionManager::<SqliteConnection>::new(config.database.url);
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to build pool");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/", web::get().to(get_items))
            .route("/", web::post().to(create_items))
            .route("/{id}", web::get().to(get_item))
            .route("/{id}", web::get().to(delete_item))
    })
    .bind("localhost:8000")?
    .run()
    .await
}
