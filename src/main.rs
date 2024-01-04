#![feature(error_generic_member_access)]

#[macro_use]
extern crate rocket;

use rocket::{fs::FileServer, http::Status, Request};

use rocket_dyn_templates::{context, Template};

#[catch(404)]
fn not_found(req: &Request) -> Template {
    Template::render("tera/error/404", context! { uri: req.uri() })
}

#[catch(default)]
fn default_catcher(status: Status, req: &Request) -> Template {
    if let Some(reason) = status.reason() {
        Template::render(
            "tera/error/default_with_reason",
            context! {
                uri: req.uri(),
                code: status.code,
                reason
            },
        )
    } else {
        Template::render(
            "tera/error/default_without_reason",
            context! {
                uri: req.uri(),
                code: status.code,
            },
        )
    }
}

#[get("/")]
fn index() -> Template {
    Template::render("tera/index", context! { title: "" })
}

#[get("/about")]
fn about() -> Template {
    Template::render("tera/about", context! { title: "about" })
}

#[get("/<year>/<name>")]
fn blogpost(year: usize, name: &str) -> String {
    format!("placeholder for blogpost `{}` from `{}`", name, year)
}

#[get("/")]
fn blog() -> Template {
    Template::render("tera/blog", context! { title: "blog"})
}

#[get("/")]
fn papers() -> Template {
    Template::render("tera/papers", context! { title: "publications"})
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, about])
        .mount("/blog", routes![blog, blogpost])
        .mount("/papers", routes![papers])
        .mount("/papers/pdf", FileServer::from("papers/pdf"))
        .mount(
            "/papers/presentations",
            FileServer::from("papers/presentations"),
        )
        .attach(Template::fairing())
        .register("/", catchers![not_found, default_catcher])
}
