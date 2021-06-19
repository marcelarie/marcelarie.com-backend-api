#![feature(proc_macro_hygiene, decl_macro)]

mod routes;

use routes::*;

#[macro_use]
extern crate rocket;

fn rocket_ignite() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![index])
}

fn main() {
    rocket_ignite().launch();
}
