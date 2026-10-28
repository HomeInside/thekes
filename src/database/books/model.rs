#[derive(Clone, Debug, Default)]
#[allow(dead_code)]
pub struct BooksModel {
    pub id: i32,
    pub title: String,
    pub author_id: i32,
    pub synopsis: String,
    pub lang: String,
    pub cover: Option<String>,
    pub cover_bytes: Option<Vec<u8>>,
    pub page_count: u32,
    pub format_type: String, //physical_book/epub/audio_book
    pub genres: Option<String>,
    // revisar este campo
    pub subjects: Option<String>,
    pub published_date: String,
    pub publisher: Option<String>,
    pub open_library_id: Option<String>,
    pub isbn: Option<String>,
    //
    pub buy: Option<String>,
    pub(crate) cover_color: Option<u32>,
    pub created_at: String,
    pub updated_at: Option<String>,
}
