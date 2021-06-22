#![feature(decl_macro, proc_macro_hygiene)]
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
#[macro_use]
extern crate rocket;
extern crate chrono;
extern crate rocket_contrib;

mod connection;
mod models;
mod schema;

use crate::schema::posts;
use connection::DbConn;
use diesel::prelude::*;
use models::post::{NewPost, Post};
use rocket::http::Status;
use rocket_contrib::json::Json;

// Routes (Handlers)
// 1. Add new blog post ---[POST]-> /posts
#[post("/", format = "application/json", data = "<new_post>")]
fn create_post(new_post: Json<NewPost>, connection: DbConn) -> Result<Json<Post>, Status> {
    let result = diesel::insert_into(posts::table)
        .values(&new_post.0)
        .get_result(&*connection) // <-- `&*c` init_pool connections defers to PgConnection
        .map(|post| Json(post))
        .map_err(|_| Status::InternalServerError);

    result
}

// Repository

fn rocket_ignite() -> rocket::Rocket {
    rocket::ignite()
        .manage(connection::init_pool())
        .mount("/posts", routes![create_post])
    // .mount("/", routes![all_posts, create_post, get_post, update_post, delete_post ])
}

fn main() {
    rocket_ignite().launch();
}
