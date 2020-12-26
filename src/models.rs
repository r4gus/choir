use diesel::{Queryable, Insertable, Identifiable, AsChangeset};
use super::schema::users; // Required for the table_name

#[derive(Queryable, Identifiable, AsChangeset)]
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
}