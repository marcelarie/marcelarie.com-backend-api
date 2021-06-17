#![feature(proc_macro_hygiene, decl_macro)]

// Imports
#[macro_use]
extern crate rocket;
use rocket_contrib::json::Json;
use rusqlite::Connection;
use serde::Serialize;

// Data Structures
#[derive(Serialize)]
struct ToDoList {
    items: Vec<ToDoItem>,
}

#[derive(Serialize)]
struct ToDoItem {
    id: i64, item: String,
}

#[derive(Serialize)]
struct StatusMessage {
    message: String,
}

// Routes
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/todo")]
fn fetch_all_todo_items() -> Result<Json<ToDoList>, String> {
    let db_connect = match Connection::open("data.sqlite") {
        Ok(connection) => connection,
        Err(_) => return Err(String::from("Failed to connect to database")),
    };

    let mut statment = match db_connect.prepare("select id, item from todo_list;") {
        Ok(statment) => statment,
        Err(_) => return Err("Failed to return query".into()),
    };

    let results = statment.query_map([], |row| {
        Ok(ToDoItem {
            id: row.get(0)?,
            item: row.get(1)?,
        })
    });

    match results {
        Ok(rows) => {
            let collection: rusqlite::Result<Vec<_>> = rows.collect();

            match collection {
                Ok(items) => Ok(Json(ToDoList { items })),
                Err(_) => Err("Could not collect items".into()),
            }
        }
        Err(_) => Err("Failed to fetch todo items.".into()),
    }
}

#[post("/todo", format = "json", data = "<item>")]
fn post_todo_item(item: Json<String>) -> Result<Json<StatusMessage>, String> {
    let db_connect = match Connection::open("data.sqlite") {
        Ok(connection) => connection,
        Err(_) => return Err(String::from("Failed to connect to database")),
    };

    let mut statment =
        match db_connect.prepare("insert into todo_list (id, item) values (null, $1);") {
            Ok(statment) => statment,
            Err(_) => return Err("Failed to return query".into()),
        };
    let results = statment.execute(&[&item.0]);

    match results {
        Ok(rows_affected) => Ok(Json(StatusMessage {
            message: format!("{} rows inserted", rows_affected),
        })),
        Err(_) => Err("Failed to insert todo item".into()),
    }
}

#[delete("/todo/<id>")]
fn delete_todo_item(id: i64) -> Result<Json<StatusMessage>, String> {
    let db_connect = match Connection::open("data.sqlite") {
        Ok(connection) => connection,
        Err(_) => return Err(String::from("Failed to connect to database")),
    };

    let mut statment = match db_connect.prepare("delete from todo_list where id = $1;") {
        Ok(statment) => statment,
        Err(_) => return Err("Failed to return query".into()),
    };
    let results = statment.execute(&[&id]);

    match results {
        Ok(rows_affected) => Ok(Json(StatusMessage {
            message: format!("{} rows deleted!", rows_affected),
        })),
        Err(_) => Err("Failed to delete todo item".into()),
    }
}

// Datbase Connection
fn main() {
    let db_connect = Connection::open("data.sqlite").unwrap();

    db_connect
        .execute(
            "create table if not exists todo_list (
                id integer primary key,
                item varchar(64) not null
            );",
            [],
        )
        .unwrap();

    rocket::ignite()
        .mount(
            "/",
            routes![
                index,
                fetch_all_todo_items,
                post_todo_item,
                delete_todo_item
            ],
        )
        .launch();
}
