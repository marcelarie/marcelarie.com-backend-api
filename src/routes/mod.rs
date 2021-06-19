use rocket_contrib::json::Json;

#[get("/")]
pub fn index() -> Json<String> {
    Json(String::from("Hello, world!"))
}
