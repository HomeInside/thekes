use super::model::AuthorModel;
use rusqlite::Error;
use rusqlite::{Connection, OptionalExtension, Result, params};

pub fn all_authors(conn: &Connection) -> Result<Vec<AuthorModel>, Error> {
    let mut stmt = conn.prepare(
		"SELECT id, name, last_name, birth_place, date_of_birth, date_of_death, genres, biography, writing_lang, cover_color FROM authors",
	)?;

    let authors_list = stmt.query_map([], |row| {
        Ok(AuthorModel {
            id: row.get(0)?,
            name: row.get(1)?,
            last_name: row.get(2)?,
            birth_place: row.get(3)?,
            date_of_birth: row.get(4)?,
            date_of_death: row.get(5)?,
            genres: row.get(6)?,
            biography: row.get(7)?,
            writing_lang: row.get(8)?,
            cover_color: row.get(9)?,
            ..Default::default()
        })
    })?;

    let mut result: Vec<AuthorModel> = Vec::new();

    for author in authors_list {
        result.push(author.unwrap());
    }

    Ok(result)
}

pub fn title_authors(conn: &Connection) -> Result<Vec<AuthorModel>, Error> {
    let mut stmt = conn.prepare("SELECT id, name, last_name FROM authors")?;

    let authors_list = stmt.query_map([], |row| {
        Ok(AuthorModel {
            id: row.get(0)?,
            name: row.get(1)?,
            last_name: row.get(2)?,
            ..Default::default()
        })
    })?;

    let mut result: Vec<AuthorModel> = Vec::new();

    for author in authors_list {
        result.push(author.unwrap());
    }

    Ok(result)
}
#[allow(dead_code)]
pub fn delete_author(conn: &Connection, id: i32) -> Result<usize> {
    println!("delete_author fn: {}", id);
    let mut stmt = conn.prepare("DELETE FROM authors WHERE id = :id")?;
    let last_id = stmt.execute(&[(":id", &id)])?;
    Ok(last_id)
}

pub fn search_author_by_id(conn: &Connection, item_id: i32) -> Result<Option<AuthorModel>> {
    conn.query_row(
        "SELECT id, name, last_name, birth_place, date_of_birth,
                date_of_death, genres, biography, langs, cover_color,open_library_id, viaf_id
         FROM authors
         WHERE id = ?1",
        params![item_id],
        |row| {
            Ok(AuthorModel {
                id: row.get("id")?,
                name: row.get("name")?,
                last_name: row.get("last_name")?,
                birth_place: row.get("birth_place")?,
                date_of_birth: row.get("date_of_birth")?,
                date_of_death: row.get("date_of_death")?,
                genres: row.get("genres")?,
                biography: row.get("biography")?,
                cover_color: row.get("cover_color")?,
                langs: row.get("langs")?,
                open_library_id: row.get("open_library_id")?,
                viaf_id: row.get("viaf_id")?,
                ..Default::default()
            })
        },
    )
    .optional()
}

#[allow(dead_code)]
pub fn search_one_author(conn: &Connection, name: String) -> Result<Vec<AuthorModel>, Error> {
    println!("search_one_author fn: {}", name);
    let mut stmt = conn.prepare("SELECT * FROM authors WHERE name LIKE :name")?;
    let sql = format!("%{}%", name);

    let rows = stmt.query_map(&[(":name", &sql)], |row| {
        Ok(AuthorModel {
            id: row.get(0)?,
            name: row.get(1)?,
            last_name: row.get(2)?,
            ..Default::default()
        })
    })?;
    rows.collect()
}

#[allow(dead_code)]
pub fn add_author(conn: &Connection, author: &AuthorModel) -> Result<usize, Error> {
    println!("add_author fn");
    let mut stmt = conn.prepare("INSERT INTO authors (name, last_name, birth_place, date_of_birth, date_of_death, pseudonyms, writing_lang, genres, biography, movements_or_trends, open_library_id, viaf_id, notes) VALUES(:name, :last_name, :birth_place, :date_of_birth, :date_of_death, :pseudonyms, :writing_lang, :genres, :biography, :movements_or_trends, :open_library_id, :viaf_id, :notes)")?;
    stmt.execute(params![
        author.name,
        author.last_name,
        author.birth_place,
        author.date_of_birth,
        author.date_of_death,
        author.pseudonyms,
        author.writing_lang,
        author.genres,
        author.biography,
        author.movements_or_trends,
        author.open_library_id,
        author.viaf_id,
        author.notes,
    ])?;

    let last_row_id = conn.last_insert_rowid() as usize;
    //
    Ok(last_row_id)
}

#[allow(dead_code)]
pub fn edit_authors(conn: &Connection, item_id: i32, author: &AuthorModel) -> Result<usize, Error> {
    println!("edit_authors fn: id:{}", item_id);
    let mut stmt = conn.prepare("UPDATE authors SET name = :name, last_name = :last_name WHERE id = :id")?;

    let rows = stmt.execute(params![author.name, author.last_name, item_id])?;

    Ok(rows)
}
