use super::seed::add_default_author;
use crate::utils::file_utils;
use rusqlite::{Connection, OpenFlags, Result};
use std::error::Error;

pub fn connect() -> Result<Connection, Box<dyn Error>> {
    println!("connect db...");
    let path = file_utils::db_path()?;

    let conn = Connection::open_with_flags(&path, OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE)?;

    println!("connecting to: {:?}... is ok", path.display());

    Ok(conn)
}

pub fn init_db(conn: &Connection) -> Result<(), Box<dyn Error>> {
    println!("init_db db...");
    conn.execute_batch("BEGIN IMMEDIATE")?;
    conn.execute_batch(
        "
    -- books
    CREATE TABLE IF NOT EXISTS books(
        id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
        title TEXT NOT NULL,
        author_id INTEGER NOT NULL,
        synopsis TEXT NOT NULL,
        lang TEXT NOT NULL,
        cover TEXT NULL,
        cover_bytes BLOB NULL,
        page_count INTEGER NOT NULL,
        format_type TEXT NOT NULL,
        genres TEXT NULL,
        subjects TEXT NULL,
        published_date INTEGER NULL,
        publisher TEXT NULL,
        open_library_id TEXT NULL,
        isbn TEXT NULL,
        buy TEXT NULL,
        created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
        updated_at TEXT NULL DEFAULT CURRENT_TIMESTAMP
	);
    -- authors
    CREATE TABLE IF NOT EXISTS authors(
		id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL,
        last_name TEXT NOT NULL,
        birth_place TEXT NULL,
        date_of_birth INTEGER NOT NULL,
        date_of_death INTEGER NULL,
        place_of_death TEXT NULL,
        pseudonyms TEXT NULL,
        alt_names TEXT NULL,
        biography TEXT NOT NULL,
        genres TEXT NOT NULL,
        writing_lang TEXT NULL,
        notes TEXT NULL,
        movements_or_trends TEXT NULL,
        influences TEXT NULL,
        picture_bytes BLOB NULL,
        picture_url TEXT NULL,
        open_library_id TEXT NULL,
        inventaire_id TEXT NULL,
        viaf_id TEXT NULL,
        website TEXT NULL,
        created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
        updated_at TEXT NULL DEFAULT CURRENT_TIMESTAMP
	);
    ",
    )?;
    conn.execute_batch("COMMIT")?;

    // crear data inicial de ejemplo
    let res = add_default_author(conn)?;
    println!("add_default_author result {:?}", res);

    Ok(())
}
