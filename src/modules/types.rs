
use r2d2::PooledConnection;
use crate::modules::response::Response;
use crate::modules::notes::Note;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

pub type Notes = Response<Note>;
pub type DBPool = Pool<ConnectionManager<PgConnection>>;
pub type DBPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;