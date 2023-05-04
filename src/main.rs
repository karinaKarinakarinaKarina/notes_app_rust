use actix_web::{web, App, HttpServer};
use actix_web::middleware::Logger;

mod models;
mod routes;
mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        let notes_db = models::NotesDb::new();

        App::new()
            .data(notes_db)
            .route("/", web::get().to(routes::index))
            .route("/note", web::post().to(routes::create_note))
            .route("/note/{id}", web::get().to(routes::read_note))
            .route("/note/{id}", web::put().to(routes::update_note))
            .route("/note/{id}", web::delete().to(routes::delete_note))
            .route("/notes", web::get().to(routes::search_notes))
            .wrap(Logger::default())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
