#[macro_use]
extern crate diesel;

use actix_web::{web, App, HttpServer};
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

mod db;
mod handlers;
mod models;
mod schema;

#[actix_web::main]

async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL debe estar en .env");

    // Crear conexión a la base de datos
    db::establish_connection();

    println!("Servidor corriendo en http://localhost:8080");

    HttpServer::new(|| {
        App::new().configure(handlers::config_routes) // Configurar rutas
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
