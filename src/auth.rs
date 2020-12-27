use rocket::response::{Flash, Redirect};
use rocket_contrib::templates::Template;
use rocket::request::Form;
use rocket::http::{Cookies, Cookie};
use crate::database::get_user_by_mail;
use crate::DbConn;
use argon2::{self, Config};

pub fn generate_salt(len: usize) -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";
    let mut rng = rand::thread_rng();

    let salt: String = (0..len).map(|_| {
        let idx = rng.gen_range(0..CHARSET.len());
        CHARSET[idx] as char
    }).collect();

    salt
}

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
pub fn login_form(form: Form<LoginForm>, mut cookies: Cookies, conn: DbConn) -> Flash<Redirect> {
    let result = get_user_by_mail(&form.email, &*conn); // Try to retrieve user from database

    if let Ok(user) = result {
        match argon2::verify_encoded(&user.password_hash, form.password.as_ref()) {
            Ok(matches) => {
                if matches {
                    cookies.add_private(Cookie::new("user_id", format!("{}", user.id)));
                    return Flash::success(Redirect::to(uri!(super::application::dashboard)), "Login successful");
                } else {
                    return Flash::warning(Redirect::to(uri!(login)), "Invalid username or password"); // Invalid password
                }
            },
            Err(_) => return Flash::error(Redirect::to(uri!(login)), "Unexpected decryption error"),
        }
    } else {
        let mut str: &str = "";
        match result.err().unwrap() {
            NotFoud => str = "Invalid e-mail or password", // Invalid e-mail
            _ => str = "Internal server error",
        }

        return Flash::error(Redirect::to(uri!(login)), str);
    }
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
