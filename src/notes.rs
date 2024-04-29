use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use serde::{Serialize, Deserialize};
use diesel::{ExpressionMethods, Insertable, Queryable, RunQueryDsl};
use diesel::query_dsl::methods::{FilterDsl, LimitDsl, OrderDsl};
use uuid::Uuid;
use diesel::result::Error;

use crate::DBPooledConnection;
use crate::response::Response;

pub type Notes = Response<Note>;

use super::schema::notes;


// ===================================================================
// NoteDB struct (struct for Diesel ORM)
// ===================================================================
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


// ===================================================================
// Note struct
// ===================================================================
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


// ===================================================================
// NoteRequest struct (used for creating a note from a request)
// ===================================================================
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


// ===================================================================
// Other functions
// ===================================================================
pub fn list_notes(total_notes: i64, conn: &mut DBPooledConnection) -> Result<Notes, Error> {
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

pub fn find_note(_id: Uuid, conn: &mut DBPooledConnection) -> Result<Note, Error> {
    use crate::schema::notes::dsl::*;

    let res = notes.filter(id.eq(_id)).load::<NoteDB>(conn);
    match res {
        Ok(notes_db) => match notes_db.first() {
            Some(note_db) => Ok(note_db.to_note()),
            _ => Err(Error::NotFound),
        },
        Err(err) => Err(err),
    }
}

pub fn create_note(note: Note, conn: &mut DBPooledConnection) -> Result<Note, Error> {
    use crate::schema::notes::dsl::*;
    let note_db_entry = note.to_note_db();
    let _ = diesel::insert_into(notes).values(&note_db_entry).execute(conn);
    Ok(note_db_entry.to_note())
}

pub fn delete_note(some_id: Uuid, conn: &mut DBPooledConnection) -> Result<(), Error> {
    use crate::schema::notes::dsl::*;
    let res = diesel::delete(notes.filter(id.eq(some_id))).execute(conn);
    match res {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}