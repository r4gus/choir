use diesel::{PgConnection, QueryResult, prelude::*};
use super::models::*;
use super::schema::users::dsl::*;

pub fn get_user(uid: i32, connection: &PgConnection) -> Result<User, diesel::result::Error> {
    users.filter(id.eq(uid)).get_result(connection)
}

pub fn get_user_by_mail(mail: &str, connection: &PgConnection) -> Result<Vec<User>, diesel::result::Error> {
    users.filter(email.eq(mail)).load::<User>(connection)
}

pub fn create_user(u: &NewUser, connection: &PgConnection) -> Result<User, diesel::result::Error> {
    diesel::insert_into(users).values(u).get_result(connection)
}