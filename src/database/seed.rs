use rusqlite::{Connection, params};
use std::error::Error;

pub fn add_default_author(conn: &Connection) -> Result<(), Box<dyn Error>> {
    println!("add_default_author fn");
    conn.execute(
        "INSERT INTO authors (
            name, last_name, birth_place, date_of_birth, date_of_death,
            pseudonyms, alt_names, biography, genres, writing_lang
        )
        SELECT ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10
        WHERE NOT EXISTS (
            SELECT 1 FROM authors
            WHERE name = ?1 AND last_name = ?2
        )",
        params![
            "Arthur",
            "Conan Doyle",
            Some("Scotland"),
            1859,
            Some(1930),
            Some("A. Conan Doyle"),
            Some("Sir Arthur Conan Doyle"),
            "Escritor escocés, conocido por crear a Sherlock Holmes.",
            "Novela policiaca, ficción histórica, ciencia ficción",
            Some("English"),
        ],
    )?;

    Ok(())
}
