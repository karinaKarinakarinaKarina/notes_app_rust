use actix_web::{web, HttpResponse};
use crate::handlers::*;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/notes")
            .route(web::post().to(create_note))
            .route(web::get().to(get_all_notes))
    );
    cfg.service(
        web::resource("/notes/{id}")
            .route(web::get().to(get_note_by_id))
            .route(web::put().to(update_note))
            .route(web::delete().to(delete_note))
    );
}