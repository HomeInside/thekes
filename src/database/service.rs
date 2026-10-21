use super::authors::model::AuthorModel;
use super::authors::repository;
use rusqlite::Connection;
use std::error::Error;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct DatabaseService {
    conn: Arc<Mutex<Connection>>,
}

impl DatabaseService {
    pub fn new(conn: Connection) -> Self {
        Self {
            conn: Arc::new(Mutex::new(conn)),
        }
    }

    pub(crate) fn get_conn(&self) -> &Arc<Mutex<Connection>> {
        &self.conn
    }
}
