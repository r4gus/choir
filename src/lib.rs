#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate diesel;

use rocket_contrib::{
    templates::Template,
    serve::StaticFiles,
};

pub mod auth; // Authentication module for login and sign-up
pub mod application;
pub mod schema; // Database schema for Diesel
pub mod models; // Database models for Diesel

#[cfg(test)]
pub mod test;

#[database("postgres_db")]
pub struct DbConn(diesel::PgConnection);

pub fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![auth::login, auth::login_form, application::dashboard])
                    .attach(Template::fairing())
                    .mount("/static", StaticFiles::from("static"))
}
