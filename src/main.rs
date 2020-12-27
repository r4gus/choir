#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use choir::attach_database;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    attach_database(choir::rocket()).launch();
}

