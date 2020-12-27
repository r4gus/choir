#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_migrations;
extern crate argon2;
extern crate dotenv;
extern crate rand;


use rocket_contrib::{
    templates::Template,
    serve::StaticFiles,
};
use dotenv::dotenv;
use std::env;
use crate::models::User;
use crate::database::create_user;

pub mod auth; // Authentication module for login and sign-up
pub mod application;
pub mod schema; // Database schema for Diesel
pub mod models; // Database models for Diesel
pub mod database; // Database functions

#[cfg(test)]
pub mod test;

// Embed diesel migrations into binary
diesel_migrations::embed_migrations!("migrations");

// Database connection used for production
#[database("postgres_db")]
pub struct DbConn(diesel::PgConnection);

/// Create a rocket instance to launch.
///
/// This function mounts the required paths as well as a static directory and attaches the template fairing.
/// To also connect a database pass the rocket instance to the `attach_database` function.
pub fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/",
                           routes![auth::login, auth::login_form,
                                application::dashboard, application::admin_panel_redirect,
                                auth::logout, application::index
                                ])
                    .attach(Template::fairing())
                    .mount("/static", StaticFiles::from("static"))
}

/// Attach the database fairing of `DbConn` to a rocket instance.
///
/// If the environment variables `DEFAULT_ADMIN_PASSWORD` and
/// `DEFAULT_ADMIN_EMAIL` are set, the function tries to create an
/// admin user with those properties.
///
/// This function also runs all embedded migrations when called.
pub fn attach_database(r: rocket::Rocket) -> rocket::Rocket {
    dotenv().ok();

    // Attach database fairing
    let r = r.attach(DbConn::fairing());

    // Get database connection
    let conn = DbConn::get_one(&r).expect("database connection");
    // Run migrations
    embedded_migrations::run(&*conn);

    // Try to create default admin
    let pw = env::var("DEFAULT_ADMIN_PASSWORD");
    let mail = env::var("DEFAULT_ADMIN_EMAIL");
    if pw.is_ok() && mail.is_ok() {
        let admin = models::NewUser {
            email: &mail.unwrap(),
            password_hash: &argon2::hash_encoded(pw.unwrap().as_ref(), auth::generate_salt(15).as_ref(), &argon2::Config::default()).unwrap(),
            first_name: "",
            last_name: "",
            street: "",
            house_number: "",
            zip: "",
            city: "",
            phone: "",
            is_admin: true,
        };

        create_user(&admin, &*conn);
    }

    r
}
