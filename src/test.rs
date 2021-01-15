use super::rocket;
use rocket::local::Client;
use rocket::http::{Status, ContentType};
use super::models::*;
use super::database::*;
use diesel::PgConnection;
use argon2::{self, Config};
use crate::DbConn;
use super::application::{UpdateMemberForm, UpdateMemberAdvancedForm, UpdateMemberPasswordForm};
use crate::application::{NewMemberForm, NewGroupForm};

/// Test the login handler.
///
/// # Arguments
///
/// * `user` - The username/ email to submit.
/// * `password` - The password to submit.
/// * `status` - The expected response status after the form has been submitted.
/// * `expected_redirect` - Expected redirection path.
///
fn test_login(client: &Client, user: &str, password: &str, status: Status, expected_redirect: Option<&str>)
{
    let query = format!("email={}&password={}", user, password);
    let mut response = client.post("/login")
        .header(ContentType::Form)
        .body(&query)
        .dispatch();

    assert_eq!(response.status(), status);
    if let Some(expected_str) = expected_redirect {
        assert!(response.headers().contains("Location")); // Check if a header field with that name exists
        assert_eq!(expected_str, response.headers().get("Location").next().unwrap()); // Check if the Location is set to the expected redirect location

    };
}

fn test_view_member(client: &Client, user: &str, password: &str, user_to_view: i32, status: Status, expected_redirect: Option<&str>)
{
    test_login(client, user, password, Status::SeeOther, Some("/dashboard")); // Successful login is a prerequisite

    let mut response = client.get(format!("/member/{}", user_to_view)).dispatch();
    assert_eq!(response.status(), status);
    if let Some(expected_str) = expected_redirect {
        assert!(response.headers().contains("Location")); // Check if a header field with that name exists
        assert_eq!(expected_str, response.headers().get("Location").next().unwrap()); // Check if the Location is set to the expected redirect location

    };
}

fn test_update_member(client: &Client, user: &str, password: &str, user_to_update: i32, form:  UpdateMemberForm, status: Status, expected_redirect: Option<&str>)
{
    test_login(client, user, password, Status::SeeOther, Some("/dashboard")); // Successful login is a prerequisite

    let query = format!("email={}&first_name={}&last_name={}&street={}&city={}&zip={}&phone={}",
                                form.email, form.first_name, form.last_name, form.street,
                                form.city, form.zip, form.phone);
    let mut response = client.post(format!("/member/{}/update", user_to_update))
        .header(ContentType::Form)
        .body(&query)
        .dispatch();

    assert_eq!(response.status(), status);
    if let Some(expected_str) = expected_redirect {
        assert!(response.headers().contains("Location")); // Check if a header field with that name exists
        assert_eq!(expected_str, response.headers().get("Location").next().unwrap()); // Check if the Location is set to the expected redirect location
    };
}

fn test_update_member_advanced(client: &Client, user: &str, password: &str, user_to_update: i32, form:  UpdateMemberAdvancedForm, status: Status, expected_redirect: Option<&str>)
{
    test_login(client, user, password, Status::SeeOther, Some("/dashboard")); // Successful login is a prerequisite

    let query = format!("is_admin={}&verified={}", form.is_admin, form.verified);
    let mut response = client.post(format!("/member/{}/advanced", user_to_update))
        .header(ContentType::Form)
        .body(&query)
        .dispatch();

    assert_eq!(response.status(), status);
    if let Some(expected_str) = expected_redirect {
        assert!(response.headers().contains("Location")); // Check if a header field with that name exists
        assert_eq!(expected_str, response.headers().get("Location").next().unwrap()); // Check if the Location is set to the expected redirect location
    };
}

fn test_update_member_password(client: &Client, user: &str, password: &str, user_to_update: i32, form:  UpdateMemberPasswordForm, status: Status, expected_redirect: Option<&str>)
{
    test_login(client, user, password, Status::SeeOther, Some("/dashboard")); // Successful login is a prerequisite

    let query = format!("new_password={}&new_password_again={}", form.new_password, form.new_password_again);
    let mut response = client.post(format!("/member/{}/password", user_to_update))
        .header(ContentType::Form)
        .body(&query)
        .dispatch();

    assert_eq!(response.status(), status);
    if let Some(expected_str) = expected_redirect {
        assert!(response.headers().contains("Location")); // Check if a header field with that name exists
        assert_eq!(expected_str, response.headers().get("Location").next().unwrap()); // Check if the Location is set to the expected redirect location
    };
}

fn test_delete_member(client: &Client, user: &str, password: &str, id: i32, status: Status, expected_redirect: Option<&str>) {
    test_login(client, user, password, Status::SeeOther, Some("/dashboard")); // Successful login is a prerequisite

    let mut response = client.post(format!("/member/{}/delete", id))
        .header(ContentType::Form)
        .dispatch();

    assert_eq!(response.status(), status);
    if let Some(expected_str) = expected_redirect {
        assert!(response.headers().contains("Location")); // Check if a header field with that name exists
        assert_eq!(expected_str, response.headers().get("Location").next().unwrap()); // Check if the Location is set to the expected redirect location
    };
}

fn test_create_member(client: &Client, conn: &DbConn, user: &str, password: &str, form: NewMemberForm, should_pass: bool) {
    test_login(client, user, password, Status::SeeOther, Some("/dashboard")); // Successful login is a prerequisite

    let query = format!("email={}&email_again={}&password={}", form.email, form.email_again, form.password);
    let mut response = client.post("/member/create")
        .header(ContentType::Form)
        .body(&query)
        .dispatch();

    assert_eq!(should_pass, get_user_by_mail(form.email.as_ref(), &**conn).is_ok());

    let s: String;
    if should_pass {
        s = format!("/member/{}", get_user_by_mail(form.email.as_ref(), &**conn).unwrap().id);
    } else {
        s = "/members".to_string();
    }

    assert_eq!(response.status(), Status::SeeOther);
    assert!(response.headers().contains("Location")); // Check if a header field with that name exists
    assert_eq!(s, response.headers().get("Location").next().unwrap()); // Check if the Location is set to the expected redirect location
}

fn test_view_groups(client: &Client, user: &str, password: &str, status: Status, expected_redirect: Option<&str>)
{
    test_login(client, user, password, Status::SeeOther, Some("/dashboard")); // Successful login is a prerequisite

    let mut response = client.get("/groups").dispatch();
    assert_eq!(response.status(), status);
    if let Some(expected_str) = expected_redirect {
        assert!(response.headers().contains("Location")); // Check if a header field with that name exists
        assert_eq!(expected_str, response.headers().get("Location").next().unwrap()); // Check if the Location is set to the expected redirect location

    };
}

fn test_create_group(client: &Client, conn: &DbConn, user: &str, password: &str, form: NewGroupForm, should_pass: bool) {
    test_login(client, user, password, Status::SeeOther, Some("/dashboard")); // Successful login is a prerequisite

    let query = format!("title={}", form.title);
    let mut response = client.post("/group/create")
        .header(ContentType::Form)
        .body(&query)
        .dispatch();

    assert_eq!(should_pass, get_group_by_title(form.title.as_ref(), &**conn).is_ok());

    if should_pass {
        assert_eq!(response.status(), Status::SeeOther);
        assert!(response.headers().contains("Location")); // Check if a header field with that name exists
        assert_eq!("/groups", response.headers().get("Location").next().unwrap()); // Check if the Location is set to the expected redirect location
    } else {
        assert_eq!(response.status(), Status::NotFound);
    }
}

fn test_delete_group_post(client: &Client, user: &str, password: &str, id: i32, status: Status, expected_redirect: Option<&str>) {
    test_login(client, user, password, Status::SeeOther, Some("/dashboard")); // Successful login is a prerequisite

    let mut response = client.post(format!("/group/{}/delete", id))
        .header(ContentType::Form)
        .dispatch();

    assert_eq!(response.status(), status);
    if let Some(expected_str) = expected_redirect {
        assert!(response.headers().contains("Location")); // Check if a header field with that name exists
        assert_eq!(expected_str, response.headers().get("Location").next().unwrap()); // Check if the Location is set to the expected redirect location
    };
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
        password_hash: &argon2::hash_encoded(b"David", b"randomsalt", &Config::default()).unwrap(),
        first_name: "David",
        last_name: "Sugar",
        street: "Test Street",
        house_number: "7a",
        zip: "12345",
        city: "IDK",
        phone: "+49 12345",
        is_admin: true,
        verified: true,
    };

    let u2 = NewUser {
        email: "pierre@web.com",
        password_hash: &argon2::hash_encoded(b"Pierre", b"randomsalt", &Config::default()).unwrap(),
        first_name: "Pierre",
        last_name: "Sugar",
        street: "Test Street",
        house_number: "31-A2",
        zip: "54321",
        city: "IDK",
        phone: "+49 54321",
        is_admin: false,
        verified: true,
    };

    let u3 = NewUser {
        email: "franzi@web.com",
        password_hash: &argon2::hash_encoded(b"Franzi", b"randomsalt", &Config::default()).unwrap(),
        first_name: "Franzi",
        last_name: "Sugar",
        street: "Test Street",
        house_number: "31-A2",
        zip: "54321",
        city: "IDK",
        phone: "+49 54321",
        is_admin: false,
        verified: true,
    };

    let u4 = NewUser {
        email: "sarah@web.com",
        password_hash: &argon2::hash_encoded(b"Sarah", b"randomsalt", &Config::default()).unwrap(),
        first_name: "Sarah",
        last_name: "Sugar",
        street: "Test Street",
        house_number: "31-A2",
        zip: "54321",
        city: "IDK",
        phone: "+49 54321",
        is_admin: false,
        verified: false,
    };

    let g1 = NewGroup {
        title: "Erste Stimme",
    };

    let g2 = NewGroup {
        title: "Zweite Stimme",
    };

    let g3 = NewGroup {
        title: "Bass",
    };

    super::embedded_migrations::run(conn); // generated by the embed_migrations! macro
    delete_all_users(conn);
    delete_all_groups(conn);
    create_user(&u1, conn);
    create_user(&u2, conn);
    create_user(&u3, conn);
    create_user(&u4, conn);
    create_group(&g1, conn);
    create_group(&g2, conn);
    create_group(&g3, conn);
}

fn make_connection_and_client() -> (DbConn, Client) {
    let r = test_rocket();
    let conn = super::DbConn::get_one(&r).expect("database connection"); // get database connection connected to the rocket instance
    setup_test_db(&*conn);
    let client = Client::new(r).unwrap();

    (conn, client)
}

/* #######################################################
################## LOGIN TESTS ########################
##########################################################
 */

#[test]
fn login_test() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let mut response = client.get("/login").dispatch();
    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn test_successful_login() {
    let (_, client) = make_connection_and_client();

    test_login(&client,"pierre@web.com", "Pierre", Status::SeeOther, Some("/dashboard"));
    test_login(&client,"franzi@web.com", "Franzi", Status::SeeOther, Some("/dashboard"));
    test_login(&client, "david@gmail.com", "David", Status::SeeOther, Some("/dashboard")); // Redirect to dashboard on success
}

#[test]
fn test_only_validated_users_can_login() {
    let (_, client) = make_connection_and_client();

    test_login(&client,"sarah@web.com", "Sarah", Status::SeeOther, Some("/login"));
}

#[test]
fn test_unsuccessful_login() {
    let (_, client) = make_connection_and_client();

    test_login(&client,"david@gmail.com", "Davd", Status::SeeOther, Some("/login")); // Redirect to dashboard on success
    test_login(&client,"pierre@web.com", "Pierrre", Status::SeeOther, Some("/login"));
    test_login(&client,"franzi@web.com", "Franziska", Status::SeeOther, Some("/login"));
    test_login(&client,"david@mail.com", "David", Status::SeeOther, Some("/login")); // Redirect to dashboard on success
    test_login(&client,"pierre@web.de", "Pierre", Status::SeeOther, Some("/login"));
    test_login(&client,"franziska@web.com", "Franzi", Status::SeeOther, Some("/login"));
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
    let (conn, _) = make_connection_and_client();

    let user = get_user_by_mail("david@gmail.com", &*conn);
    assert!(user.is_ok());
    assert_eq!("David", user.unwrap().first_name);

    let user2 = get_user_by_mail("franzi@gmail.com", &*conn);
    assert!(user2.is_err());
    assert_eq!(diesel::result::Error::NotFound, user2.err().unwrap());
}

#[test]
fn test_update_user() {
    let (conn, _) = make_connection_and_client();

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

#[test]
fn test_create_group_success() {
    let (conn, _) = make_connection_and_client();

    let g = NewGroup {
        title: "Erste Stimme",
    };

    assert!(create_group(&g, &*conn).is_ok());
}

#[test]
fn test_get_group() {
    let (conn, _) = make_connection_and_client();

    assert!(get_group_by_title("Bass", &*conn).is_ok());
    assert_eq!("Erste Stimme", get_group_by_title("Erste Stimme",&*conn).unwrap().title);
    assert!(get_group_by_title("Some non existing title", &*conn).is_err());
}


#[test]
fn test_update_group() {
    let (conn, _) = make_connection_and_client();

    let mut g = get_group_by_title("Bass", &*conn).unwrap();
    let i = g.id;

    g.title = "BassMitX".to_string();
    assert!(update_group(&g, &*conn).is_ok());
    assert_eq!("BassMitX", get_group(i, &*conn).unwrap().title);
}

#[test]
fn test_delete_group() {
    let (conn, _) = make_connection_and_client();

    let mut g = get_group_by_title("Bass", &*conn).unwrap();
    assert!(delete_group(g.id, &*conn).is_ok());
    assert!(get_group(g.id, &*conn).is_err());
}


/* #######################################################
################## MEMBER TESTS ########################
##########################################################
 */



#[test]
fn test_view_own_member_page() {
    let (conn, client) = make_connection_and_client();

    if let Ok(users) = get_users(&*conn) {
        for user in users {
            if user.verified {
                test_view_member(&client,user.email.as_ref(), user.first_name.as_ref(), user.id, Status::Ok, None); // everyone can view his/her own member page
            }
        }
    } else {
        assert!(false);
    }
}

#[test]
fn test_view_member_page_by_admin() {
    let (conn, client) = make_connection_and_client();

    if let Ok(users) = get_users(&*conn) {
        for user in users {
            test_view_member(&client,"david@gmail.com", "David", user.id, Status::Ok, None); // Admin can view all member pages
        }
    } else {
        assert!(false);
    }
}

#[test]
fn test_unallowed_member_page_access_by_member() {
    let (conn, client) = make_connection_and_client();

    let member_name = "franzi@web.com";
    let member_pw = "Franzi";

    if let Ok(users) = get_users(&*conn) {
        for user in users {
            if user.email == member_name {
                test_view_member(&client,member_name, member_pw, user.id, Status::Ok, None);
            } else {
                test_view_member(&client,member_name, member_pw, user.id, Status::SeeOther, Some("/dashboard"));
            }
        }
    } else {
        assert!(false);
    }
}

#[test]
fn test_members_can_update_their_own_profile() {
    let (conn, client) = make_connection_and_client();

    if let Ok(users) = get_users(&*conn) {
        for user in users {
            let form = UpdateMemberForm {
                email: format!("newmail{}.de", user.id),
                first_name: user.first_name.clone(),
                last_name: user.last_name.clone(),
                street: "New+street+for+user".to_string(),
                city: user.city.clone(),
                zip: user.zip.clone(),
                phone: "666".to_string(),
            };

            if user.verified {
                test_update_member(&client, user.email.as_ref(), user.first_name.as_ref(), user.id, form, Status::SeeOther, Some(&format!("/member/{}", user.id)));

                if let Ok(u) = get_user(user.id, &*conn) {
                    assert_ne!(user, u); // user hasn't changed
                }
            }

        }
    } else {
        assert!(false);
    }
}

#[test]
fn test_a_non_admin_member_cant_update_other_profiles() {
    let (conn, client) = make_connection_and_client();

    let member_name = "franzi@web.com";
    let member_pw = "Franzi";

    if let Ok(users) = get_users(&*conn) {
        for user in users {
            let form = UpdateMemberForm {
                email: format!("newmail{}.de", user.id),
                first_name: user.first_name.clone(),
                last_name: user.last_name.clone(),
                street: "New+street+for+user".to_string(),
                city: user.city.clone(),
                zip: user.zip.clone(),
                phone: "666".to_string(),
            };

            if user.email != member_name {
                test_update_member(&client, member_name, member_pw, user.id, form, Status::SeeOther, Some("/dashboard"));

                if let Ok(u) = get_user(user.id, &*conn) {
                    assert_eq!(user, u); // user hasn't changed
                }
            }
        }
    } else {
        assert!(false);
    }
}

#[test]
fn test_a_admin_user_can_update_all_profiles() {
    let (conn, client) = make_connection_and_client();

    let member_name = "david@gmail.com";
    let member_pw = "David";

    if let Ok(users) = get_users(&*conn) {
        for user in users {
            let form = UpdateMemberForm {
                email: format!("newmail{}.de", user.id),
                first_name: user.first_name.clone(),
                last_name: user.last_name.clone(),
                street: "New+street+for+user".to_string(),
                city: user.city.clone(),
                zip: user.zip.clone(),
                phone: "666".to_string(),
            };

            if user.email != member_name {
                test_update_member(&client, member_name, member_pw, user.id, form, Status::SeeOther, Some(&format!("/member/{}", user.id)));

                if let Ok(u) = get_user(user.id, &*conn) {
                    assert_ne!(user, u); // Admin was able to change user data
                }
            }
        }
    } else {
        assert!(false);
    }
}

#[test]
fn test_a_admin_user_can_update_advanced_settings_of_a_member()  {
    let (conn, client) = make_connection_and_client();

    let admin_name = "david@gmail.com";
    let admin_pw = "David";

    let target_mail = "franzi@web.com";

    if let Ok(user) = get_user_by_mail(target_mail, &*conn) {
        let form = UpdateMemberAdvancedForm {
            is_admin: true,
            verified: true,
        };

        test_update_member_advanced(&client, admin_name, admin_pw, user.id, form, Status::SeeOther, Some(&format!("/member/{}", user.id)));
    } else {
        assert!(false);
    }
}

#[test]
fn test_a_user_can_not_update_advanced_settings_of_a_member()  {
    let (conn, client) = make_connection_and_client();

    let admin_name = "franzi@web.com";
    let admin_pw = "Franzi";

    let target_mail = "franzi@web.com";

    if let Ok(user) = get_user_by_mail(target_mail, &*conn) {
        let form = UpdateMemberAdvancedForm {
            is_admin: true,
            verified: true,
        };

        test_update_member_advanced(&client, admin_name, admin_pw, user.id, form, Status::NotFound, None);
    } else {
        assert!(false);
    }
}

#[test]
fn test_a_user_can_update_the_own_password() {
    let (conn, client) = make_connection_and_client();

    let new_pw = "NewPw";

    if let Ok(users) = get_users(&*conn) {
        for user in users {
            let form = UpdateMemberPasswordForm {
                new_password: new_pw.to_string(),
                new_password_again: new_pw.to_string(),
            };

            if user.verified {
                test_update_member_password(&client, user.email.as_ref(), user.first_name.as_ref(), user.id, form, Status::SeeOther, Some(&format!("/member/{}", user.id)));
                test_login(&client,user.email.as_ref(), new_pw, Status::SeeOther, Some("/dashboard"));
            }

        }
    } else {
        assert!(false);
    }
}

#[test]
fn test_a_admin_can_update_all_other_passwords() {
    let (conn, client) = make_connection_and_client();

    let new_pw = "NewPw";

    if let Ok(users) = get_users(&*conn) {
        for user in users {
            let form = UpdateMemberPasswordForm {
                new_password: new_pw.to_string(),
                new_password_again: new_pw.to_string(),
            };

            if user.verified && user.email != "david@gmail.com" {
                test_update_member_password(&client, "david@gmail.com", "David", user.id, form, Status::SeeOther, Some(&format!("/member/{}", user.id)));
                test_login(&client,user.email.as_ref(), new_pw, Status::SeeOther, Some("/dashboard"));
            }
        }
    } else {
        assert!(false);
    }
}

#[test]
fn test_a_user_can_delete_himself() {
    let (conn, client) = make_connection_and_client();

    if let Ok(users) = get_users(&*conn) {
        for user in users {

            if user.verified {
                test_delete_member(&client, user.email.as_ref(), user.first_name.as_ref(), user.id, Status::SeeOther, Some("/logout"));
            }
        }
    } else {
        assert!(false);
    }
}

#[test]
fn test_a_admin_can_delete_all_users() {
    let (conn, client) = make_connection_and_client();

    if let Ok(users) = get_users(&*conn) {
        for user in users {

            if user.verified && user.email != "david@gmail.com" {
                test_delete_member(&client, "david@gmail.com", "David", user.id, Status::SeeOther, Some("/members"));
                assert!(get_user(user.id, &*conn).is_err());
            }
        }
    } else {
        assert!(false);
    }
}

#[test]
fn test_create_a_new_member() {
    let (conn, client) = make_connection_and_client();

    let form = NewMemberForm {
        email: "new@member.de".to_string(),
        email_again: "new@member.de".to_string(),
        password: "lol".to_string(),
    };

    test_create_member(&client, &conn, "david@gmail.com", "David", form, true);
}

#[test]
fn test_create_a_new_member_without_matching_email_addresses() {
    let (conn, client) = make_connection_and_client();

    let form = NewMemberForm {
        email: "new@member.de".to_string(),
        email_again: "ne@member.de".to_string(),
        password: "lol".to_string(),
    };

    test_create_member(&client, &conn, "david@gmail.com", "David", form, false);
}

#[test]
fn test_admins_can_view_all_groups() {
    let (conn, client) = make_connection_and_client();

    test_view_groups(&client, "david@gmail.com", "David", Status::Ok, None);
}

#[test]
fn test_normal_users_cant_view_groups() {
    let (conn, client) = make_connection_and_client();

    test_view_groups(&client, "franzi@web.com", "Franzi", Status::SeeOther, Some("/dashboard"));
}

#[test]
fn test_admins_can_create_new_groups() {
    let (conn, client) = make_connection_and_client();

    let g = NewGroupForm {
        title: "New Group".to_string(),
    };

    test_create_group(&client, &conn, "david@gmail.com", "David", g, true);
}

#[test]
fn test_users_cant_create_new_groups() {
    let (conn, client) = make_connection_and_client();

    let g = NewGroupForm {
        title: "New Group".to_string(),
    };

    test_create_group(&client, &conn, "franzi@web.com", "Franzi", g, false);
}

#[test]
fn test_admins_can_delete_groups() {
    let (conn, client) = make_connection_and_client();

    let g = get_group_by_title("Bass", &*conn).unwrap();

    test_delete_group_post(&client, "david@gmail.com", "David", g.id, Status::SeeOther, Some("/groups"));
}

#[test]
fn test_users_cant_delete_groups() {
    let (conn, client) = make_connection_and_client();

    let g = get_group_by_title("Bass", &*conn).unwrap();

    test_delete_group_post(&client, "franzi@web.com", "Franzi", g.id, Status::NotFound, None);
}