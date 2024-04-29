use std::str::FromStr;

use actix_web::{delete, get, post, web, HttpResponse};
use actix_web::web::{Data, Json};
use uuid::Uuid;

use crate::DBPool;
use crate::constants::{APPLICATION_JSON, CONNECTION_POOL_ERROR};
use crate::notes::{list_notes, find_note, create_note, delete_note, NoteRequest};

#[get("/")]
async fn index() -> HttpResponse {
    /* Tested and working. */
    HttpResponse::Ok().body("Oh hello there!")
}

#[get("/notes")]
pub async fn list(pool: Data<DBPool>) -> HttpResponse {
    /* Tested and working. */
    let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
    let some_notes = web::block(move || list_notes(50, &mut conn)).await.unwrap();
    match some_notes {
        Ok(notes) => HttpResponse::Ok()
            .content_type(APPLICATION_JSON)
            .json(notes),
        _ => HttpResponse::NoContent().content_type(APPLICATION_JSON).await.unwrap()
    }
}

#[get("/notes/{id}")]
pub async fn get(path: web::Path<String>, pool: Data<DBPool>) -> HttpResponse {
    /* Tested and working. */
    let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
    let some_note = web::block(move || find_note(Uuid::from_str(path.as_str()).unwrap(), &mut conn)).await;
    match some_note {
        Ok(note) => {
            HttpResponse::Ok()
                .content_type(APPLICATION_JSON)
                .json(note.ok())
        }
        _ => HttpResponse::NoContent().content_type(APPLICATION_JSON).await.unwrap()
    }
}

#[post("/notes")]
pub async fn create(note_req: Json<NoteRequest>, pool: Data<DBPool>) -> HttpResponse {
    /* Tested and working. */
    let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
    let some_note = web::block(move || create_note(note_req.to_note().unwrap(), &mut conn)).await;
    match some_note {
        Ok(note) => HttpResponse::Created()
            .content_type(APPLICATION_JSON)
            .json(note.ok()),
        _ => HttpResponse::NoContent().content_type(APPLICATION_JSON).await.unwrap()
    }
}

// delete a note by its id
#[delete("/notes/{id}")]
pub async fn delete(path: web::Path<String>, pool: Data<DBPool>) -> HttpResponse {
    /* Tested and working. */
    let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
    let _ = web::block(move || delete_note(Uuid::from_str(path.as_str()).unwrap(), &mut conn)).await;
    HttpResponse::NoContent().content_type(APPLICATION_JSON).await.unwrap()
}