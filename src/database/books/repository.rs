use rusqlite::{Connection, Result, params};
use rusqlite::{Error, OptionalExtension};

use super::model::BooksModel;

#[allow(dead_code)]
pub fn all_books(conn: &Connection) -> Result<Vec<BooksModel>, Error> {
    let mut stmt = conn.prepare(
        "SELECT id, title, author_id, published_date, open_library_id, isbn, buy,favorite,current_read FROM books",
    )?;

    let books_list = stmt.query_map([], |row| {
        Ok(BooksModel {
            id: row.get(0)?,
            title: row.get(1)?,
            author_id: row.get(2)?,
            published_date: row.get(3)?,
            open_library_id: row.get(4)?,
            isbn: row.get(5)?,
            buy: row.get(6)?,
            favorite: row.get(7)?,
            current_read: row.get(8)?,
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

pub fn all_books_by_author(conn: &Connection, item_id: i32) -> Result<Vec<BooksModel>, Error> {
    let mut stmt = conn.prepare("SELECT id, title, published_date, genres FROM books where author_id = :author_id")?;

    let books_list = stmt.query_map(params![item_id], |row| {
        Ok(BooksModel {
            id: row.get(0)?,
            title: row.get(1)?,
            published_date: row.get(2)?,
            genres: row.get(3)?,
            ..Default::default()
        })
    })?;

    books_list.collect()
}

#[allow(dead_code)]
pub fn delete_book(conn: &Connection, id: i32) -> Result<usize> {
    println!("delete_book fn: {}", id);
    let mut stmt = conn.prepare("DELETE FROM books WHERE id = :id")?;
    let last_id = stmt.execute(&[(":id", &id)])?;
    Ok(last_id)
}

pub fn search_book_by_id(conn: &Connection, item_id: i32) -> Result<Option<BooksModel>> {
    conn.query_row(
        "SELECT id,title,author_id,synopsis,lang,page_count,format_type,genres,subjects,published_date,publisher,open_library_id,isbn,buy,cover_color,created_at FROM books
         WHERE id = ?1",
        params![item_id],
        |row| {
            Ok(BooksModel {
                id: row.get("id")?,
                title: row.get("title")?,
                author_id: row.get("author_id")?,
                synopsis: row.get("synopsis")?,
                lang: row.get("lang")?,
                //cover_bytes: row.get("cover_bytes")?,
                page_count: row.get("page_count")?,
                format_type: row.get("format_type")?,
                genres: row.get("genres")?,
                subjects: row.get("subjects")?,
                published_date:row.get("published_date")?,
                publisher: row.get("publisher")?,
                open_library_id: row.get("open_library_id")?,
                isbn: row.get("isbn")?,
                buy: row.get("buy")?,
                cover_color: row.get("cover_color")?,
                created_at: row.get("created_at")?,
                //
                ..Default::default()
            })
        },
    ).optional()
}

#[allow(dead_code)]
/// consulta los libros por nombres usando Like
pub fn search_one_book(conn: &Connection, title: &str) -> Result<Vec<BooksModel>, Error> {
    println!("search_one_book fn: {}", title);
    let mut stmt = conn.prepare(
        "SELECT id, title, author_id, published_date, open_library_id, isbn, buy,favorite,current_read FROM books WHERE title LIKE :title",
    )?;
    let sql = format!("%{}%", title.trim().to_lowercase());

    let rows = stmt.query_map(&[(":title", &sql)], |row| {
        Ok(BooksModel {
            id: row.get(0)?,
            title: row.get(1)?,
            author_id: row.get(2)?,
            published_date: row.get(3)?,
            open_library_id: row.get(4)?,
            isbn: row.get(5)?,
            buy: row.get(6)?,
            favorite: row.get(7)?,
            current_read: row.get(8)?,
            ..Default::default()
        })
    })?;

    rows.collect()
}

#[allow(dead_code)]
pub fn add_book(conn: &Connection, book: &BooksModel) -> Result<usize, Error> {
    let mut stmt = conn.prepare(
        "INSERT INTO books(title,author_id,synopsis,lang,page_count,format_type,genres,subjects,published_date,publisher,open_library_id,isbn,buy,cover_color) VALUES(:title,:author_id,:synopsis,:lang,:page_count,:format_type,:genres,:subjects,:published_date,:publisher,:open_library_id,:isbn,:buy,:cover_color)",
    )?;
    stmt.execute(params![
        book.title,
        book.author_id,
        book.synopsis,
        book.lang,
        book.page_count,
        book.format_type,
        book.genres,
        book.subjects,
        book.published_date,
        book.publisher,
        book.open_library_id,
        book.isbn,
        book.buy,
        book.cover_color
    ])?;

    //let last_row_id = conn.last_insert_rowid() as usize;
    let row_id = conn.last_insert_rowid();
    let last_row_id = usize::try_from(row_id).map_err(|_| rusqlite::Error::IntegralValueOutOfRange(0, row_id))?;

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

/// actualiza un libro como favorito (ó no)
pub fn set_favorite_book(conn: &Connection, id: i32) -> Result<bool> {
    // hace un toggle del valor directamente en la tabla
    let is_favorite: bool = conn.query_row(
        "UPDATE books SET favorite = NOT favorite WHERE id = :id RETURNING favorite;",
        [id],
        |row| row.get(0),
    )?;

    Ok(is_favorite)
}

/// Actualiza el estado de lectura de un libro
pub fn set_status_reading(conn: &Connection, id: i32, new_value: i32) -> Result<Option<i32>> {
    let current_status = conn.query_row(
        "UPDATE books SET current_read = ?1 WHERE id = ?2 RETURNING current_read;",
        rusqlite::params![new_value, id],
        |row| row.get(0),
    )?;

    Ok(current_status)
}
