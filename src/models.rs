use diesel::{Queryable, Insertable, Identifiable, AsChangeset};
use super::schema::{users, groups, belongs};
use rocket::request::{self, FromRequest};
use rocket::{Request, Outcome};
use crate::DbConn;
use crate::database::get_user;
use rocket::outcome::IntoOutcome; // Required for the table_name

#[derive(Queryable, Identifiable, AsChangeset, serde::Serialize, PartialEq, Debug, Clone)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password_hash: String,
    pub first_name: String,
    pub last_name: String,
    pub street: String,
    pub house_number: String,
    pub zip: String,
    pub city: String,
    pub phone: String,
    pub is_admin: bool,
    pub verified: bool,
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub password_hash: &'a str,
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub street: &'a str,
    pub house_number: &'a str,
    pub zip: &'a str,
    pub city: &'a str,
    pub phone: &'a str,
    pub is_admin: bool,
    pub verified: bool,
}

pub struct AdminUser<'a>(pub &'a User);

impl<'a, 'r> FromRequest<'a, 'r> for &'a User {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<&'a User, Self::Error> {
        let user_result = request.local_cache(|| {
            let db = request.guard::<DbConn>().succeeded()?;
            request.cookies()
                .get_private("user_id")
                .and_then(|cookie| cookie.value().parse().ok())
                .and_then(|id| get_user(id, &*db).ok())
        });

        user_result.as_ref().or_forward(())
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for AdminUser<'a> {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<AdminUser<'a>, Self::Error> {
        let user = request.guard::<&User>()?; // Leverage the FromRequest implementation of User.

        if user.is_admin {
            Outcome::Success(AdminUser(user))
        } else {
            Outcome::Forward(())
        }
    }
}

#[derive(Queryable, Identifiable, AsChangeset, serde::Serialize, PartialEq, Debug)]
pub struct Group {
    pub id: i32,
    pub title: String,
}

#[derive(Insertable)]
#[table_name="groups"]
pub struct NewGroup<'a> {
    pub title: &'a str,
}

#[derive(Queryable, AsChangeset, serde::Serialize, PartialEq, Debug, Insertable)]
pub struct Belong {
    pub gid: i32,
    pub uid: i32,
}