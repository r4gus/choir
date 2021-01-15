use super::models::{User, AdminUser, NewUser};
use rocket::response::{Flash, Redirect};
use rocket_contrib::templates::Template;
use rocket::request::{FlashMessage, Form};
use crate::DbConn;
use crate::database::{get_users, get_user, update_user, delete_user, create_user, get_user_by_mail, get_groups, create_group, delete_group, get_user_for_group};
use rocket::http::{Cookies, Cookie};
use diesel::result::Error;
use crate::models::{Group, NewGroup};
use std::collections::HashMap;


#[derive(FromForm)]
pub struct UpdateMemberForm {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub street: String,
    pub city: String,
    pub zip: String,
    pub phone: String,
}

#[derive(FromForm)]
pub struct UpdateMemberAdvancedForm {
    pub is_admin: bool,
    pub verified: bool,
}

#[derive(FromForm)]
pub struct UpdateMemberPasswordForm {
    pub new_password: String,
    pub new_password_again: String,
}

#[derive(FromForm, Clone)]
pub struct NewMemberForm {
    pub email: String,
    pub email_again: String,
    pub password: String,
}

#[derive(FromForm)]
pub struct NewGroupForm {
    pub title: String,
}

#[derive(serde::Serialize)]
pub struct Context<'a, T> {
    pub flash: Option<String>,
    pub flash_type: Option<String>,
    pub user: Option<&'a User>,
    pub collection: Option<T>,
}

impl<T> Context<'_, T> {
    pub fn new() -> Self {
        Context {
            flash: None,
            flash_type: None,
            user: None,
            collection: None
        }
    }

    pub fn parse_falsh_message(&mut self, msg: &FlashMessage) {
        self.flash = Some(msg.msg().to_string());
        match msg.name() {
            "error" => self.flash_type = Some("alert-danger".to_string()),
            "warning" => self.flash_type = Some("alert-warning".to_string()),
            _ => self.flash_type = Some("alert-success".to_string()),
        };
    }
}

/// View the dashboard.
#[get("/dashboard")]
pub fn dashboard(user: &User, flash: Option<FlashMessage<'_, '_>>) -> Template {
    let mut context = Context::<Vec<User>>::new();

    if let Some(ref msg) = flash {
        context.parse_falsh_message(msg);
    }
    context.user = Some(user);

    Template::render("dashboard", &context)
}

/// Redirect from the dashboard to the login page if user isn't logged in.
#[get("/dashboard", rank = 2)]
pub fn admin_panel_redirect() -> Flash<Redirect> {
    Flash::warning(Redirect::to(uri!(super::auth::login)), "Please login to visit this page")
}

/// Request to the index page directly redirects to the dashboard.
#[get("/")]
pub fn index() -> Redirect {
    Redirect::to("/dashboard")
}

/// Show all registered members.
///
/// This page can only be viewed by an administrator.
#[get("/members")]
pub fn members(user: AdminUser, flash: Option<FlashMessage<'_, '_>>, conn: DbConn) -> Template {
    let mut context = Context::<Vec<User>>::new();

    if let Some(ref msg) = flash {
        context.parse_falsh_message(msg);
    }
    context.user = Some(user.0);

    if let Ok(members) = get_users(&*conn) {
        context.collection = Some(members);
    }

    Template::render("members", &context)
}

#[get("/members", rank = 2)]
pub fn members_redirect() -> Flash<Redirect> {
    Flash::warning(Redirect::to(uri!(dashboard)), "You must be admin to view this page.")
}

/// Show the member status page.
///
/// A page can only be viewed by the associated member or by an admin.
///
/// # Arguments
///
/// * `user` - Reference to a user request guard (this function gets only executed if the guard is met, i.e. the request comes from a loged in user).
/// * `id` - The id of the member to show.
/// * `conn` - The database connection.
/// * `flash` - Potential flash message.
#[get("/member/<id>")]
pub fn member(user: &User, id: i32, conn: DbConn, flash: Option<FlashMessage<'_, '_>>) -> Result<Template, Flash<Redirect>> {
    if user.id != id && !user.is_admin { // You're only allowed to view your own profile, except for admins.
        return Err(Flash::error(Redirect::to(uri!(dashboard)), "You're not allowed to access this page."));
    }

    let mut context = Context::<Vec<User>>::new();

    if let Some(ref msg) = flash {
        context.parse_falsh_message(msg);
    }
    context.user = Some(user);

    if let Ok(member) = get_user(id, &*conn) {
        context.collection = Some(vec![member]);
    }

    Ok(Template::render("member", &context))
}

/// Triggers a redirect if the `member` function didn't match.
#[get("/member/<id>", rank = 2)]
pub fn member_redirect(id: i32) -> Flash<Redirect> {
    Flash::warning(Redirect::to(uri!(super::auth::login)), "Please login")
}

/// Update the member with the specified id.
///
/// # Arguments
///
/// * `user` - Reference to a user request guard (this function gets only executed if the guard is met, i.e. the request comes from a loged in user).
/// * `id` - The id of the member to update.
/// * `conn` - The database connection.
/// * `form` - The passed data as a struct.
#[post("/member/<id>/update", data = "<form>")]
pub fn member_update(user: &User, id: i32, conn: DbConn, form: Form<UpdateMemberForm>) -> Flash<Redirect> {
    if user.id != id && !user.is_admin {
        return Flash::warning(Redirect::to(uri!(dashboard)), "You're not allowed to perform this action.");
    }

    match get_user(id, &*conn) {
        Ok(mut u) => {
            u.email = form.email.clone();
            u.first_name = form.first_name.clone();
            u.last_name = form.last_name.clone();
            u.street = form.street.clone();
            u.city = form.city.clone();
            u.zip = form.zip.clone();
            u.phone = form.phone.clone();

            match update_user(&u, &*conn) {
                Ok(_) => Flash::success(Redirect::to(format!("/member/{}", id)), "Member successfully updated"),
                Err(error) => Flash::error(Redirect::to(format!("/member/{}", id)), format!("Couldn't update member: {}", error.to_string()))
            }
        },
        Err(error) => Flash::error(Redirect::to(format!("/member/{}", id)), format!("Couldn't retrieve member from Database: {}", error.to_string()))
    }
}

/// Update the `verified` and `is_admin` status of a member.
///
/// This can only be done by an administrator.
///
/// # Argumenst
///
/// * `user` - A admin request guard (this function gets only executed if the guard is met, i.e. the request comes from a loged in administrator).
/// * `id` - The id of the member to update.
/// * `conn` - The database connection.
/// * `form` - The passed data as a struct.
#[post("/member/<id>/advanced", data = "<form>")]
pub fn member_update_advanced(user: AdminUser, id: i32, conn: DbConn, form: Form<UpdateMemberAdvancedForm>) -> Flash<Redirect> {
    match get_user(id, &*conn) {
        Ok(mut u) => {
            u.is_admin = form.is_admin;
            u.verified = form.verified;

            match update_user(&u, &*conn) {
                Ok(_) => Flash::success(Redirect::to(format!("/member/{}", id)), "Member successfully updated"),
                Err(error) => Flash::error(Redirect::to(format!("/member/{}", id)), format!("Couldn't update member: {}", error.to_string()))
            }
        },
        Err(error) => Flash::error(Redirect::to(format!("/member/{}", id)), format!("Couldn't retrieve member from Database: {}", error.to_string()))
    }
}

/// Update the password of the member with the given id.
///
/// A member password can either be updated by him/ her self or by an administrator.
///
/// # Argumenst
///
/// * `user` - Reference to a user request guard (this function gets only executed if the guard is met, i.e. the request comes from a loged in user).
/// * `id` - The id of the member to update.
/// * `conn` - The database connection.
/// * `form` - The passed data as a struct.
#[post("/member/<id>/password", data = "<form>")]
pub fn member_update_password(user: &User, id: i32, conn: DbConn, form: Form<UpdateMemberPasswordForm>) -> Flash<Redirect> {
    if user.id != id && !user.is_admin {
        return Flash::warning(Redirect::to(uri!(dashboard)), "You're not allowed to perform this action.");
    }

    match get_user(id, &*conn) {
        Ok(mut u) => {
            if form.new_password == form.new_password_again {
                u.password_hash = argon2::hash_encoded(form.new_password.as_ref(), super::auth::generate_salt(15).as_ref(), &argon2::Config::default()).unwrap();

                match update_user(&u, &*conn) {
                    Ok(_) => Flash::success(Redirect::to(format!("/member/{}", id)), "Password successfully updated"),
                    Err(error) => Flash::error(Redirect::to(format!("/member/{}", id)), format!("Couldn't update member: {}", error.to_string()))
                }
            } else {
                return Flash::warning(Redirect::to(format!("/member/{}", id)), "Passwords don't match"); // Invalid password
            }
        },
        Err(error) => Flash::error(Redirect::to(format!("/member/{}", id)), format!("Couldn't retrieve member from Database: {}", error.to_string()))
    }
}

/// Delete a member from the attached database.
///
/// A member can either be deleted by him/ her self or by an administrator.
///
/// # Argumenst
///
/// * `user` - Reference to a user request guard (this function gets only executed if the guard is met, i.e. the request comes from a loged in user).
/// * `id` - The id of the member to delete.
/// * `conn` - The database connection.
#[post("/member/<id>/delete")]
pub fn member_delete(user: &User, id: i32, conn: DbConn) -> Flash<Redirect> {
    if user.id != id && !user.is_admin {
        return Flash::warning(Redirect::to(uri!(dashboard)), "You're not allowed to perform this action.");
    }

    match delete_user(id, &*conn) {
        Ok(_) => {
            if user.id == id {
                Flash::success(Redirect::to(uri!(super::auth::logout)), "Account successfully deleted")
            } else {
                Flash::success(Redirect::to(uri!(members)), "Account successfully deleted")
            }
        },
        Err(err) => Flash::error(Redirect::to(format!("/member/{}", id)), format!("Couldn't delete member: {}", err.to_string()))
    }
}

#[post("/member/create", data = "<form>")]
pub fn member_create(user: AdminUser, conn: DbConn, form: Form<NewMemberForm>) -> Flash<Redirect> {
    if form.email != form.email_again {
        return Flash::warning(Redirect::to(uri!(members)), "Emails don't match.");
    }

    let new_user = NewUser {
        email: &form.email,
        password_hash: &argon2::hash_encoded(form.password.as_ref(), super::auth::generate_salt(15).as_ref(), &argon2::Config::default()).unwrap(),
        first_name: "",
        last_name: "",
        street: "",
        house_number: "",
        zip: "",
        city: "",
        phone: "",
        is_admin: false,
        verified: false,
    };

    match create_user(&new_user, &*conn) {
        Ok(user) => Flash::success(Redirect::to(format!("/member/{}", user.id)), "Account successfully created."),
        Err(error) => {
            match error {
                Error::DatabaseError(diesel::result::DatabaseErrorKind::UniqueViolation, _) => {
                    Flash::warning(Redirect::to(uri!(members)), "The email you chose does already exist.")
                }
                _ => {
                    Flash::error(Redirect::to(uri!(members)), format!("Database error: {}", error.to_string()))
                }
            }
        }
    }
}

#[get("/groups")]
pub fn view_groups(user: AdminUser, conn: DbConn, flash: Option<FlashMessage<'_, '_>>) -> Template {
    let mut context = Context::<Vec<(Group, Vec<User>)>>::new();

    if let Some(ref msg) = flash {
        context.parse_falsh_message(msg);
    }
    context.user = Some(user.0);

    if let Ok(groups) = get_groups(&*conn) {
        let mut new_collection: Vec<(Group, Vec<User>)> = Vec::new();
        for group in groups {
            let v = match get_user_for_group(group.id, &*conn) {
                Ok(uvec) => uvec,
                Err(_) => Vec::<User>::new()
            };

            new_collection.push((group, v));
        }

        context.collection = Some(new_collection);
    }

    Template::render("groups", &context)
}

#[get("/groups", rank = 2)]
pub fn view_groups_redirect() -> Flash<Redirect> {
    Flash::warning(Redirect::to(uri!(dashboard)), "You must be admin to view this page.")
}

#[post("/group/create", data = "<form>")]
pub fn new_group(user: AdminUser, conn: DbConn, form: Form<NewGroupForm>) -> Flash<Redirect> {
    let new_group = NewGroup {
        title: form.title.as_ref(),
    };

    match create_group(&new_group, &*conn) {
        Ok(_) => Flash::success(Redirect::to(uri!(view_groups)), "Group successfully created."),
        Err(error) => Flash::error(Redirect::to(uri!(view_groups)), format!("Database error: {}", error.to_string())),
    }
}

#[post("/group/<id>/delete")]
pub fn del_group(user: AdminUser, conn: DbConn, id: i32) -> Flash<Redirect> {
    match delete_group(id, &*conn) {
        Ok(_) => Flash::success(Redirect::to(uri!(view_groups)), "Group successfully deleted."),
        Err(error) => Flash::error(Redirect::to(uri!(view_groups)), format!("Database error: {}", error.to_string())),
    }
}