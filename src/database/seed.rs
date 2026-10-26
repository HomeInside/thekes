use rusqlite::{Connection, params};
use std::error::Error;

pub fn add_default_author(conn: &Connection) -> Result<(), Box<dyn Error>> {
    println!("add_default_author fn");
    conn.execute(
        "INSERT INTO authors (
            name, last_name, birth_place, date_of_birth, date_of_death,
            pseudonyms, alt_names, biography, genres, langs, writing_lang, cover_color, open_library_id, viaf_id
        )
        SELECT ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14
        WHERE NOT EXISTS (
            SELECT 1 FROM authors
            WHERE name = ?1 AND last_name = ?2
        )",
        params![
            "Arthur",
            "Conan Doyle",
            Some("Scotland"),
            "1859-05-22",
            Some("1930-07-07"),
            Some("A. Conan Doyle"),
            Some("Sir Arthur Conan Doyle"),
            "Sir Arthur Ignatius Conan Doyle KStJ, DL (22 May 1859 – 7 July 1930) was a Scottish writer and physician, most noted for creating the fictional detective Sherlock Holmes and writing stories about him which are generally considered milestones in the field of crime fiction. \
            \nHe is also known for writing the fictional adventures of a second character he invented, Professor Challenger, and for popularising the mystery of the Mary Celeste. He was a prolific writer whose other works include fantasy and science fiction stories, plays, romances, poetry, non-fiction and historical novels.",
            "Novela policiaca, ficción histórica, ciencia ficción",
            Some("English"),
            Some("English"),
            Some(0xF4D5C2),
            Some("OL161167A"),
            Some("65283845"),
        ],
    )?;
    conn.execute(
        "INSERT INTO books (
            title,author_id,synopsis,lang,cover,page_count,format_type,genres,published_date,publisher,open_library_id,buy, cover_color
        )
        SELECT ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13
        WHERE NOT EXISTS (
            SELECT 1 FROM books
            WHERE title = ?1 AND author_id = ?2
        )",
        params![
            "The Adventures of Sherlock Holmes",
            1,
            "The Adventures of Sherlock Holmes is a collection of twelve short stories by Arthur Conan Doyle, first published on 14 October 1892. It contains the earliest short stories featuring the consulting detective Sherlock Holmes, which had been published in twelve monthly issues of The Strand Magazine from July 1891 to June 1892. The stories are collected in the same sequence, which is not supported by any fictional chronology. The only characters common to all twelve are Holmes and Dr. Watson and all are related in first-person narrative from Watson's point of view.",
            "English",
            "https://covers.openlibrary.org/b/id/12918470-L.jpg",
            307,
            "physical_book",
            Some("classic literature, mistery, crime & mystery fiction"),
            1920,
            "A. L. Burt Company, Publishers",
            "OL262421W",
            "https://www.amazon.com/s?k=The+Adventures+of+Sherlock+Holmes+Arthur+Conan+Doyle&i=stripbooks",
            Some(0x000000),
        ],
    )?;

    Ok(())
}
