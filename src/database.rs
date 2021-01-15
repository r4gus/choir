use diesel::{PgConnection, QueryResult, prelude::*};
use super::models::*;
use super::schema::users::dsl::*;
use super::schema::groups::dsl::*;
use rocket::http::uri::Query;

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

pub fn delete_all_users(connection: &PgConnection) -> QueryResult<usize> {
    diesel::delete(users).execute(connection)
}

// ##########################################################################
//                            Groups
// ###########################################################################

pub fn get_groups(connection: &PgConnection) -> Result<Vec<Group>, diesel::result::Error> {
    groups.load(connection)
}

pub fn create_group(g: &NewGroup, connection: &PgConnection) -> Result<Group, diesel::result::Error> {
    diesel::insert_into(groups).values(g).get_result(connection)
}

pub fn get_group_by_title(t: &str, connection: &PgConnection) -> Result<Group, diesel::result::Error> {
    groups.filter(title.eq(t)).get_result(connection)
}

pub fn get_group(i: i32, conn: &PgConnection) -> Result<Group, diesel::result::Error> {
    groups.filter(gid.eq(i)).get_result(conn)
}

pub fn update_group(g: &Group, conn: &PgConnection) -> Result<Group, diesel::result::Error> {
    g.save_changes(conn)
}

pub fn delete_group(i: i32, connection: &PgConnection) -> QueryResult<usize> {
    diesel::delete(groups.filter(gid.eq(i))).execute(connection)
}

pub fn delete_all_groups(connection: &PgConnection) -> QueryResult<usize> {
    diesel::delete(groups).execute(connection)
}