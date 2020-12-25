use diesel::{Queryable, Insertable};

#[derive(Queryable)]
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