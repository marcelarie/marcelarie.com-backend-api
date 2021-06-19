#![allow(proc_macro_derive_resolution_fallback)]

use crate::schema::posts;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, AsChangeset, Queryable, Deserialize, Debug)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub created_at: NaiveDateTime,
    pub published: bool,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "posts"]
pub struct NewPost {
    pub title: String,
    pub body: String,
}
