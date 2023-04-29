use actix_web::{web, HttpResponse, Responder};
use serde_json::json;

use crate::models::Note;

pub async fn create_note(note: web::Json<Note>) -> impl Responder {
    // код для сохранения заметки в базе данных
    let response = json!({
        "status": "success",
        "message": "Note created successfully"
    });
    HttpResponse::Ok().json(response)
}

pub async fn get_note_by_id(id: web::Path<i32>) -> impl Responder {
    // код для получения заметки из базы данных по id
    let note = Note::new(id, String::from("Sample Note"), String::from("This is a sample note."));
    HttpResponse::Ok().json(note)
}

pub async fn update_note(id: web::Path<i32>, note: web::Json<Note>) -> impl Responder {
    // код для обновления заметки в базе данных по id
    let response = json!({
        "status": "success",
        "message": "Note updated successfully"
    });
    HttpResponse::Ok().json(response)
}

pub async fn delete_note(id: web::Path<i32>) -> impl Responder {
    // код для удаления заметки из базы данных по id
    let response = json!({
        "status": "success",
        "message": "Note deleted successfully"
    });
    HttpResponse::Ok().json(response)
}