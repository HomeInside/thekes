use rusqlite::Error;
use rusqlite::{Connection, Result, params};

use super::model::AuthorModel;

pub fn all_authors(conn: &Connection) -> Result<Vec<AuthorModel>, Error> {
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

    for movie in authors_list {
        result.push(movie.unwrap());
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
    let mut stmt = conn.prepare("INSERT INTO authors (name, last_name) VALUES(:name, :last_name)")?;
    stmt.execute(params![author.name, author.last_name])?;

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
