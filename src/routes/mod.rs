use std::env;

use diesel::result::Error;
use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;

use crate::connection::DbConn;
use crate::models::post;
use crate::models::post::{NewPost, Post};
use crate::repositories;

fn post_created(post: Post) -> status::Created<Json<Post>> {
    println!("here final");
    status::Created(
        format!(
            "{host}:{port}/post/{id}",
            host = host(),
            port = port(),
            id = post.id
        )
        .to_string(),
        Some(Json(post)),
    )
}

fn host() -> String {
    // env::var("ROCKET_ADDRESS").expect("ROCKET_ADDRESS must be set")
    String::from("localhost")
}

fn port() -> String {
    // env::var("ROCKET_PORT").expect("ROCKET_PORT must be set")
    String::from("8000")
}

fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError,
    }
}

#[get("/")]
pub fn all_posts(connection: DbConn) -> Result<Json<Vec<Post>>, Status> {
    repositories::show_all_posts(&connection)
        .map(|post| Json(post))
        .map_err(|error| error_status(error))
}

#[post("/", format = "application/json", data = "<new_post>")]
pub fn create_post(
    new_post: Json<NewPost>,
    connection: DbConn,
) -> Result<status::Created<Json<Post>>, Status> {
    println!("here 0 {}", &new_post.title);
    repositories::create_post(new_post.into_inner(), &connection)
        .map(|post| post_created(post))
        .map_err(|error| error_status(error))
}

#[get("/<id>")]
pub fn get_post(id: i32, connection: DbConn) -> Result<Json<Post>, Status> {
    repositories::get_post(id, &connection)
        .map(|post| Json(post))
        .map_err(|error| error_status(error))
}

#[put("/<id>", format = "application/json", data = "<post>")]
pub fn update_post(id: i32, post: Json<Post>, connection: DbConn) -> Result<Json<Post>, Status> {
    repositories::update_post(id, post.into_inner(), &connection)
        .map(|post| Json(post))
        .map_err(|error| error_status(error))
}

#[delete("/<id>")]
pub fn delete_post(id: i32, connection: DbConn) -> Result<status::NoContent, Status> {
    repositories::delete_post(id, &connection)
        .map(|_| status::NoContent)
        .map_err(|error| error_status(error))
}
