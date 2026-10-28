use super::model::BooksModel;
use super::repository;
use crate::database::service::DatabaseService;
use rusqlite::Result;
use std::sync::Arc;

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

    pub fn get_one(&self, item_id: i32) -> Result<Option<BooksModel>> {
        println!("BookService::get_one fn");
        let conn = self.db.get_conn().lock().map_err(|_| rusqlite::Error::InvalidQuery)?;

        repository::search_book_by_id(&conn, item_id)
    }

    pub fn by_author(&self, item_id: i32) -> Result<Vec<BooksModel>> {
        println!("BookService::by_author fn");
        let conn = self.db.get_conn().lock().map_err(|_| rusqlite::Error::InvalidQuery)?;

        repository::all_books_by_author(&conn, item_id)
    }

    #[allow(dead_code)]
    pub fn create_book(&self, author: &BooksModel) -> Result<usize> {
        let conn = self.db.get_conn().lock().map_err(|_| rusqlite::Error::InvalidQuery)?;

        repository::add_book(&conn, author)
    }

    pub fn remove_book(&self, item_id: i32) -> Result<usize> {
        let conn = self.db.get_conn().lock().map_err(|_| rusqlite::Error::InvalidQuery)?;

        repository::delete_book(&conn, item_id)
    }
}
