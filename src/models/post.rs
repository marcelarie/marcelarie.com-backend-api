// use super::schema::posts;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

impl Post {
}
