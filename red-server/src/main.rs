#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate dotenv;

pub mod models;
pub mod schema;
pub mod views;

use diesel::prelude::*;
use dotenv::dotenv;
use rocket_cors::AllowedOrigins;
use std::env;

pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn main() {
    dotenv().ok();

    let allowed_origins = AllowedOrigins::some(
        &["https://www.greshilov.me"],
        &["^https://(.+).greshilov.me$"],
    );
    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        ..Default::default()
    }
    .to_cors()
    .unwrap();
    rocket::ignite()
        .mount("/reds", routes![views::top_scores, views::submit_scores])
        .attach(cors)
        .launch();
}
