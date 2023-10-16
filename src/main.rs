#[macro_use] extern crate rocket;
use rocket::fs::NamedFile;
use rocket_db_pools::{Database, mongodb};
use std::path::{Path, PathBuf};
use std::io;
mod api;
mod cors;

#[derive(Database)]
#[database("mongodb_projects")]
pub struct ProjectDB(mongodb::Client);

#[get("/<file..>")]
async fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("build/").join(file)).await.ok()
}

#[get("/")]
async fn index() -> io::Result<NamedFile> {
    NamedFile::open("build/index.html").await
}

#[catch(404)]
async fn not_found() -> io::Result<NamedFile> {
    NamedFile::open("build/index.html").await
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(cors::CORS)
        .attach(ProjectDB::init())
        .register("/", catchers![not_found])
        .mount("/", routes![index, files])
        .mount("/api", routes![api::projects, api::project_info])
}
