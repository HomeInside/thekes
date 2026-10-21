use super::model::BooksModel;
use super::repository;
use crate::database::service::DatabaseService;
use rusqlite::Result;
use std::sync::Arc;

/*
#[derive(Clone)]
pub struct BookService {
    db: DatabaseService,
}

impl BookService {
    pub fn new(db: DatabaseService) -> Self {
        Self { db }
    }
*/

#[derive(Clone)]
pub struct BookService {
    db: Arc<DatabaseService>,
}

impl BookService {
    pub fn new(db: Arc<DatabaseService>) -> Self {
        Self { db }
    }
}

impl BookService {
    pub fn get_all(&self) -> Result<Vec<BooksModel>> {
        println!("BookService::get_all fn");
        let conn = self.db.get_conn().lock().map_err(|_| rusqlite::Error::InvalidQuery)?;

        repository::all_books(&conn)
    }

    pub fn create_book(&self, author: &BooksModel) -> Result<usize> {
        let conn = self.db.get_conn().lock().map_err(|_| rusqlite::Error::InvalidQuery)?;

        repository::add_book(&conn, author)
    }
}
