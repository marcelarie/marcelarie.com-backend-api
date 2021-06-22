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
use rocket_contrib::json::Json;
// Responses
use diesel::result::Error;

// Routes (Handlers)
// 1. Add new blog post ---[POST]-> /posts
#[post("/", format = "application/json", data = "<new_post>")]
fn create_post(new_post: Json<NewPost>, connection: DbConn) -> Result<Json<Post>, Error> {
    let result = diesel::insert_into(posts::table)
        .values(&new_post.0)
        .get_result(&*connection)?; // <-- `&*c` init_pool connections defers to PgConnection

    Ok(Json(result))
}
// 2. Get all blog posts ---[GET]-> /posts/all
#[get("/all")]
fn get_all_posts(connection: DbConn) -> Result<Json<Vec<Post>>, Error> {
    let result = posts::table.load::<Post>(&*connection)?;

    Ok(Json(result))
}

// Repository

fn rocket_ignite() -> rocket::Rocket {
    rocket::ignite()
        .manage(connection::init_pool())
        .mount("/posts", routes![create_post, get_all_posts])
    // .mount("/", routes![all_posts, create_post, get_post, update_post, delete_post ])
}

fn main() {
    rocket_ignite().launch();
}
