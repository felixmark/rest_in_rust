use actix_web::{delete, get, post, web, HttpResponse};
use actix_web::web::{Data, Json, Path};
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use log::info;
use serde::{Serialize, Deserialize};
use diesel::{ExpressionMethods, Insertable, Queryable, RunQueryDsl};
use diesel::query_dsl::methods::{FilterDsl, LimitDsl, OrderDsl};
use uuid::Uuid;
use std::str::FromStr;
use diesel::result::Error;

use crate::{DBPool, DBPooledConnection};
use crate::response::Response;
use crate::constants::{APPLICATION_JSON, CONNECTION_POOL_ERROR};

pub type Notes = Response<Note>;

use super::schema::notes;

#[derive(Debug, Serialize, Deserialize)]
pub struct Note {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub content: String,
}

impl Note {
    pub fn new(content: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            content,
        }
    }

    pub fn to_note_db(&self) -> NoteDB {
        NoteDB {
            id: Uuid::new_v4(),
            timestamp: Utc::now().naive_utc(),
            content: self.content.clone(),
        }
    }
}



#[derive(Debug, Serialize, Deserialize)]
pub struct NoteRequest {
    pub content: Option<String>,
}

impl NoteRequest {
    pub fn to_note(&self) -> Option<Note> {
        match &self.content {
            Some(content) => Some(Note::new(content.to_string())),
            None => None,
        }
    }
}



#[derive(Queryable, Insertable)]
#[table_name = "notes"]
pub struct NoteDB {
    pub id: Uuid,
    pub timestamp: NaiveDateTime,
    pub content: String,
}

impl NoteDB {
    fn to_note(&self) -> Note {
        Note {
            id: self.id.to_string(),
            timestamp: Utc.from_utc_datetime(&self.timestamp),
            content: self.content.clone(),
        }
    }
}

fn list_notes(total_notes: i64, conn: &mut DBPooledConnection) -> Result<Notes, Error> {
    use crate::schema::notes::dsl::*;
    let _tweets = match notes
        .order(timestamp.desc())
        .limit(total_notes)
        .load::<NoteDB>(conn)
    {
        Ok(tws) => tws,
        Err(_) => vec![],
    };

    Ok(Notes {
        results: _tweets
            .into_iter()
            .map(|t| t.to_note())
            .collect::<Vec<Note>>(),
    })
}

#[get("/notes")]
pub async fn list(pool: Data<DBPool>) -> HttpResponse {
    /* Tested and working. */
    let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
    let mut _notes = web::block(move || list_notes(50, &mut conn)).await.unwrap();
    match _notes {
        Ok(_) => HttpResponse::Ok()
            .content_type(APPLICATION_JSON)
            .json(_notes.ok()),
        _ => HttpResponse::Ok()
            .content_type(APPLICATION_JSON)
            .json(())
    }
}

fn create_note(note: Note, conn: &mut DBPooledConnection) -> Result<Note, Error> {
    use crate::schema::notes::dsl::*;
    let note_db_entry = note.to_note_db();
    let _ = diesel::insert_into(notes).values(&note_db_entry).execute(conn);
    Ok(note_db_entry.to_note())
}

#[post("/notes")]
pub async fn create(note_req: Json<NoteRequest>, pool: Data<DBPool>) -> HttpResponse {
    /* Tested and working. */
    let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
    let note = web::block(move || create_note(note_req.to_note().unwrap(), &mut conn)).await;
    match note {
        Ok(note) => HttpResponse::Created()
            .content_type(APPLICATION_JSON)
            .json(note.ok()),
        _ => HttpResponse::NoContent().await.unwrap(),
    }
}

// find a note by its id
#[get("/notes/{id}")]
pub async fn get(path: web::Path<String>) -> HttpResponse {
    let found_note: Option<Note> = Some(
        Note { id: "10".to_string(), timestamp: Utc::now(), content: path.to_string()}
    );
    match found_note {
        Some(note) => HttpResponse::Ok()
            .content_type(APPLICATION_JSON)
            .json(note),
        None => HttpResponse::NoContent()
            .content_type(APPLICATION_JSON)
            .await
            .unwrap(),
    }
}

// delete a note by its id
#[delete("/notes/{id}")]
pub async fn delete(path: web::Path<(String,)>) -> HttpResponse {
    HttpResponse::NoContent()
        .content_type(APPLICATION_JSON)
        .await
        .unwrap()
}