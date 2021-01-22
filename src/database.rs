use diesel::{PgConnection, QueryResult, prelude::*};
use super::models::*;
use super::schema::users::dsl::*;
use super::schema::groups::dsl::*;
use super::schema::belongs::dsl::{belongs, gid as bgid, uid as buid};
use super::schema::appointments::dsl::{appointments, id as aid, title as atitle};
use rocket::http::uri::Query;
use std::collections::HashMap;
use crate::schema::appointments::columns::begins;
use chrono::prelude::*;


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

pub fn add_user_to_group(group_id: i32, user_id: i32, conn: &PgConnection) -> Result<Belong, diesel::result::Error> {
    diesel::insert_into(belongs).values(&Belong {
        gid: group_id,
        uid: user_id,
    }).get_result(conn)
}

pub fn delete_user_from_group(group_id: i32, user_id: i32, conn: &PgConnection) -> QueryResult<usize> {
    diesel::delete(
        belongs
            .filter(bgid.eq(group_id))
            .filter(buid.eq(user_id))
    ).execute(conn)
}

pub fn get_user_for_group(group_id: i32, conn: &PgConnection) -> Result<Vec<User>, diesel::result::Error> {
    match belongs.filter(bgid.eq(group_id)).inner_join(users).load(conn) {
        Ok(v) => {
            Ok(v.iter().map(|(b, u): &(Belong, User)| u.clone()).collect::<Vec<User>>())
        },
        Err(err) => Err(err)
    }
}

// ##########################################################################
//                            Appointments
// ###########################################################################

pub fn create_appointment(a: &NewAppointment, connection: &PgConnection) -> Result<Appointment, diesel::result::Error> {
    diesel::insert_into(appointments).values(a).get_result(connection)
}

pub fn get_appointments(connection: &PgConnection) -> Result<Vec<Appointment>, diesel::result::Error> {
    appointments.load(connection)
}

pub fn get_appointment_by_title(t: &str, connection: &PgConnection) -> Result<Appointment, diesel::result::Error> {
    appointments.filter(atitle.eq(t)).get_result(connection)
}

pub fn get_appointment(i: i32, conn: &PgConnection) -> Result<Appointment, diesel::result::Error> {
    appointments.filter(aid.eq(i)).get_result(conn)
}

pub fn delete_appointment(i: i32, connection: &PgConnection) -> QueryResult<usize> {
    diesel::delete(appointments.filter(aid.eq(i))).execute(connection)
}

pub fn delete_all_appointments(connection: &PgConnection) -> QueryResult<usize> {
    diesel::delete(appointments).execute(connection)
}

pub fn update_appointment(a: &Appointment, conn: &PgConnection) -> Result<Appointment, diesel::result::Error> {
    a.save_changes(conn)
}

pub fn get_future_appointments(connection: &PgConnection) -> Result<Vec<Appointment>, diesel::result::Error> {
    appointments.filter(begins.ge(NaiveDateTime::from_timestamp(Utc::now().timestamp(), 0))).load(connection)
}