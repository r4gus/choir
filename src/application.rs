use super::models::{User, AdminUser, NewUser};
use rocket::response::{Flash, Redirect};
use rocket_contrib::templates::Template;
use rocket_contrib::json::Json;
use rocket::request::{FlashMessage, Form, FromFormValue};
use crate::DbConn;
use crate::database::{get_users, get_user, update_user, delete_user, create_user, get_user_by_mail, get_groups, create_group, delete_group, get_user_for_group, add_user_to_group, delete_user_from_group, get_appointments, create_appointment, get_appointment, update_appointment, delete_appointment, get_future_appointments, get_participants_for_appointment, add_member_to_event, delete_participant, get_participants_of_group};
use rocket::http::{Cookies, Cookie, RawStr};
use diesel::result::Error;
use crate::models::{Group, NewGroup, Appointment, NewAppointment, Participate};
use std::collections::HashMap;
use chrono::NaiveDateTime;
use rocket::response::status;


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

#[derive(FromForm)]
pub struct NewAppointmentForm {
    pub title: String,
    pub place: String,
    pub begins: NaiveDateTimeWrapper,
    pub ends: NaiveDateTimeWrapper,
    pub description: String,
}

pub struct NaiveDateTimeWrapper(chrono::NaiveDateTime);

impl<'v> FromFormValue<'v> for NaiveDateTimeWrapper {
    type Error = &'v RawStr;

    fn from_form_value(form_value: &'v RawStr) -> Result<NaiveDateTimeWrapper, &'v RawStr> {
        match form_value.percent_decode() {
            Ok(s) => {
                println!("{}", s.to_string());
                match NaiveDateTime::parse_from_str(&s.to_string(), "%Y-%m-%d+%H:%M:%S") {
                    Ok(dt) => Ok(NaiveDateTimeWrapper(dt)),
                    Err(error) => Err(form_value),
                }
            },
            Err(error) => Err(form_value),
        }
    }
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
pub fn dashboard(user: &User, conn: DbConn, flash: Option<FlashMessage<'_, '_>>) -> Template {
    let mut context = Context::<(Vec<(Appointment, HashMap<i32, i32>)>, Vec<(Group, Vec<User>)>)>::new();

    if let Some(ref msg) = flash {
        context.parse_falsh_message(msg);
    }
    context.user = Some(user);

    if let Ok(apps) = get_future_appointments(&*conn) {
        if let Ok(groups) = get_groups(&*conn) {
            let mut gv: Vec<(Group, Vec<User>)> = Vec::new();

            for group in groups.into_iter() {
                if let Ok(users) = get_user_for_group(group.id, &*conn) {
                    gv.push((group, users));
                }
            }

            let mut appsu = Vec::<(Appointment, HashMap<i32, i32>)>::new();

            for app in apps.into_iter() {
                if let Ok(parts) = get_participants_for_appointment(app.id, &*conn) {
                    let mut map = HashMap::<i32, i32>::new();

                    for (u, g) in parts.into_iter() {
                        map.insert(u, g);
                    }

                    appsu.push((app, map));
                }
            }

            context.collection = Some((appsu, gv));
        }
    }

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
    let mut context = Context::<Vec<(Group, Vec<User>, Vec<User>)>>::new();

    if let Some(ref msg) = flash {
        context.parse_falsh_message(msg);
    }
    context.user = Some(user.0);

    if let Ok(groups) = get_groups(&*conn) {
        let mut new_collection: Vec<(Group, Vec<User>, Vec<User>)> = Vec::new();
        let users = match get_users(&*conn) {
            Ok(users) => users,
            Err(_) => Vec::<User>::new(),
        };

        for group in groups {
            let v = match get_user_for_group(group.id, &*conn) {
                Ok(uvec) => uvec,
                Err(_) => Vec::<User>::new()
            };

            let mut addable_users = Vec::<User>::new();
            for u in users.iter() {
                if !v.contains(u) {
                    addable_users.push(u.clone());
                }
            }


            new_collection.push((group, v, addable_users));
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

#[derive(FromForm)]
pub struct InsertUserForm {
    pub user: i32,
}

#[post("/group/<id>/insert", data = "<form>")]
pub fn insert_into_group(user: AdminUser, conn: DbConn, id: i32, form: Form<InsertUserForm>) -> Flash<Redirect> {
    match add_user_to_group(id, form.user, &*conn) {
        Ok(_) => Flash::success(Redirect::to(uri!(view_groups)), "Users added."),
        Err(err) => Flash::error(Redirect::to(uri!(view_groups)), format!("Database error: {}", err.to_string())),
    }
}

#[post("/group/<id>/remove/<uid>")]
pub fn remove_from_group(user: AdminUser, conn: DbConn, id: i32, uid: i32) -> Flash<Redirect> {
    match delete_user_from_group(id, uid, &*conn) {
        Ok(_) => Flash::success(Redirect::to(uri!(view_groups)), "Users successfully removed."),
        Err(err) => Flash::error(Redirect::to(uri!(view_groups)), format!("Database error: {}", err.to_string())),
    }
}

#[get("/appointments")]
pub fn view_appointments(user: AdminUser, conn: DbConn, flash: Option<FlashMessage<'_, '_>>) -> Template {
    let mut context = Context::<Vec<Appointment>>::new();

    if let Some(ref msg) = flash {
        context.parse_falsh_message(msg);
    }
    context.user = Some(user.0);

    if let Ok(appmnts) = get_appointments(&*conn) {
        context.collection = Some(appmnts);
    }

    Template::render("appointments", &context)
}


#[post("/appointment/new", data = "<form>")]
pub fn new_appointment(user: AdminUser, conn: DbConn, form: Form<NewAppointmentForm>) -> Flash<Redirect> {
    let appointment = NewAppointment {
        title: form.title.clone(),
        place: form.place.clone(),
        begins: form.begins.0.clone(),
        ends: form.ends.0.clone(),
        description: form.description.clone(),
    };

    match create_appointment(&appointment, &*conn) {
        Ok(a) => Flash::success(Redirect::to(uri!(view_appointments)), format!("{} erfolgreich erstellt.", a.title)),
        Err(err) => Flash::error(Redirect::to(uri!(view_appointments)), format!("Database error: {}", err.to_string())),
    }
}

#[get("/appointment/<id>")]
pub fn view_appointment(user: AdminUser, conn: DbConn, id: i32, flash: Option<FlashMessage<'_, '_>>) -> Result<Template, Flash<Redirect>> {
    match get_appointment(id, &*conn) {
        Ok(ap) => {
            let mut context = Context::<Appointment>::new();

            if let Some(ref msg) = flash {
                context.parse_falsh_message(msg);
            }
            context.user = Some(user.0);
            context.collection = Some(ap);
            Ok(Template::render("appointment", &context))
        },
        Err(error) => Err(Flash::warning(Redirect::to(uri!(view_appointments)), "Der angefragte Termin scheint nicht zu existieren.")),
    }
}

#[post("/appointment/<id>/update", data = "<form>")]
pub fn appointment_update(user: AdminUser, conn: DbConn, id: i32, form: Form<NewAppointmentForm>) -> Flash<Redirect> {
        let app_update = Appointment {
            id: id,
            title: form.title.clone(),
            place: form.place.clone(),
            begins: form.begins.0.clone(),
            ends: form.ends.0.clone(),
            description: form.description.clone(),
        };

        match update_appointment(&app_update, &*conn) {
            Ok(_) => Flash::success(Redirect::to(format!("/appointment/{}", id)), "Änderungen gespeichert."),
            Err(err) => Flash::error(Redirect::to(format!("/appointment/{}", id)), format!("Änderungen konnten nicht gespeichert werden: {}", err.to_string())),
        }
}

#[post("/appointment/<id>/delete")]
pub fn appointment_delete(user: AdminUser, conn: DbConn, id: i32) -> Flash<Redirect> {
    match delete_appointment(id, &*conn) {
        Ok(_) => Flash::success(Redirect::to(uri!(view_appointments)), "Termin wurde gelöscht."),
        Err(err) => Flash::error(Redirect::to(uri!(view_appointments)), format!("Database error: {}", err.to_string())),
    }
}

#[get("/participate/<aid>/<gid>/<uid>/join")]
pub fn join_appointment(user: &User, conn: DbConn, aid: i32, gid: i32, uid: i32) -> Flash<Redirect> {
    if user.id != uid && !user.is_admin {
        return Flash::warning(Redirect::to(uri!(dashboard)), "Behalte deine Finger bei dir!");
    }

    match add_member_to_event(aid, gid, uid, &*conn) {
        Ok(_) => Flash::success(Redirect::to(uri!(dashboard)), "Anmeldung erfolgreich."),
        Err(_) => Flash::warning(Redirect::to(uri!(dashboard)), "Pro Veranstaltung is nur eine Anmeldung erlaubt."),
    }
}

#[get("/participate/<aid>/<gid>/<uid>/revoke")]
pub fn leave_appointment(user: &User, conn: DbConn, aid: i32, gid: i32, uid: i32) -> Flash<Redirect> {
    if user.id != uid && !user.is_admin {
        return Flash::warning(Redirect::to(uri!(dashboard)), "Behalte deine Finger bei dir!");
    }

    match delete_participant(aid, uid, &*conn) {
        Ok(_) => Flash::success(Redirect::to(uri!(dashboard)), "Abmeldung erfolgreich."),
        Err(_) => Flash::warning(Redirect::to(uri!(dashboard)), "Konnte nicht abgemeldet werden."),
    }
}

#[post("/participate/join", data = "<form>")]
pub fn join(user: &User, conn: DbConn, form: Form<Participate>) -> Result<status::Accepted<String>, status::Forbidden<String>> {
    if user.id != form.uid && !user.is_admin {
        return Err(status::Forbidden(Some("Behalte deine Finger bei dir!".to_string())));
    }

    match add_member_to_event(form.aid, form.gid, form.uid, &*conn) {
        Ok(_) => Ok(status::Accepted(Some(String::from("Anmeldung erfolgreich.")))),
        Err(_) => Err(status::Forbidden(Some("Behalte deine Finger bei dir!".to_string()))),
    }
}

#[post("/participate/revoke", data = "<form>")]
pub fn revoke(user: &User, conn: DbConn, form: Form<Participate>) -> Result<status::Accepted<String>, status::Forbidden<String>> {
    if user.id != form.uid && !user.is_admin {
        return Err(status::Forbidden(Some("Behalte deine Finger bei dir!".to_string())));
    }

    match delete_participant(form.aid, form.uid, &*conn) {
        Ok(_) => Ok(status::Accepted(Some(String::from("Abmeldung erfolgreich.")))),
        Err(_) => Err(status::Forbidden(Some("Behalte deine Finger bei dir!".to_string()))),
    }
}

#[derive(Debug, serde::Serialize)]
pub struct AppointmentInfo {
    pub appointment: Appointment,
    pub groups: Vec<(Group, Vec<User>)>,
}

#[get("/participate/<aid>/info")]
pub fn get_appointment_info(user: &User, conn: DbConn, aid: i32) -> Result<Json<AppointmentInfo>, status::NotFound<String>> {
    if let Ok(a) = get_appointment(aid, &*conn) {
        let mut gvec = Vec::<(Group, Vec<User>)>::new();

        if let Ok(groups) = get_groups(&*conn) {
            for g in groups.into_iter() {
                if let Ok(users) = get_participants_of_group(aid,g.id, &*conn) {
                    gvec.push((g, users));
                }
            }
        }

        Ok(Json(AppointmentInfo {
            appointment: a,
            groups: gvec,
        }))
    } else {
        Err(status::NotFound("Ein Event mit der gegebenen ID existiert nicht.".to_string()))
    }
}