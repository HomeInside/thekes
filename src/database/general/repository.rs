use rusqlite::OptionalExtension;
use rusqlite::{Connection, Result};

use super::model::StatsModel;

/// estadisticas de DB
pub fn stats_counter(conn: &Connection) -> Result<Option<StatsModel>> {
    conn.query_row(
        "WITH random_book AS MATERIALIZED (
          SELECT b.id, b.title, b.published_date, b.cover_color,
            a.name || ' ' || a.last_name AS names
          FROM books b
          JOIN authors a ON a.id = b.author_id
          WHERE b.favorite IS NOT NULL AND b.favorite == 0
          ORDER BY
            RANDOM()
          LIMIT 1
        )
        SELECT
            (SELECT COUNT(id) FROM authors) AS a_count,
            (SELECT COUNT(id) FROM books) AS b_count,
            (SELECT COUNT(id) FROM books WHERE favorite != 0) AS fav_count,
            (SELECT COUNT(DISTINCT publisher) FROM books
                WHERE publisher IS NOT NULL) AS pub_count,
            (SELECT COUNT(DISTINCT genres) FROM books
                WHERE genres IS NOT NULL) AS genre_count,
            (SELECT COUNT(DISTINCT lang) FROM books
                WHERE lang IS NOT NULL) AS lang_count,
            (SELECT quote_text FROM quotes ORDER BY RANDOM() LIMIT 1) AS quote_text,
            (SELECT title FROM random_book) AS rb_title,
            (SELECT published_date FROM random_book) AS rb_published,
            (SELECT cover_color FROM random_book) AS rb_cover_color,
            (SELECT names FROM random_book) AS r_author;",
        [],
        |row| {
            Ok(StatsModel {
                authors: row.get("a_count")?,
                books: row.get("b_count")?,
                favs: row.get("fav_count")?,
                publ: row.get("pub_count")?,
                genre: row.get("genre_count")?,
                lang: row.get("lang_count")?,
                quote: row.get("quote_text")?,
                rb_title: row.get("rb_title")?,
                rb_published: row.get("rb_published")?,
                rb_cover_color: row.get("rb_cover_color")?,
                r_author: row.get("r_author")?,
            })
        },
    )
    .optional()
}
