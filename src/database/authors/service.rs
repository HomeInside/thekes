use super::model::AuthorModel;
use super::repository;
use crate::database::service::DatabaseService;
use rusqlite::Result;
//use std::sync::MutexGuard;
use std::sync::Arc;

/*
#[derive(Clone)]
pub struct AuthorService {
    db: DatabaseService,
}

impl AuthorService {
    pub fn new(db: DatabaseService) -> Self {
        Self { db }
    }
*/

#[derive(Clone)]
pub struct AuthorService {
    db: Arc<DatabaseService>,
}

impl AuthorService {
    pub fn new(db: Arc<DatabaseService>) -> Self {
        Self { db }
    }
}

impl AuthorService {
    pub fn get_all(&self) -> Result<Vec<AuthorModel>> {
        println!("AuthorService::get_all fn");
        let conn = self.db.get_conn().lock().map_err(|_| rusqlite::Error::InvalidQuery)?;

        repository::all_authors(&conn)
    }

    pub fn get_one(&self, item_id: i32) -> Result<Option<AuthorModel>> {
        println!("AuthorService::get_one fn");
        let conn = self.db.get_conn().lock().map_err(|_| rusqlite::Error::InvalidQuery)?;

        repository::search_author_by_id(&conn, item_id)
    }

    #[allow(dead_code)]
    pub fn create_author(&self, author: &AuthorModel) -> Result<usize> {
        let conn = self.db.get_conn().lock().map_err(|_| rusqlite::Error::InvalidQuery)?;

        repository::add_author(&conn, author)
    }

    pub fn remove_author(&self, item_id: i32) -> Result<usize> {
        let conn = self.db.get_conn().lock().map_err(|_| rusqlite::Error::InvalidQuery)?;

        repository::delete_author(&conn, item_id)
    }
}
