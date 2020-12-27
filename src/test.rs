use super::rocket;
use rocket::local::Client;
use rocket::http::{Status, ContentType};
use super::models::*;
use super::database::*;
use diesel::PgConnection;
use argon2::{self, Config};

/// Test the login handler.
///
/// # Arguments
///
/// * `user` - The username/ email to submit.
/// * `password` - The password to submit.
/// * `status` - The expected response status after the form has been submitted.
/// * `body` - Part of the expected response (has to implement the Into and Send trait).
///
fn test_login<T>(user: &str, password: &str, status: Status, body: T)
    where T: Into<Option<&'static str>> + Send
{
    let client = Client::new(test_rocket()).unwrap();
    let query = format!("email={}&password={}", user, password);
    let mut response = client.post("/login")
        .header(ContentType::Form)
        .body(&query)
        .dispatch();

    assert_eq!(response.status(), status);
    if let Some(expected_str) = body.into() {
        if let Some(body_str) = response.body_string() {
            assert!(body_str.contains(expected_str));
        }
    }
}

/// Test login handler using abnormal or incomplete forms.
///
/// # Arguments
///
/// * `form_str` - The form string to test.
/// * `status` - The expected status message.
///
fn test_bad_form(form_str: &str, status: Status) {
    let client = Client::new(test_rocket()).unwrap();
    let response = client.post("/login")
        .header(ContentType::Form)
        .body(form_str)
        .dispatch();

    assert_eq!(response.status(), status);
}

/// Create a rocket instance for test purposes
fn test_rocket() -> rocket::Rocket {
    rocket()
        .attach(super::DbConn::fairing())
}

/// Setup the attached database.
///
/// Requires a database to be attached.
fn setup_test_db(conn: &PgConnection) {
    let u1 = NewUser {
        email: "david@gmail.com",
        password_hash: &argon2::hash_encoded(b"david", b"randomsalt", &Config::default()).unwrap(),
        first_name: "David",
        last_name: "Sugar",
        street: "Test Street",
        house_number: "7a",
        zip: "12345",
        city: "IDK",
        phone: "+49 12345",
        is_admin: true,
    };

    let u2 = NewUser {
        email: "pierre@web.com",
        password_hash: &argon2::hash_encoded(b"pierre", b"randomsalt", &Config::default()).unwrap(),
        first_name: "Pierre",
        last_name: "Sugar",
        street: "Test Street",
        house_number: "31-A2",
        zip: "54321",
        city: "IDK",
        phone: "+49 54321",
        is_admin: false,
    };

    super::embedded_migrations::run(conn); // generated by the embed_migrations! macro
    delete_user_by_mail("david@gmail.com", conn);
    delete_user_by_mail("pierre@web.com", conn);
    create_user(&u1, conn);
    create_user(&u2, conn);
}



#[test]
fn login_test() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let mut response = client.get("/login").dispatch();
    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn test_successful_login() {
    test_login("david@gamil.com", "password", Status::SeeOther, None); // Redirect to dashboard on success
}

#[test]
fn test_bad_form_wrong_number_of_fields() {
    test_bad_form("email=david@web.de", Status::UnprocessableEntity);
    test_bad_form("password=12345", Status::UnprocessableEntity);
    test_bad_form("email=david@web.de&password=12345&cool=true", Status::UnprocessableEntity);
}

#[test]
fn test_bad_form_abnormal() {
    test_bad_form("&&&===&", Status::BadRequest);
    test_bad_form("&&&=password==&", Status::BadRequest);
}

/* #######################################################
################## DATABASE TESTS ########################
##########################################################
 */

#[test]
fn test_get_user() {
    let r = test_rocket();
    let conn = super::DbConn::get_one(&r).expect("database connection"); // get database connection connected to the rocket instance
    setup_test_db(&*conn);

    let user = get_user_by_mail("david@gmail.com", &*conn);
    assert!(user.is_ok());
    assert_eq!("David", user.unwrap().first_name);

    let user2 = get_user_by_mail("franzi@gmail.com", &*conn);
    assert!(user2.is_err());
    assert_eq!(diesel::result::Error::NotFound, user2.err().unwrap());
}

#[test]
fn test_update_user() {
    let r = test_rocket();
    let conn = super::DbConn::get_one(&r).expect("database connection"); // get database connection connected to the rocket instance
    setup_test_db(&*conn);

    // Get user
    let pierre = get_user_by_mail("pierre@web.com", &*conn);
    assert!(pierre.is_ok());
    let mut pierre = pierre.unwrap();
    assert_eq!(false, pierre.is_admin);

    // Update user
    pierre.is_admin = true;
    let result = update_user(&pierre, &*conn);
    assert!(result.is_ok());
    assert!(result.unwrap().is_admin);
}

#[test]
fn test_delete_user() {
    let r = test_rocket();
    let conn = super::DbConn::get_one(&r).expect("database connection"); // get database connection connected to the rocket instance
    setup_test_db(&*conn);

    // Get user
    let user = get_user_by_mail("pierre@web.com", &*conn);
    assert!(user.is_ok());
    let user = user.unwrap();

    // Delete user based on id
    let result = delete_user(user.id, &*conn);
    assert!(result.is_ok());

    // User should be gone
    let user = get_user(user.id, &*conn);
    assert!(user.is_err());
}

#[test]
fn test_salt_generator() {
    let s1 = super::auth::generate_salt(15);
    let s2 = super::auth::generate_salt(15);

    assert_ne!(s1, s2);
}
