#![feature(proc_macro_hygiene, decl_macro)]

use rocket::fairing::AdHoc;
use rocket::response::NamedFile;
use rocket::State;
use std::path::{Path, PathBuf};

struct AssetsDir(String);

#[macro_use]
extern crate rocket;

#[get("/")]
fn index(assets_dir: State<AssetsDir>) -> Option<NamedFile> {
    NamedFile::open(Path::new(&assets_dir.0).join("index.html")).ok()
}

#[get("/<asset..>", rank = 1)]
fn static_assets(asset: PathBuf, assets_dir: State<AssetsDir>) -> Option<NamedFile> {
    NamedFile::open(Path::new(&assets_dir.0).join(asset)).ok()
}

#[get("/api/<msg..>")]
fn api(msg: PathBuf) -> String {
    let x = msg.to_str().unwrap_or("No msg").to_string();
    return x;
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, api, static_assets])
        .attach(AdHoc::on_attach("Static Assets Configuration", |rocket| {
            let assets_dir = rocket
                .config()
                .get_str("static_directory")
                .unwrap_or("../web-frontend/dist/")
                .to_string();
            Ok(rocket.manage(AssetsDir(assets_dir)))
        }))
        .launch();
}
