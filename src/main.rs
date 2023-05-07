use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use handlers::*;

mod models;
// mod routes;
mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    // env_logger::init();

    HttpServer::new(|| {
        // let notes_db = models::NotesDb::new();

        App::new()
            // .data(notes_db)
            .route("/", web::get().to(get_all_notes))
            .route("/note", web::post().to(create_note))
            .route("/note/{id}", web::get().to(get_note_by_id))
            .route("/note/{id}", web::put().to(update_note))
            .route("/note/{id}", web::delete().to(delete_note))
            .route("/notes", web::get().to(get_all_notes))
            .wrap(Logger::default())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
