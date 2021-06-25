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
// Responses/ Errors
use diesel::result::Error;
use rocket::response::Debug;

use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response};

pub struct CORS;

impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }
    fn on_response(&self, _request: &Request, response: &mut Response) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

// Routes (Handlers)
// 1. Add new blog post ---[POST]-> /posts
#[post("/", format = "application/json", data = "<new_post>")]
fn create_post(new_post: Json<NewPost>, connection: DbConn) -> Result<Json<Post>, Debug<Error>> {
    let result = diesel::insert_into(posts::table)
        .values(&new_post.0)
        .get_result(&*connection)?; // <-- `&*c` init_pool connections defers to PgConnection

    Ok(Json(result))
}
// 2. Get all blog posts ---[GET]-> /posts/all
#[get("/all")]
fn get_all_posts(connection: DbConn) -> Result<Json<Vec<Post>>, Debug<Error>> {
    let result = posts::table.load::<Post>(&*connection)?;

    Ok(Json(result))
}

// 3. Get post ---[GET]-> /posts/:id
#[get("/<id>")]
fn get_post_by_id(id: i32, connection: DbConn) -> Result<Json<Post>, Debug<Error>> {
    let result = posts::table.find(id).get_result::<Post>(&*connection)?;
    println!("{:#?}", result);

    Ok(Json(result))
}

// 4. Delete post ---[DELETE]-> /posts/:id
#[delete("/<id>")]
fn delete_post_by_id(id: i32, connection: DbConn) -> Result<Json<usize>, Debug<Error>> {
    let result = diesel::delete(posts::table.find(id)).execute(&*connection)?;
    Ok(Json(result))
}

// 5. Get user ---[GET]-> /user/:id
// 7. Add new user ---[POST]-> /user/new
// 9. Add new comment ---[POST]-> /comment/new

// Repository

fn rocket_ignite() -> rocket::Rocket {
    rocket::ignite()
        .attach(CORS)
        .manage(connection::init_pool())
        .mount(
            "/posts",
            routes![create_post, get_all_posts, get_post_by_id, delete_post_by_id],
        )
    // .mount("/", routes![all_posts, create_post, get_post, update_post, delete_post ])
}

fn main() {
    rocket_ignite().launch();
}
