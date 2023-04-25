#![feature(proc_macro_hygiene, decl_macro)]

use rocket::{get, routes};
use rocket::fs::NamedFile;
use rocket_dyn_templates::Template;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[get("/")]
fn index() -> Template {
    let context: HashMap<String, String> = HashMap::new();
    Template::render("index", &context)
}

#[get("/tools")]
fn tools() -> Template {
    let context: HashMap<String, String> = HashMap::new();
    Template::render("tools", &context)
}

#[get("/bitflipfinder")]
fn bitflipfinder() -> Template {
    let context: HashMap<String, String> = HashMap::new();
    Template::render("bitflipfinder", &context)
}

#[get("/static/<file..>", rank = 2)]
async fn static_files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).await.ok()
}

fn rocket() -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .attach(Template::fairing())
        .mount("/", routes![index, tools, bitflipfinder, static_files])
}

#[rocket::main]
async fn main() {
    if let Err(e) = rocket().launch().await {
        println!("Rocket failed to launch: {:?}", e);
    }
}
