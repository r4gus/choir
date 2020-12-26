#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_migrations;

use rocket_contrib::{
    templates::Template,
    serve::StaticFiles,
};

pub mod auth; // Authentication module for login and sign-up
pub mod application;
pub mod schema; // Database schema for Diesel
pub mod models; // Database models for Diesel
pub mod database; // Database functions

#[cfg(test)]
pub mod test;

diesel_migrations::embed_migrations!("migrations");

// Database connection used for production
#[database("postgres_db")]
pub struct DbConn(diesel::PgConnection);

pub fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![auth::login, auth::login_form, application::dashboard])
                    .attach(Template::fairing())
                    .mount("/static", StaticFiles::from("static"))
}
