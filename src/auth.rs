use rocket::response::{Flash, Redirect};
use rocket_contrib::templates::Template;
use rocket::request::{Form, FlashMessage};
use rocket::http::{Cookies, Cookie};
use crate::database::get_user_by_mail;
use crate::DbConn;
use argon2::{self, Config};
use rocket::response::status::NotFound;

/// Generate a random string meant to be used as a salt.
///
/// This function can actually be used to generate a random string of length `len`
/// for any purpose.
///
/// # Arguments
///
/// * `len` - Length of the expected string.
///
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

#[derive(FromForm, Debug)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
}

#[get("/login")]
pub fn login(flash: Option<FlashMessage<'_, '_>>) -> Template {
    let mut context = std::collections::HashMap::<&str, &str>::new();
    if let Some(ref msg) = flash {
        context.insert("flash", msg.msg());
        match msg.name() {
            "error" => context.insert("flash_type", "alert-danger"),
            "warning" => context.insert("flash_type", "alert-warning"),
            _ => context.insert("flash_type", "alert-success"),
        };
    }

    Template::render("login", &context)
}


#[post("/login", data = "<form>")]
pub fn login_form(form: Form<LoginForm>, mut cookies: Cookies, conn: DbConn) -> Flash<Redirect> {
    let result = get_user_by_mail(&form.email, &*conn); // Try to retrieve user from database

    if let Ok(user) = result {
        if !user.verified {
            return Flash::warning(Redirect::to(uri!(login)), "You're account hasn't been validated yet. Please contact your administrator.");
        }

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
        return match result.err().unwrap() {
            NotFoud => Flash::warning(Redirect::to(uri!(login)), "Invalid username or password"), // Invalid e-mail
            _ => Flash::error(Redirect::to(uri!(login)), "Internal server error"),
        };
    }
}

#[get("/logout")]
pub fn logout(mut cookies: Cookies) -> Flash<Redirect> {
    cookies.remove_private(Cookie::named("user_id"));
    Flash::success(Redirect::to("/"), "Successfully logged out.")
}
