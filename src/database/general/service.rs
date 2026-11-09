use super::model::StatsModel;
use super::repository;
use crate::database::service::DatabaseService;
use rusqlite::Result;
use std::sync::Arc;

#[derive(Clone)]
pub struct GeneralService {
    db: Arc<DatabaseService>,
}

impl GeneralService {
    pub fn new(db: Arc<DatabaseService>) -> Self {
        Self { db }
    }
}

impl GeneralService {
    /// obtiene estadisticas de algunas tablas
    pub fn get_stats(&self) -> Result<Option<StatsModel>> {
        let conn = self.db.get_conn().lock().map_err(|_| rusqlite::Error::InvalidQuery)?;

        repository::stats_counter(&conn)
    }
}
