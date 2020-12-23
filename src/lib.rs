#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket_contrib::{
    templates::Template,
    serve::StaticFiles,
};

pub mod auth; // Authentication module for login and sign-up

pub mod application;

#[cfg(test)]
pub mod test;

pub fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![auth::login, auth::login_form, application::dashboard])
                    .attach(Template::fairing())
                    .mount("/static", StaticFiles::from("static"))
}
