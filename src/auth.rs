use rocket::response::{Flash, Redirect};
use rocket_contrib::templates::Template;
use rocket::request::Form;

#[derive(FromForm)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
}

#[get("/login")]
pub fn login() -> Template {
    let mut context = std::collections::HashMap::<&str, &str>::new();
    Template::render("login", &context)
}

#[post("/login", data = "<form>")]
pub fn login_form(form: Form<LoginForm>) -> Redirect {
    Redirect::to(uri!(super::application::dashboard))
}

/*
#[get("/admin")]
fn admin_panel(admin: AdminUser) -> &'static str {
    "Admin Panel"
}

#[get("/admin", rank = 2)]
fn admin_panel(user: User) -> &'static str {
    "Sorry, you must be an admin to access this page"
}

#[get("/admin", rank = 3)]
fn admin_panel_redirect() -> Redirect {
   Redirect::to(uri!(login)) 
}
*/
