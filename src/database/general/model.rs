#[derive(Clone, Debug, Default)]
pub struct StatsModel {
    pub books: i32,
    pub authors: i32,
    pub favs: i32,
    pub publ: i32,
    pub genre: i32,
    pub lang: i32,
    pub quote: String,
    //
    pub rb_title: String,
    pub rb_published: String,
    pub rb_cover_color: Option<u32>,
    pub r_author: String,
}
