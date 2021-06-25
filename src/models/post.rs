#![allow(proc_macro_derive_resolution_fallback)]

use crate::schema::posts;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

// the struct must be in the same order than the database object
#[derive(Serialize, Queryable, AsChangeset, Deserialize, Debug)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub body: String,
    pub created_at: NaiveDateTime,
    pub published: bool,
}

#[derive(AsChangeset, Insertable, Serialize, Deserialize, Debug)]
#[table_name = "posts"]
pub struct NewPost {
    pub title: Option<String>,
    pub body: Option<String>,
    pub description: Option<String>,
}
