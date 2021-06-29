#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate serde;

pub mod models;
pub mod recipes;
pub mod schema;

use crate::recipes::{delete_recipe, get_recipe, get_recipes, patch_recipe, post_recipe};
use actix_web::{web, App, HttpServer};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use std::sync::Mutex;

struct AppState {
    pool: PgConnection,
}

fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let conn = establish_connection();
    let data = web::Data::new(Mutex::new(AppState { pool: conn }));

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(get_recipes)
            .service(get_recipe)
            .service(post_recipe)
            .service(patch_recipe)
            .service(delete_recipe)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
