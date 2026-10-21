use rusqlite::Error;
use rusqlite::{Connection, Result, params};

use super::model::BooksModel;

#[allow(dead_code)]
pub fn all_books(conn: &Connection) -> Result<Vec<BooksModel>, Error> {
    let mut stmt =
        conn.prepare("SELECT id, title, author_id, published_date, open_library_id, isbn, buy FROM books")?;

    let books_list = stmt.query_map([], |row| {
        Ok(BooksModel {
            id: row.get(0)?,
            title: row.get(1)?,
            author_id: row.get(2)?,
            published_date: row.get(3)?,
            open_library_id: row.get(4)?,
            isbn: row.get(5)?,
            buy: row.get(6)?,
            ..Default::default()
        })
    })?;

    let mut result: Vec<BooksModel> = Vec::new();

    for book in books_list {
        result.push(book.unwrap());
    }

    Ok(result)
    //
    //let result: Result<Vec<_>, _> = books_list.collect();
    // result
}

#[allow(dead_code)]
pub fn delete_book(conn: &Connection, id: i32) -> Result<usize> {
    println!("delete_book fn: {}", id);
    let mut stmt = conn.prepare("DELETE FROM books WHERE id = :id")?;
    let last_id = stmt.execute(&[(":id", &id)])?;
    Ok(last_id)
}

#[allow(dead_code)]
pub fn search_one_book(conn: &Connection, title: String) -> Result<Vec<BooksModel>, Error> {
    println!("search_one_book fn: {}", title);
    let mut stmt = conn.prepare("SELECT * FROM books WHERE title LIKE :title")?;
    let sql = format!("%{}%", title);

    let rows = stmt.query_map(&[(":title", &sql)], |row| {
        Ok(BooksModel {
            id: row.get(0)?,
            title: row.get(1)?,
            author_id: row.get(2)?,
            published_date: row.get(3)?,
            isbn: row.get(4)?,
            ..Default::default()
        })
    })?;
    rows.collect()
}

#[allow(dead_code)]
pub fn add_book(conn: &Connection, book: &BooksModel) -> Result<usize, Error> {
    let mut stmt = conn.prepare(
        "INSERT INTO books (title, author_id, published_date, isbn) VALUES(:title, :author_id, :published_date, :isbn)",
    )?;
    stmt.execute(params![book.title, book.author_id, book.published_date, book.isbn])?;

    let last_row_id = conn.last_insert_rowid() as usize;
    //
    Ok(last_row_id)
}

#[allow(dead_code)]
pub fn edit_book(conn: &Connection, item_id: i32, book: &BooksModel) -> Result<usize, Error> {
    let mut stmt = conn
        .prepare("UPDATE books SET title = :title, author_id = :author_id, published_date = :published_date, isbn = :isbn WHERE id = :id")?;

    let rows = stmt.execute(params![
        book.title,
        book.author_id,
        book.published_date,
        book.isbn,
        item_id
    ])?;

    Ok(rows)
}
