#[macro_use]
extern crate diesel;

extern crate dotenv;

use dotenv::dotenv;
use std::env;
use diesel::prelude::*;
use diesel::pg::PgConnection;

mod schema;
mod models;

fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("set DATABASE_URL");
    let mut conn = PgConnection::establish(&database_url).expect("Failed to connect to database");


    let book = models::NewBook {
        title: String::from("Gravity's Rainbow"),
        author: String::from("Thomas Pynchon"),
        published: true,
    };

    if models::Book::insert(book, &mut conn) {
        println!("success");
    } else {
        println!("failed");
    }
}