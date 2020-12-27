use super::models::{User};
use rocket::response::{Flash, Redirect};
use rocket_contrib::templates::Template;
use rocket::request::FlashMessage;

#[get("/dashboard")]
pub fn dashboard(user: &User, flash: Option<FlashMessage<'_, '_>>) -> Template {
    let mut context = std::collections::HashMap::<&str, &str>::new();
    if let Some(ref msg) = flash {
        context.insert("flash", msg.msg());
        match msg.name() {
            "error" => context.insert("flash_type", "alert-danger"),
            "warning" => context.insert("flash_type", "alert-warning"),
            _ => context.insert("flash_type", "alert-success"),
        };
    }

    Template::render("dashboard", &context)
}

#[get("/dashboard", rank = 2)]
pub fn admin_panel_redirect() -> Flash<Redirect> {
    Flash::warning(Redirect::to(uri!(super::auth::login)), "Please log-in to visit this page")
}

#[get("/")]
pub fn index() -> Redirect {
    Redirect::to("/login")
}