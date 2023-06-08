mod parser;

use std::{fs, ffi::OsString};
use parser::Post;
use rocket::fs::{FileServer, relative};
use rocket_dyn_templates::{Template, context};

pub fn search_files(query: &str) -> Option<OsString> {
    if let Ok(paths) = fs::read_dir("static/blog/") {
        for entry in paths {
            if let Ok(entry) = entry {
                let filequery = OsString::from(format!("{}.mrs", query));
                if entry.file_name() == filequery {
                    return Some(entry.file_name());
                }
            }
        }
    }
    None
} 

#[rocket::get("/blog/<path>")]
pub fn blog(path: &str) -> Template {
    let found_path = search_files(path);
    if found_path.is_none(){
        return Template::render("404", context! {
            path:path,
        })

    }
    else {
        let full_path = found_path.unwrap().into_string().unwrap();
        let post = Post::from_file(&full_path);
        return Template::render("post", context!{
            post: post.build_html(),
        })
    }

}

#[rocket::launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", rocket::routes![blog])
        .mount("/", FileServer::from(relative!("static")))
        .attach(Template::fairing())
}