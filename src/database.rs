use diesel::{PgConnection, QueryResult, prelude::*};
use super::models::*;
use super::schema::users::dsl::*;

pub fn get_users(connection: &PgConnection) -> Result<Vec<User>, diesel::result::Error> {
    users.load(connection)
}

/// Get a user from the database based on its ID.
///
/// # Arguments
///
/// * `uid` - Unique ID of the user.
/// * `connection` - Reference to a postgres connection
pub fn get_user(uid: i32, connection: &PgConnection) -> Result<User, diesel::result::Error> {
    users.filter(id.eq(uid)).get_result(connection)
}

/// Get a user from the database based on its e-mail.
///
/// # Arguments
///
/// * `mail` - Unique e-mal address of the user.
/// * `connection` - Reference to a postgres connection
pub fn get_user_by_mail(mail: &str, connection: &PgConnection) -> Result<User, diesel::result::Error> {
    users.filter(email.eq(mail)).get_result(connection)
}

/// Insert a new user into the database.
///
/// # Arguments
///
/// * `u` - Reference to a NewUser structure.
/// * `connection` - Reference to a postgres connection
pub fn create_user(u: &NewUser, connection: &PgConnection) -> Result<User, diesel::result::Error> {
    diesel::insert_into(users).values(u).get_result(connection)
}

pub fn update_user(u: &User, connection: &PgConnection) -> Result<User, diesel::result::Error> {
    u.save_changes(connection)
}

pub fn delete_user(uid: i32, connection: &PgConnection) -> QueryResult<usize> {
    diesel::delete(users.filter(id.eq(uid))).execute(connection)
}

pub fn delete_user_by_mail(mail: &str, connection: &PgConnection) -> QueryResult<usize> {
    diesel::delete(users.filter(email.eq(mail))).execute(connection)
}