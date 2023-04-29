#![feature(proc_macro_hygiene, decl_macro)] 

use rocket;
use rocket_contrib;

mod models;
mod routes;
mod handlers;

use rocket_contrib::databases::postgres;
use rocket_contrib::templates::tera::Template;

#[database("notes")]
pub struct DbConn(postgres::Connection);

fn main() {
    rocket::ignite()
        .attach(DbConn::fairing())
        .mount("/", routes![
            routes::index,
            routes::create_note,
            routes::read_note,
            routes::edit_note,
            routes::update_note,
            routes::delete_note,
            routes::search_notes
        ])
        .attach(Template::fairing())
        .launch();
}
