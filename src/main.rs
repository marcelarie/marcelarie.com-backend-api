#![feature(decl_macro, proc_macro_hygiene)]
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

use dotenv::dotenv;

mod connection;
mod models;
mod repositories;
mod routes;
mod schema;

use routes::*;

fn rocket_ignite() -> rocket::Rocket {
    rocket::ignite()
        .manage(connection::init_pool())
        .mount("/", routes![all_posts, create_post, get_post, update_post, delete_post ])
}

fn main() {
    rocket_ignite().launch();
}
