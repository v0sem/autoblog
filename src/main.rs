use rocket::fs::{FileServer, relative};
use rocket_dyn_templates::Template;

// If we wanted or needed to serve files manually, we'd use `NamedFile`. Always
// prefer to use `FileServer`!
mod manual {
    use rocket_dyn_templates::{Template, context};

    #[rocket::get("/blog/<path>")]
    pub fn blog(path: &str) -> Template {
        Template::render("404", context! {
            path:path,
        })
    }
}

#[rocket::launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", rocket::routes![manual::blog])
        .mount("/", FileServer::from(relative!("static")))
        .attach(Template::fairing())
}