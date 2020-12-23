use super::rocket;
use rocket::local::Client;
use rocket::http::{Status, ContentType};

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
    let client = Client::new(rocket()).unwrap();
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

fn test_bad_form(form_str: &str, status: Status) {
    let client = Client::new(rocket()).unwrap();
    let response = client.post("/login")
        .header(ContentType::Form)
        .body(form_str)
        .dispatch();

    assert_eq!(response.status(), status);
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
