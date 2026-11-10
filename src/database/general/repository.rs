use rusqlite::OptionalExtension;
use rusqlite::{Connection, Result};

use super::model::StatsModel;

/// estadisticas de DB
pub fn stats_counter(conn: &Connection) -> Result<Option<StatsModel>> {
    conn.query_row(
        "SELECT (SELECT COUNT(*) FROM authors) AS a_count, (SELECT COUNT(*) FROM books) AS b_count, (SELECT COUNT(*) FROM books where books.favorite!=0) AS fav_count;",
        [],
        |row| {
            Ok(StatsModel {
                authors: row.get("a_count")?,
                books: row.get("b_count")?,
                favs: row.get("fav_count")?,
            })
        },
    )
    .optional()
}
