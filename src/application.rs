use super::models::{User, AdminUser};
use rocket::response::{Flash, Redirect};
use rocket_contrib::templates::Template;
use rocket::request::{FlashMessage, Form};
use crate::DbConn;
use crate::database::{get_users, get_user, update_user};


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

#[derive(serde::Serialize)]
pub struct Context<'a> {
    pub flash: Option<String>,
    pub flash_type: Option<String>,
    pub user: Option<&'a User>,
    pub members: Option<Vec<User>>,
}

impl Context<'_> {
    pub fn new() -> Self {
        Context {
            flash: None,
            flash_type: None,
            user: None,
            members: None
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

#[get("/dashboard")]
pub fn dashboard(user: &User, flash: Option<FlashMessage<'_, '_>>) -> Template {
    let mut context = Context::new();

    if let Some(ref msg) = flash {
        context.parse_falsh_message(msg);
    }
    context.user = Some(user);

    Template::render("dashboard", &context)
}

#[get("/dashboard", rank = 2)]
pub fn admin_panel_redirect() -> Flash<Redirect> {
    Flash::warning(Redirect::to(uri!(super::auth::login)), "Please login to visit this page")
}

#[get("/")]
pub fn index() -> Redirect {
    Redirect::to("/login")
}

#[get("/members")]
pub fn members(user: AdminUser, flash: Option<FlashMessage<'_, '_>>, conn: DbConn) -> Template {
    let mut context = Context::new();

    if let Some(ref msg) = flash {
        context.parse_falsh_message(msg);
    }
    context.user = Some(user.0);

    if let Ok(members) = get_users(&*conn) {
        context.members = Some(members);
    }

    Template::render("members", &context)
}

#[get("/members", rank = 2)]
pub fn members_redirect() -> Flash<Redirect> {
    Flash::warning(Redirect::to(uri!(dashboard)), "You must be admin to view this page.")
}

#[get("/member/<id>")]
pub fn member(user: &User, id: i32, conn: DbConn, flash: Option<FlashMessage<'_, '_>>) -> Result<Template, Flash<Redirect>> {
    if user.id != id && !user.is_admin { // You're only allowed to view your own profile, except for admins.
        return Err(Flash::error(Redirect::to(uri!(dashboard)), "You're not allowed to access this page."));
    }

    let mut context = Context::new();

    if let Some(ref msg) = flash {
        context.parse_falsh_message(msg);
    }
    context.user = Some(user);

    if let Ok(member) = get_user(id, &*conn) {
        context.members = Some(vec![member]);
    }

    Ok(Template::render("member", &context))
}

#[get("/member/<id>", rank = 2)]
pub fn member_redirect(id: i32) -> Flash<Redirect> {
    Flash::warning(Redirect::to(uri!(super::auth::login)), "Please login")
}

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